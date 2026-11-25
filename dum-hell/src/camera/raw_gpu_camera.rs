#![allow(dead_code)]

use crate::CameraUniform;

use wgpu::util::DeviceExt;

pub struct RawGpuCamera {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl RawGpuCamera {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, mat4: plam::Mat4) -> Self {
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

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
