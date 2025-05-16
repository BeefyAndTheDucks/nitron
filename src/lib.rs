use crate::vulkan_init::{setup};
use crate::renderer::{run};

mod vulkan_init;
mod renderer;
mod vulkan_context;
mod shaders;

pub fn start() {
    let event_loop = setup();

    let vertex1 = renderer::Vert { position: [-0.5, -0.5] };
    let vertex2 = renderer::Vert { position: [-0.5,  0.5] };
    let vertex3 = renderer::Vert { position: [ 0.5,  0.5] };
    let vertex4 = renderer::Vert { position: [ 0.5, -0.5] };

    let vertices = vec![vertex1, vertex2, vertex3, vertex4];

    let indices = vec!
    [
        0, 1, 2,
        2, 3, 0
    ];

    run(
        event_loop,
        vertices,
        indices
    );
}

pub fn stop() {

}