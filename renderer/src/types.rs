use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex, Clone)]
#[repr(C)]
pub struct Vert {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],

    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],

    #[format(R32G32_SFLOAT)]
    pub tex_coord: [f32; 2],
}
