use crate::Space;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    position: plam::Vec3,
    rotation: plam::Quat,
    scale: plam::Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: plam::Vec3::ZERO,
            rotation: plam::Quat::IDENTITY,
            scale: plam::Vec3::ONE,
        }
    }
}

impl Transform {
    #[must_use]
    pub const fn new(position: plam::Vec3, rotation: plam::Quat, scale: plam::Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl Transform {
    #[inline]
    #[must_use]
    pub fn matrix(&self) -> plam::Mat4 {
        plam::Mat4::unchecked_from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        )
    }

    #[inline]
    pub fn set_position(&mut self, position: plam::Vec3) {
        self.position = position;
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: plam::Quat) {
        self.rotation = rotation;
    }

    #[inline]
    pub fn set_scale(&mut self, scale: plam::Vec3) {
        self.scale = scale;
    }

    #[inline]
    pub const fn set_position_and_rotation(&mut self, position: plam::Vec3, rotation: plam::Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    #[inline]
    #[must_use]
    pub fn forward(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::FORWARD
    }

    #[inline]
    #[must_use]
    pub fn backward(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::BACKWARD
    }

    #[inline]
    #[must_use]
    pub fn right(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::RIGHT
    }

    #[inline]
    #[must_use]
    pub fn left(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::LEFT
    }

    #[inline]
    #[must_use]
    pub fn up(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::UP
    }

    #[inline]
    #[must_use]
    pub fn down(&self) -> plam::Vec3 {
        self.rotation * plam::Vec3::DOWN
    }

    #[inline]
    pub fn translate(&mut self, translation: plam::Vec3, space: Space) {
        match space {
            Space::Global => self.position += translation,
            Space::Local => self.position += self.rotation * translation,
        }
    }

    #[inline]
    pub fn rotate(&mut self, axis: plam::Vec3, angle: f32, space: Space) {
        match space {
            Space::Global => {
                self.rotation =
                    plam::Quat::unchecked_from_axis_angle(axis.unchecked_normalize(), angle)
                        * self.rotation
            }
            Space::Local => {
                self.rotation *=
                    plam::Quat::unchecked_from_axis_angle(axis.unchecked_normalize(), angle)
            }
        }
    }
}
