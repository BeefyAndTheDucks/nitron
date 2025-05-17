use glam::Mat4;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};
use renderer::renderer::Renderer;
use renderer::types::Vert;

pub struct App {
    pub renderer: Renderer,
    
    app_event_listeners: Vec<Box<dyn AppEventListener>>,
}

impl App {
    pub fn new(window_attributes: WindowAttributes) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        let app_event_listeners = Vec::new();
        
        let renderer = Renderer::new(&event_loop, window_attributes);
        (
            App {
                renderer,
                app_event_listeners
            },
            event_loop
        )
    }
    
    pub fn add_listener(&mut self, listener: Box<dyn AppEventListener>) {
        self.app_event_listeners.push(listener);
    }
    
    pub fn create_object(&mut self, vertices: Vec<crate::types::Vert>, indices: Vec<u32>, transform: Mat4) {
        let mut renderer_vertices = Vec::new();
        for vert in vertices.iter() {
            renderer_vertices.push(Vert {
                position: vert.position.to_array(),
                normal: vert.normal.to_array(),
            })
        }
        self.renderer.create_object(renderer_vertices, indices, transform);
    }
}

impl ApplicationHandler for App {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.new_events(event_loop, cause.clone());
        }
    }
    
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.resumed(event_loop);
        }
        
        self.renderer.resumed(event_loop)
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.user_event(event_loop, event);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.window_event(event_loop, window_id, event.clone());
        }
        
        self.renderer.window_event(event_loop, window_id, event.clone());
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.device_event(event_loop, device_id, event.clone());
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.about_to_wait(event_loop);
        }
        
        self.renderer.about_to_wait(event_loop);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.suspended(event_loop);
        }
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.exiting(event_loop);
        }
    }
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        for listener in self.app_event_listeners.iter_mut() {
            listener.memory_warning(event_loop);
        }
    }
}

pub trait AppEventListener<T: 'static = ()> {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let _ = (event_loop, cause);
    }
    fn resumed(&mut self, event_loop: &ActiveEventLoop);
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: T) {
        let _ = (event_loop, event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    );

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
}
