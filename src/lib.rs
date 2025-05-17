use glam::{Mat4, Vec3};
use winit::window::Window;
use crate::app::App;
use crate::model::{INDICES, VERTICES};

mod model;
mod app;

pub fn start(window_title: String) {
    let attributes = Window::default_attributes()
        .with_title(window_title);
    let (mut app, event_loop) = App::new(attributes);
    
    let _quad1 = app.renderer.create_object(VERTICES.to_vec(), INDICES.to_vec(), Mat4::IDENTITY);

    let quad2_transform = Mat4::from_translation(Vec3::new(0.0, 30.0, 0.0));
    let _quad2 = app.renderer.create_object(VERTICES.to_vec(), INDICES.to_vec(), quad2_transform);
    
    event_loop.run_app(&mut app).unwrap();
}

pub fn stop() {

}