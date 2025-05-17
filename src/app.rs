use glam::Mat4;
use renderer::renderer::Renderer;
use renderer::types::Vert;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};
use crate::types::Object;

pub struct App {
    pub renderer: Renderer,
    
    update_event_listeners: Vec<fn(f32)>,

    last_frame: Instant
}

impl App {
    pub fn new(window_attributes: WindowAttributes) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let app_event_listeners = Vec::new();
        
        let renderer = Renderer::new(&event_loop, window_attributes);
        (
            App {
                renderer,
                update_event_listeners: app_event_listeners,
                last_frame: Instant::now()
            },
            event_loop
        )
    }
    
    pub fn add_update_listener(&mut self, listener: fn(f32)) {
        self.update_event_listeners.push(listener);
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

    pub fn update_object(&mut self, object: &Object) {
        self.renderer.update_object(object.id, object.transform);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.resumed(event_loop)
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                let now = Instant::now();

                let delta_time = now.duration_since(self.last_frame).as_secs_f32();
                for listener in self.update_event_listeners.iter() {
                    listener(delta_time);
                }

                self.last_frame = now;
            }
            _ => {}
        }

        self.renderer.window_event(event_loop, window_id, event.clone());
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.about_to_wait(event_loop);
    }
}