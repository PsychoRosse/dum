#[derive(Debug, Clone, Copy)]
pub struct PerspectiveProjection {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(90.0, 1.0, 0.03, 1000.0)
    }
}

impl PerspectiveProjection {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
        }
    }
}

impl PerspectiveProjection {
    #[inline]
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    #[inline]
    pub fn projection_matrix(&self) -> plam::Mat4 {
        plam::Mat4::unchecked_perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    #[inline]
    pub const fn fov(&self) -> f32 {
        self.fov
    }

    #[inline]
    pub const fn aspect(&self) -> f32 {
        self.aspect
    }

    #[inline]
    pub const fn near(&self) -> f32 {
        self.near
    }

    #[inline]
    pub const fn far(&self) -> f32 {
        self.far
    }
}
