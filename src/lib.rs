use std::io::Cursor;
use crate::app::App;
use crate::types::{Object, Texture, Transformation, Vert};
use egui_winit_vulkano::Gui;
use std::time::Instant;
use image::ImageReader;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Icon, Window, WindowId};

pub mod app;
pub mod types;

pub enum NitronTask {
    UpdateObject(Object),
    CreateObject {
        vertices: Vec<Vert>,
        indices: Vec<u32>,
        transformation: Transformation,
        visible: bool,
        texture: Option<Texture>,
    },
    CreateObjectFromFile {
        path: String,
        transformation: Transformation,
        visible: bool,
        texture: Option<Texture>,
    },

    MoveCamera(Transformation)
}

pub trait NitronApplication {
    fn update(&mut self, delta_time: f32) -> Vec<NitronTask>;
    fn on_window_event(&mut self, event: &WindowEvent);
    fn create_ui(&mut self, gui: &mut Gui);
}

const DEFAULT_ICON_PNG: &[u8] = include_bytes!("../assets/logo.png");

fn load_icon() -> Icon {
    let image = ImageReader::new(Cursor::new(DEFAULT_ICON_PNG))
        .with_guessed_format()
        .expect("Cursor I/O never fails")
        .decode()
        .expect("Failed to decode image");

    let image = image.to_rgba8(); // RGBA8 format
    let (width, height) = image.dimensions();
    let rgba = image.into_raw(); // raw RGBA bytes

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}

pub struct Nitron {
    pub app: App,
    application: Option<Box<dyn NitronApplication>>,
    last_frame: Instant,
}

impl Nitron {
    pub fn create(window_title: String) -> (Self, EventLoop<()>) {
        let attributes = Window::default_attributes()
            .with_title(window_title)
            .with_window_icon(Some(load_icon()));
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
        if let Some(application) = &mut self.application {
            if self.app.window_event(event_loop, window_id, event.clone(), |gui| {
                application.create_ui(gui);
            })
            {
                return;
            }

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
                            NitronTask::CreateObject { vertices, indices, transformation, visible, texture} => {
                                self.app.create_object(vertices, indices, transformation, visible, texture);
                            }
                            NitronTask::CreateObjectFromFile { path, transformation, visible, texture } => {
                                self.app.create_objects_from_file(&path, transformation, visible, texture);
                            }

                            NitronTask::MoveCamera(new_transformation) => {
                                self.app.renderer.move_camera(new_transformation.position, new_transformation.rotation, new_transformation.scale);
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