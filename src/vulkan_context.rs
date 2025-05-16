use once_cell::sync::OnceCell;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::device::{Device, Queue};
use vulkano::image::Image;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::swapchain::Swapchain;
use winit::window::Window;

#[derive(Debug)]
pub struct VulkanContext {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) memory_allocator: Arc<StandardMemoryAllocator>,
    pub(crate) command_buffer_allocator: StandardCommandBufferAllocator,
    pub(crate) render_pass: Arc<RenderPass>,
    pub(crate) framebuffers: Vec<Arc<Framebuffer>>,
    pub(crate) window: Arc<Window>,
    pub(crate) swapchain: Arc<Swapchain>,
    pub(crate) images: Vec<Arc<Image>>,
}

pub static VULKAN_CONTEXT: OnceCell<RwLock<VulkanContext>> = OnceCell::new();