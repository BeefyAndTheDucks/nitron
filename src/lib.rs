use winit::event_loop::EventLoop;
use crate::renderer::App;

mod renderer;
mod types;
mod model;
mod shaders;

pub fn start() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app).unwrap();
}

pub fn stop() {

}