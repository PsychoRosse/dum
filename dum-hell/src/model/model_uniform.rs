#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    model: [[f32; 4]; 4],
}

impl ModelUniform {
    #[inline]
    pub fn from_mat4(model: plam::Mat4) -> Self {
        Self {
            model: model.to_cols_array_2d(),
        }
    }
}
