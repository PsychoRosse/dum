use crate::Projection;

#[derive(Default, Debug, Clone, Copy)]
pub struct Camera {
    projection: Projection,
}

impl Camera {
    pub fn view_projection_matrix(&self, view_matrix: plam::Mat4) -> plam::Mat4 {
        self.projection.projection_matrix() * view_matrix
    }

    pub fn set_aspect_from_surface(&mut self, size: plam::Extent) {
        let aspect = size.aspect_ratio();

        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect),
        }
    }

    pub fn projection_matrix(&self) -> plam::Mat4 {
        self.projection.projection_matrix()
    }

    pub const fn projection(&self) -> Projection {
        self.projection
    }
}

impl From<Projection> for Camera {
    fn from(value: Projection) -> Self {
        Self { projection: value }
    }
}
