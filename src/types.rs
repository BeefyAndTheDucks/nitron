use glam::{Mat4, Quat, Vec3};

#[derive(Clone)]
pub struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy)]
pub struct Object {
    pub(crate) id: usize,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3
}

impl Object {
    pub(crate) fn new(id: usize, position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            id,
            position,
            rotation,
            scale,
        }
    }

    pub(crate) fn generate_transform(position: Vec3, rotation: Quat, scale: Vec3) -> Mat4 {
        Mat4::from_scale_rotation_translation(scale, rotation, position)
    }

    pub(crate) fn get_transform(&self) -> Mat4 {
        Self::generate_transform(self.position, self.rotation, self.scale)
    }
}
