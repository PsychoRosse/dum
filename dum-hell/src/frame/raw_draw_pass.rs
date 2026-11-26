use crate::{RawGpuCamera, RawGpuMaterial, RawGpuMesh, RawGpuModel};

#[derive(Debug)]
pub struct RawDrawPass<'p> {
    raw: wgpu::RenderPass<'p>,
}

impl<'r> RawDrawPass<'r> {
    pub(crate) fn new(raw: wgpu::RenderPass<'r>) -> Self {
        Self { raw }
    }
}

impl<'r> RawDrawPass<'r> {
    #[inline]
    pub(crate) fn set_pipeline(&mut self, pipeline: &'r wgpu::RenderPipeline) {
        self.raw.set_pipeline(pipeline);
    }

    #[inline]
    pub fn set_camera(&mut self, camera: &RawGpuCamera) {
        self.raw.set_bind_group(0, camera.bind_group(), &[]);
    }

    #[inline]
    pub fn draw(&mut self, mesh: &RawGpuMesh, model: &RawGpuModel, material: &RawGpuMaterial) {
        self.raw.set_bind_group(1, material.bind_group(), &[]);
        self.raw.set_bind_group(2, model.bind_group(), &[]);

        self.raw
            .set_vertex_buffer(0, mesh.vertex_buffer().slice(..));

        self.raw
            .set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);

        self.raw.draw_indexed(0..mesh.index_count(), 0, 0..1);
    }
}
