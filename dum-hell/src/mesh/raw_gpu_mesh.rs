use crate::{Mesh, RawGraphicsDevice};

use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct RawGpuMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl RawGpuMesh {
    pub fn new(device: &wgpu::Device, mesh: &Mesh) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(mesh.vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(mesh.indices()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = mesh.indices().len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

impl RawGpuMesh {
    #[inline]
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    #[inline]
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    #[inline]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    #[inline]
    pub fn update(&mut self, device: &RawGraphicsDevice, mesh: &Mesh) {
        let vertices = mesh.vertices();
        let indices = mesh.indices();

        device.write_buffer(&self.vertex_buffer, vertices);
        device.write_buffer(&self.index_buffer, indices);

        self.index_count = indices.len() as u32;
    }
}
