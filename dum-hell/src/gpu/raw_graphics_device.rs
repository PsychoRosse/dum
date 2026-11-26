use crate::{Material, Mesh, RawGpuCamera, RawGpuMaterial, RawGpuMesh, RawGpuModel, RawShader};

#[derive(Debug)]
pub struct RawGraphicsDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RawGraphicsDevice {
    #[inline]
    pub(crate) fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }
}

impl RawGraphicsDevice {
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

    #[inline]
    pub(crate) fn write_buffer<T: bytemuck::Pod + bytemuck::Zeroable>(
        &self,
        buffer: &wgpu::Buffer,
        data: &[T],
    ) {
        self.queue
            .write_buffer(buffer, 0, bytemuck::cast_slice(data));
    }

    #[inline]
    pub fn create_gpu_mesh(&self, mesh: &Mesh) -> RawGpuMesh {
        RawGpuMesh::new(&self.device, mesh)
    }

    #[inline]
    pub fn create_gpu_model(
        &self,
        model: plam::Mat4,
        object_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RawGpuModel {
        RawGpuModel::new(&self.device, object_bind_group_layout, model)
    }

    #[inline]
    pub fn create_gpu_material(
        &self,
        material: &Material,
        material_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RawGpuMaterial {
        RawGpuMaterial::new(&self.device, material_bind_group_layout, material)
    }

    #[inline]
    pub fn create_gpu_camera(
        &self,
        mat4: plam::Mat4,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> RawGpuCamera {
        RawGpuCamera::new(&self.device, camera_bind_group_layout, mat4)
    }

    #[inline]
    pub fn create_shader(&self, label: Option<&str>, source: &str) -> RawShader {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            });

        RawShader::new(shader)
    }
}
