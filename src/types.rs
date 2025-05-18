use glam::{Mat4, Quat, Vec3};

#[derive(Clone)]
pub struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3
}

impl Transformation {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale
        }
    }

    pub fn new_identity() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }

    pub(crate) fn to_matrix(self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    pub(crate) id: usize,
    pub transformation: Transformation
}

impl Object {
    /*
    pub(crate) fn new(id: usize, position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let transformation = Transformation::new(position, rotation, scale);
        Self {
            id,
            transformation
        }
    }
    */

    pub(crate) fn new_from_transformation(id: usize, transformation: Transformation) -> Self {
        Self {
            id,
            transformation
        }
    }
}
