use crate::{GlobalBindGroupLayout, MaterialBindGroupLayout, ObjectBindGroupLayout};

pub struct RenderPipelineLayout {
    inner: wgpu::PipelineLayout,
}

impl RenderPipelineLayout {
    pub fn new(
        device: &wgpu::Device,
        global: &GlobalBindGroupLayout,
        material: &MaterialBindGroupLayout,
        object: &ObjectBindGroupLayout,
    ) -> Self {
        let inner = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                // Some(global.layout()),
                // Some(material.layout()),
                // Some(object.layout()),
            ],
            immediate_size: 0,
        });

        Self { inner }
    }

    pub(crate) fn get(&self) -> &wgpu::PipelineLayout {
        &self.inner
    }
}
