use crate::PerspectiveProjection;

#[derive(Debug, Clone, Copy)]
pub enum Projection {
    Perspective(PerspectiveProjection),
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

impl Projection {
    #[inline]
    pub fn projection_matrix(&self) -> plam::Mat4 {
        match self {
            Projection::Perspective(projection) => projection.projection_matrix(),
        }
    }
}
