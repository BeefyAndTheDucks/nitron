use glam::{Mat4, Vec3};

#[derive(Clone)]
pub struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy)]
pub struct Object {
    pub(crate) id: usize,
    pub transform: Mat4,
}

impl Object {
    pub(crate) fn new(id: usize, transform: Mat4) -> Self {
        Self {
            id,
            transform
        }
    }
}
