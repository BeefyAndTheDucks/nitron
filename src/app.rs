use crate::types::Object;
use glam::Mat4;
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

    pub fn create_object(&mut self, vertices: Vec<crate::types::Vert>, indices: Vec<u32>, transform: Mat4) -> Object {
        let mut renderer_vertices = Vec::new();
        for vert in vertices.iter() {
            renderer_vertices.push(Vert {
                position: vert.position.to_array(),
                normal: vert.normal.to_array(),
            })
        }
        let id = self.renderer.create_object(renderer_vertices, indices, transform);

        Object::new(id, transform)
    }

    pub fn update_object(&mut self, object: Object) {
        self.renderer.update_object(object.id, object.transform);
    }

    pub fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.resumed(event_loop)
    }

    pub fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.renderer.window_event(event_loop, window_id, event.clone());
    }

    pub fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.about_to_wait(event_loop);
    }
}
