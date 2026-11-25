#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelUniform {
    model: [[f32; 4]; 4],
}

impl ModelUniform {
    pub fn from_mat4(model: plam::Mat4) -> Self {
        Self {
            model: model.to_cols_array_2d(),
        }
    }
}
