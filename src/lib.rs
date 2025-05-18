use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowId, Window};

pub trait NitronApplication {
    fn update(&mut self, delta_time: f32);
    fn on_window_event(&mut self, event: &WindowEvent);
}

pub struct Nitron {
    pub app: App,
    application: Option<Box<dyn NitronApplication>>,
    last_frame: Instant,
}

use crate::app::App;

pub mod app;
pub mod types;

impl Nitron {
    pub fn create(window_title: String) -> (Self, EventLoop<()>) {
        let attributes = Window::default_attributes()
            .with_title(window_title);
        let (app, event_loop) = App::new(attributes);

        (
            Nitron {
                app,
                application: None,
                last_frame: Instant::now(),
            },
            event_loop
        )
    }

    pub fn set_application<T: NitronApplication + 'static>(&mut self, application: T) {
        self.application = Some(Box::new(application));
    }

    pub fn run(mut self, event_loop: EventLoop<()>) -> Result<(), winit::error::EventLoopError> {
        event_loop.run_app(&mut self)
    }
}

impl ApplicationHandler for Nitron {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app.resumed(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        self.app.window_event(event_loop, window_id, event.clone());

        if let Some(application) = &mut self.application {
            match event.clone() {
                WindowEvent::RedrawRequested => {
                    let now = Instant::now();
                    let delta_time = now.duration_since(self.last_frame).as_secs_f32();
                    
                    application.update(delta_time);
                    
                    self.last_frame = now;
                }
                _ => {
                    application.on_window_event(&event);
                }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.app.about_to_wait(event_loop);
    }
}