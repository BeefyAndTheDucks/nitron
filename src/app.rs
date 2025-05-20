use egui_winit_vulkano::Gui;
use crate::types::{Object, Transformation};
use glam::Vec3;
use renderer::renderer::Renderer;
use renderer::types::Vert;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};

pub struct App {
    pub renderer: Renderer,
}

impl App {
    pub fn new(window_attributes: WindowAttributes) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();

        let renderer = Renderer::new(&event_loop, window_attributes);
        (
            App {
                renderer,
            },
            event_loop
        )
    }

    pub fn create_objects_from_file(&mut self, filepath: &str, transformation: Transformation, visible: bool) -> Vec<Object> {
        let model = tobj::load_obj(filepath, &tobj::GPU_LOAD_OPTIONS);
        assert!(model.is_ok());

        let (models, _materials) = model.expect("Failed to load OBJ file");

        let mut objects = Vec::new();

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            println!("Name of #{} is \'{}\'", i, m.name);

            let mut vertices = Vec::new();
            for vertex_idx in 0..mesh.positions.len() / 3 {
                vertices.push(crate::types::Vert {
                    position: Vec3::from_array([mesh.positions[vertex_idx * 3], mesh.positions[vertex_idx * 3 + 1], mesh.positions[vertex_idx * 3 + 2]]),
                    normal: Vec3::from_array([mesh.normals[vertex_idx * 3], mesh.normals[vertex_idx * 3 + 1], mesh.normals[vertex_idx * 3 + 2]]),
                });
            }

            let obj = self.create_object(vertices, mesh.clone().indices, transformation, visible);
            objects.push(obj);
        }

        objects
    }

    pub fn create_object(&mut self, vertices: Vec<crate::types::Vert>, indices: Vec<u32>, transformation: Transformation, visible: bool) -> Object {
        let mut renderer_vertices = Vec::new();
        for vert in vertices.iter() {
            renderer_vertices.push(Vert {
                position: vert.position.to_array(),
                normal: vert.normal.to_array(),
            })
        }

        let id = self.renderer.create_object(renderer_vertices, indices, transformation.clone().to_matrix(), visible);

        Object::new_from_transformation(id, transformation, visible)
    }

    pub fn update_object(&mut self, object: Object) {
        self.renderer.update_object(object.id, object.transformation.to_matrix(), object.visible);
    }

    pub fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.resumed(event_loop)
    }

    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent, layout_function: impl FnOnce(&mut Gui)) -> bool {
        self.renderer.window_event(event_loop, window_id, event.clone(), layout_function)
    }

    pub fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.about_to_wait(event_loop);
    }
}
