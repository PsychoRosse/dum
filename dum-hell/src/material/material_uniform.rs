use crate::Material;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    color: [f32; 4],
}

impl MaterialUniform {
    #[inline]
    pub fn from_material(material: &Material) -> Self {
        Self {
            color: material.color_value(),
        }
    }
}
