#![allow(dead_code)]

use crate::{CameraUniform, RawGraphicsDevice};

use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct RawGpuCamera {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl RawGpuCamera {
    pub(crate) fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        mat4: plam::Mat4,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::bytes_of(&CameraUniform::from_mat4(mat4)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self { buffer, bind_group }
    }
}

impl RawGpuCamera {
    #[inline]
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    #[inline]
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    #[inline]
    pub fn update(&self, device: &RawGraphicsDevice, view_projection: plam::Mat4) {
        device.write_buffer(
            &self.buffer,
            std::slice::from_ref(&CameraUniform::from_mat4(view_projection)),
        );
    }
}
