use glam::Mat4;
use vulkano::buffer::Subbuffer;
use crate::types::Vert;

pub struct RenderedObject {
    pub transform: Mat4,

    pub(crate) vertex_buffer: Subbuffer<[Vert]>,
    pub(crate) index_buffer: Subbuffer<[u32]>,
}