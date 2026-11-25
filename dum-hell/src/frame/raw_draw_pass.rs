use crate::{RawGpuMaterial, RawGpuMesh, RawGpuModel};

#[derive(Debug)]
pub struct RawDrawPass<'p> {
    inner: wgpu::RenderPass<'p>,
}

impl<'r> RawDrawPass<'r> {
    pub(crate) fn new(pass: wgpu::RenderPass<'r>) -> Self {
        Self { inner: pass }
    }

    pub fn set_pipeline(&mut self, pipeline: &'r wgpu::RenderPipeline) {
        self.inner.set_pipeline(pipeline);
    }

    pub fn draw(&mut self, mesh: &RawGpuMesh, model: &RawGpuModel, material: &RawGpuMaterial) {
        self.inner.set_bind_group(1, material.bind_group(), &[]);
        self.inner.set_bind_group(2, model.bind_group(), &[]);

        self.inner
            .set_vertex_buffer(0, mesh.vertex_buffer().slice(..));

        self.inner
            .set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);

        self.inner.draw_indexed(0..mesh.index_count(), 0, 0..1);
    }
}
