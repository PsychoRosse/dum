use crate::Material;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    color: [f32; 4],
}

impl MaterialUniform {
    pub fn from_material(material: &Material) -> Self {
        Self {
            color: material.color().value,
        }
    }
}
