#[derive(Debug, Clone, Copy)]
pub struct Model {
    position: plam::Vec3,
    rotation: plam::Quat,
    scale: plam::Vec3,
}

impl Model {
    pub fn new() -> Self {
        Self {
            position: plam::Vec3::ZERO,
            rotation: plam::Quat::IDENTITY,
            scale: plam::Vec3::ONE,
        }
    }

    pub fn set_position(&mut self, position: plam::Vec3) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: plam::Quat) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: plam::Vec3) {
        self.scale = scale;
    }

    pub fn matrix(&self) -> plam::Mat4 {
        plam::Mat4::unchecked_from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }
}
