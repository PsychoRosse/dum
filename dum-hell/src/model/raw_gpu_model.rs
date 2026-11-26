#![allow(dead_code)]

use crate::{ModelUniform, RawGraphicsDevice};

use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct RawGpuModel {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl RawGpuModel {
    pub(crate) fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        model: plam::Mat4,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: bytemuck::bytes_of(&ModelUniform::from_mat4(model)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Model Bind Group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl RawGpuModel {
    #[inline]
    pub(crate) fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline]
    pub(crate) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    #[inline]
    pub fn update(&mut self, device: &RawGraphicsDevice, model: plam::Mat4) {
        device.write_buffer(
            &self.buffer,
            std::slice::from_ref(&ModelUniform::from_mat4(model)),
        );
    }
}
