use crate::{GpuMesh, Mesh, Shader};

#[derive(Debug)]
pub struct GraphicsDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsDevice {
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }

    #[inline]
    pub(crate) fn get(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    pub(crate) fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    #[inline]
    pub(crate) fn present(&self, texture: wgpu::SurfaceTexture, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(texture);
    }

    pub fn create_gpu_mesh(&self, mesh: &Mesh) -> GpuMesh {
        GpuMesh::new(&self.device, mesh)
    }

    pub fn create_shader(&self, label: Option<&str>, source: &str) -> Shader {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });

        Shader::new(shader)
    }
}
