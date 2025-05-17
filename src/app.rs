use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};
use renderer::renderer::Renderer;

pub struct App {
    pub renderer: Renderer
}

impl App {
    pub fn new(window_attributes: WindowAttributes) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        
        let renderer = Renderer::new(&event_loop, window_attributes);
        (
            App {
            renderer
        }, 
            event_loop
        )
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.resumed(event_loop)
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.renderer.window_event(event_loop, window_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer.about_to_wait(event_loop);
    }
}