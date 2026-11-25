#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_mat4(mat: plam::Mat4) -> Self {
        Self {
            view_projection: mat.to_cols_array_2d(),
        }
    }
}
