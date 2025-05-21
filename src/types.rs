use glam::{EulerRot, Mat4, Quat, Vec2, Vec3};

#[derive(Clone)]
pub struct Vert {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coord: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct EulerTransformation {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl EulerTransformation {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn as_transformation(self) -> Transformation {
        Transformation::new(
            self.position,
            Quat::from_euler(
                EulerRot::XYZ,
                self.rotation.x.to_radians(),
                self.rotation.y.to_radians(),
                self.rotation.z.to_radians(),
            ),
            self.scale,
        )
    }

    pub fn from_transformation(transformation: Transformation) -> Self {
        let rot_radians = transformation.rotation.to_euler(EulerRot::XYZ);
        let rot_degrees = Vec3::new(
            rot_radians.0.to_degrees(),
            rot_radians.1.to_degrees(),
            rot_radians.2.to_degrees(),
        );

        Self {
            position: transformation.position,
            rotation: rot_degrees,
            scale: transformation.scale,
        }
    }

    pub fn new_identity() -> Self {
        Self::new(Vec3::ZERO, Vec3::ZERO, Vec3::ONE)
    }
}

impl Into<Transformation> for EulerTransformation {
    fn into(self) -> Transformation {
        self.as_transformation()
    }
}

impl From<Transformation> for EulerTransformation {
    fn from(value: Transformation) -> Self {
        Self::from_transformation(value)
    }
}

impl Transformation {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn new_identity() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn forwards(self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    pub fn right(self) -> Vec3 {
        self.rotation * Vec3::NEG_X
    }

    pub fn up(self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub(crate) fn to_matrix(self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    pub(crate) id: usize,
    pub transformation: Transformation,
    pub visible: bool,
}

impl Object {
    pub(crate) fn new_from_transformation(
        id: usize,
        transformation: Transformation,
        visible: bool,
    ) -> Self {
        Self {
            id,
            transformation,
            visible,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id: usize,
}

impl Texture {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}
