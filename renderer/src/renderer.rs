use crate::rendered_object::RenderedObject;
use crate::shaders::{frag, vert};
use crate::types::Vert;
use egui_winit_vulkano::{Gui, GuiConfig};
use glam::{Mat4, Vec3};
use std::sync::Arc;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{AllocateBufferError, Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferInheritanceInfo, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage, SampleCount};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::depth_stencil::{DepthState, DepthStencilState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::EntryPoint;
use vulkano::swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError, VulkanLibrary};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct Renderer {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    uniform_buffer_allocator: SubbufferAllocator,
    window_attribs: WindowAttributes,
    objects: Vec<RenderedObject>,
    rcx: Option<RenderContext>,
    gui: Option<Gui>,
}

struct RenderContext {
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    vs: EntryPoint,
    fs: EntryPoint,
    pipeline: Arc<GraphicsPipeline>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
}

pub fn create_buffer<T, I>(
    allocator: Arc<dyn MemoryAllocator>,
    usage: BufferUsage,
    iter: I,
) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
where
    T: BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    Buffer::from_iter(
        allocator.clone(),
        BufferCreateInfo {
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        iter,
    )
}

impl Renderer {
    pub fn create_object(&mut self, vertices: Vec<Vert>, indices: Vec<u32>, transform: Mat4) -> usize {
        let vertex_buffer = create_buffer(self.memory_allocator.clone(), BufferUsage::VERTEX_BUFFER, vertices).expect("Failed to create vertex buffer");
        let index_buffer = create_buffer(self.memory_allocator.clone(), BufferUsage::INDEX_BUFFER, indices).expect("Failed to create index buffer");
        
        let obj = RenderedObject {
            transform,
            vertex_buffer,
            index_buffer
        };

        self.objects.push(obj);
        self.objects.len() - 1
    }

    pub fn update_object(&mut self, id: usize, transform: Mat4) {
        if let Some(object) = self.objects.get_mut(id) {
            object.transform = transform;
        }
    }

    pub fn new(event_loop: &EventLoop<()>, window_attribs: WindowAttributes) -> Self {
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = Surface::required_extensions(event_loop).unwrap();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
            .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.presentation_support(i as u32, event_loop).unwrap()
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: Vec::from(&[QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }]),
                ..Default::default()
            },
        )
            .unwrap();

        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo {
                secondary_buffer_count: 32,
                ..Default::default()
            },
        ));

        let uniform_buffer_allocator = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        Renderer {
            instance,
            device,
            queue,
            memory_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
            uniform_buffer_allocator,
            window_attribs,
            objects: Vec::new(),
            rcx: None,
            gui: None
        }
    }

    pub fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.window_attribs.clone())
                .unwrap(),
        );
        
        let surface = Surface::from_window(self.instance.clone(), window.clone()).unwrap();
        let window_size = window.inner_size();

        let (swapchain, images) = {
            let surface_capabilities = self
                .device.clone()
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();
            let (image_format, _) = self
                .device.clone()
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0];

            Swapchain::new(
                self.device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: window_size.into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
                .unwrap()
        };

        let render_pass = vulkano::ordered_passes_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: SampleCount::Sample1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth_stencil: {
                    format: Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                },
            },
            passes: [
                { color: [color], depth_stencil: {depth_stencil}, input: [] }, // Draw what you want on this pass
                { color: [color], depth_stencil: {}, input: [] } // Gui render pass
            ]
            /*pass: {
                color: [color],
                depth_stencil: {depth_stencil},
            },*/
        )
            .unwrap();

        self.gui = Some(Gui::new_with_subpass(
            event_loop,
            surface.clone(),
            self.queue.clone(),
            Subpass::from(render_pass.clone(), 1).unwrap(),
            Format::R8G8B8A8_UNORM,
            GuiConfig::default(),
        ));

        let vs = vert::load(self.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();
        let fs = frag::load(self.device.clone())
            .unwrap()
            .entry_point("main")
            .unwrap();

        let (framebuffers, pipeline) = regen_framebuffer(
            window_size,
            images,
            render_pass.clone(),
            self.memory_allocator.clone(),
            &vs,
            &fs,
        );

        let previous_frame_end = Some(sync::now(self.device.clone()).boxed());

        self.rcx = Some(RenderContext {
            window,
            swapchain,
            render_pass,
            framebuffers,
            vs,
            fs,
            pipeline,
            recreate_swapchain: false,
            previous_frame_end,
        });
    }

    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent, layout_function: impl FnOnce(&mut Gui)) {
        if let Some(gui) = &mut self.gui {
            if event == WindowEvent::RedrawRequested {
                gui.immediate_ui(layout_function);
            }

            let consumed = gui.update(&event);

            if consumed {
                if let Some(rcx) = &self.rcx {
                    rcx.window.request_redraw();
                }
                return;
            }
        }
        
        let rcx = self.rcx.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_) => {
                rcx.recreate_swapchain = true;
            }
            WindowEvent::RedrawRequested => {
                let gui = self.gui.as_mut().unwrap();

                gui.begin_frame();

                let ctx = gui.context();

                egui_winit_vulkano::egui::Window::new("Test")
                    .show(&ctx, |ui| {
                        ui.label("Hello world!");
                    });

                let window_size = rcx.window.inner_size();

                if window_size.width == 0 || window_size.height == 0 {
                    return;
                }

                rcx.previous_frame_end.as_mut().unwrap().cleanup_finished();

                if rcx.recreate_swapchain {
                    let (new_swapchain, new_images) = rcx
                        .swapchain
                        .recreate(SwapchainCreateInfo {
                            image_extent: window_size.into(),
                            ..rcx.swapchain.create_info()
                        })
                        .expect("failed to recreate swapchain");

                    rcx.swapchain = new_swapchain;
                    (rcx.framebuffers, rcx.pipeline) = regen_framebuffer(
                        window_size,
                        new_images,
                        rcx.render_pass.clone(),
                        self.memory_allocator.clone(),
                        &rcx.vs,
                        &rcx.fs,
                    );
                    rcx.recreate_swapchain = false;
                }

                let uniform_buffers = {
                    let aspect_ratio = rcx.swapchain.image_extent()[0] as f32
                        / rcx.swapchain.image_extent()[1] as f32;

                    let proj = Mat4::perspective_rh_gl(
                        std::f32::consts::FRAC_PI_2,
                        aspect_ratio,
                        0.01,
                        100.0,
                    );
                    let view = Mat4::look_to_rh(
                        Vec3::new(0.0, 0.0, 10.0),
                        Vec3::new(0.0, 0.0, -1.0),
                        Vec3::new(0.0, -1.0, 0.0),
                    );
                    let scale = Mat4::from_scale(Vec3::splat(1.0));

                    let mut buffers = Vec::new();

                    for obj in &self.objects {
                        let uniform_data = vert::Data {
                            world: obj.transform.to_cols_array_2d(),
                            view: (view * scale).to_cols_array_2d(),
                            proj: proj.to_cols_array_2d(),
                        };

                        let buffer = self.uniform_buffer_allocator.allocate_sized().unwrap();
                        *buffer.write().unwrap() = uniform_data;

                        buffers.push(buffer);
                    }

                    buffers
                };

                let layout = &rcx.pipeline.layout().set_layouts()[0];
                let mut descriptor_sets = Vec::new();

                let mut idx = 0;

                for _obj in &self.objects {
                    descriptor_sets.push(DescriptorSet::new(
                        self.descriptor_set_allocator.clone(),
                        layout.clone(),
                        [WriteDescriptorSet::buffer(0, uniform_buffers[idx].clone())],
                        [],
                    )
                        .unwrap());

                    idx += 1;
                }

                let (image_index, suboptimal, acquire_future) = match acquire_next_image(
                    rcx.swapchain.clone(),
                    None,
                )
                    .map_err(Validated::unwrap)
                {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        rcx.recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };

                if suboptimal {
                    rcx.recreate_swapchain = true;
                }

                let mut builder = AutoCommandBufferBuilder::primary(
                    self.command_buffer_allocator.clone(),
                    self.queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                    .unwrap();

                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![
                                Some([0.0, 0.0, 1.0, 1.0].into()),
                                Some(1f32.into()),
                            ],
                            ..RenderPassBeginInfo::framebuffer(
                                rcx.framebuffers[image_index as usize].clone(),
                            )
                        },
                        SubpassBeginInfo {
                            contents: SubpassContents::SecondaryCommandBuffers,
                            ..Default::default()
                        },
                    )
                    .unwrap();

                let subpass = Subpass::from(rcx.render_pass.clone(), 0).unwrap();

                let mut objects_cmd_buffer_builder = AutoCommandBufferBuilder::secondary(
                    self.command_buffer_allocator.clone(),
                    self.queue.queue_family_index(),
                    CommandBufferUsage::MultipleSubmit,
                    CommandBufferInheritanceInfo {
                        render_pass: Some(subpass.clone().into()),
                        ..Default::default()
                    },
                )
                    .unwrap();

                objects_cmd_buffer_builder
                    .bind_pipeline_graphics(rcx.pipeline.clone())
                    .unwrap();

                idx = 0;
                for obj in &self.objects {
                    objects_cmd_buffer_builder
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            rcx.pipeline.layout().clone(),
                            0,
                            descriptor_sets[idx].clone(),
                        )
                        .unwrap()
                        .bind_vertex_buffers(
                            0,
                            obj.vertex_buffer.clone(),
                        )
                        .unwrap()
                        .bind_index_buffer(obj.index_buffer.clone())
                        .unwrap()
                    ;
                    unsafe { objects_cmd_buffer_builder.draw_indexed(obj.index_buffer.len() as u32, 1, 0, 0, 0) }
                        .unwrap();

                    idx += 1;
                }

                let cb = objects_cmd_buffer_builder.build().unwrap();
                builder.execute_commands(cb).unwrap();

                builder
                    .next_subpass(Default::default(), SubpassBeginInfo {
                        contents: SubpassContents::SecondaryCommandBuffers,
                        ..Default::default()
                    })
                    .unwrap();

                let dimensions = [
                    rcx.swapchain.image_extent()[0],
                    rcx.swapchain.image_extent()[1]
                ];
                let gui_commands = self.gui.as_mut().unwrap().draw_on_subpass_image(dimensions);
                builder.execute_commands(gui_commands.clone()).unwrap();

                builder.end_render_pass(Default::default()).unwrap();

                let command_buffer = builder.build().unwrap();
                let future = rcx
                    .previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(self.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        self.queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(rcx.swapchain.clone(), image_index),
                    )
                    .then_signal_fence_and_flush();

                match future.map_err(Validated::unwrap) {
                    Ok(future) => {
                        rcx.previous_frame_end = Some(future.boxed());
                    }
                    Err(VulkanError::OutOfDate) => {
                        rcx.recreate_swapchain = true;
                        rcx.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("failed to flush future: {e}");
                        rcx.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                }

                
            }
            _ => {}
        }
    }

    pub fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let rcx = self.rcx.as_mut().unwrap();
        rcx.window.request_redraw();
    }
}

// This function is called once during initialization, then again whenever the window is resized.
fn regen_framebuffer(
    window_size: PhysicalSize<u32>,
    images: Vec<Arc<Image>>,
    render_pass: Arc<RenderPass>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    vs: &EntryPoint,
    fs: &EntryPoint,
) -> (Vec<Arc<Framebuffer>>, Arc<GraphicsPipeline>)
{
    let device = memory_allocator.device();

    let depth_buffer = ImageView::new_default(
        Image::new(
            memory_allocator.clone(),
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
            .unwrap(),
    )
        .unwrap();

    let framebuffers = images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();

            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_buffer.clone()],
                    ..Default::default()
                },
            )
                .unwrap()
        })
        .collect::<Vec<_>>();

    let pipeline = {
        let vertex_input_state = Vert::per_vertex()
            .definition(vs)
            .unwrap();
        let stages = [
            PipelineShaderStageCreateInfo::new(vs.clone()),
            PipelineShaderStageCreateInfo::new(fs.clone()),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
            .unwrap();
        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [Viewport {
                        offset: [0.0, 0.0],
                        extent: window_size.into(),
                        depth_range: 0.0..=1.0,
                    }]
                        .into_iter()
                        .collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
            .unwrap()
    };

    (framebuffers, pipeline)
}
