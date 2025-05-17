use glam::Mat4;
use crate::types::Vert;

pub struct RenderedObject {
    pub transform: Mat4,
    pub vertices: Vec<Vert>,
    pub indices: Vec<u32>,
}

impl RenderedObject {
    pub fn new(transform: Mat4, vertices: Vec<Vert>, indices: Vec<u32>) -> RenderedObject {
        RenderedObject {
            transform,
            vertices,
            indices
        }
    }

    pub fn new_with_identity(vertices: Vec<Vert>, indices: Vec<u32>) -> RenderedObject {
        Self::new(Mat4::IDENTITY, vertices, indices)
    }
}