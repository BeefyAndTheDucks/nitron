use crate::types::{Object, Vert};
use glam::{Quat, Vec3};
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use crate::app::App;

pub mod app;
pub mod types;

pub enum NitronTask {
    UpdateObject(Object),
    CreateObject {
        vertices: Vec<Vert>,
        indices: Vec<u32>,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    },
    CreateObjectFromFile {
        path: String,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    }
}

pub trait NitronApplication {
    fn update(&mut self, delta_time: f32) -> Vec<NitronTask>;
    fn on_window_event(&mut self, event: &WindowEvent);
}

pub struct Nitron {
    pub app: App,
    application: Option<Box<dyn NitronApplication>>,
    last_frame: Instant,
}

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
                    
                    // Get the tasks from the application update
                    let tasks = application.update(delta_time);
                    
                    // Process all tasks
                    for task in tasks {
                        match task {
                            NitronTask::UpdateObject(object) => {
                                self.app.update_object(object);
                            }
                            NitronTask::CreateObject { vertices, indices, position, rotation, scale } => {
                                self.app.create_object(vertices, indices, position, rotation, scale);
                            }
                            NitronTask::CreateObjectFromFile { path, position, rotation, scale } => {
                                self.app.create_objects_from_file(&path, position, rotation, scale);
                            }
                        }
                    }
                    
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