use crate::{RawGlobalBindGroupLayout, RawMaterialBindGroupLayout, RawObjectBindGroupLayout};

#[derive(Debug)]
pub struct RawRenderPipelineLayout {
    raw: wgpu::PipelineLayout,
}

impl RawRenderPipelineLayout {
    pub fn new(
        device: &wgpu::Device,
        global: &RawGlobalBindGroupLayout,
        material: &RawMaterialBindGroupLayout,
        object: &RawObjectBindGroupLayout,
    ) -> Self {
        let raw = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[Some(global.raw()), Some(material.raw()), Some(object.raw())],
            immediate_size: 0,
        });

        Self { raw }
    }

    pub(crate) fn get(&self) -> &wgpu::PipelineLayout {
        &self.raw
    }
}
