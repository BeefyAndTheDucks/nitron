use glam::{Mat4, Vec3};
use winit::event_loop::EventLoop;
use winit::window::Window;
use renderer::rendered_object::RenderedObject;
use renderer::renderer::Renderer;
use crate::model::{INDICES, VERTICES};

mod model;

pub fn start(window_title: String) {
    let event_loop = EventLoop::new().unwrap();
    let attributes = Window::default_attributes()
        .with_title(window_title);
    let mut renderer = Renderer::new(&event_loop, attributes);
    
    let quad = RenderedObject::new_with_identity(VERTICES.to_vec(), INDICES.to_vec());
    renderer.add_object(quad);
    
    let quad2_transform = Mat4::from_translation(Vec3::new(0.0, 30.0, 0.0));
    let quad2 = RenderedObject::new(quad2_transform, VERTICES.to_vec(), INDICES.to_vec());
    renderer.add_object(quad2);
    

    event_loop.run_app(&mut renderer).unwrap();
}

pub fn stop() {

}