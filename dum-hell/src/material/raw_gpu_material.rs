use crate::{Material, MaterialUniform, RawGraphicsDevice};

use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct RawGpuMaterial {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl RawGpuMaterial {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, material: &Material) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::bytes_of(&MaterialUniform::from_material(material)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl RawGpuMaterial {
    #[inline]
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline]
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    #[inline]
    pub fn update(&self, device: &RawGraphicsDevice, material: &Material) {
        device.write_buffer(
            &self.buffer,
            std::slice::from_ref(&MaterialUniform::from_material(material)),
        );
    }
}
