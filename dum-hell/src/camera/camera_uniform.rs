#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    #[inline]
    pub fn from_mat4(mat4: plam::Mat4) -> Self {
        Self {
            view_projection: mat4.to_cols_array_2d(),
        }
    }
}
