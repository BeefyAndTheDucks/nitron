use crate::app::App;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub mod app;
pub mod types;

pub struct Nitron {
    pub app: App,
}

impl Nitron {
    pub fn create(window_title: String) -> (Self, EventLoop<()>) {
        let attributes = Window::default_attributes()
            .with_title(window_title);
        let (app, event_loop) = App::new(attributes);

        (
            Nitron {
                app,
            },
            event_loop
        )
    }

    pub fn run(&mut self, event_loop: EventLoop<()>) {
        event_loop.run_app(&mut self.app).unwrap();
    }

    pub fn stop(&mut self) {
        
    }
}
