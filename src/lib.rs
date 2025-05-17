use glam::{Mat4, Vec3};
use winit::window::Window;
use renderer::rendered_object::RenderedObject;
use crate::app::App;
use crate::model::{INDICES, VERTICES};

mod model;
mod app;

pub fn start(window_title: String) {
    let attributes = Window::default_attributes()
        .with_title(window_title);
    let (mut app, event_loop) = App::new(attributes);
    
    let quad = RenderedObject::new_with_identity(VERTICES.to_vec(), INDICES.to_vec());
    app.renderer.add_object(quad);

    let quad2_transform = Mat4::from_translation(Vec3::new(0.0, 30.0, 0.0));
    let quad2 = RenderedObject::new(quad2_transform, VERTICES.to_vec(), INDICES.to_vec());
    app.renderer.add_object(quad2);


    event_loop.run_app(&mut app).unwrap();
}

pub fn stop() {

}