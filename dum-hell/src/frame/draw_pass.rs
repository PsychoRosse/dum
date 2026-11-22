use crate::GpuMesh;

pub struct DrawPass<'p> {
    inner: wgpu::RenderPass<'p>,
}

impl<'r> DrawPass<'r> {
    pub(crate) fn new(pass: wgpu::RenderPass<'r>) -> Self {
        Self { inner: pass }
    }

    pub fn draw(&mut self, mesh: &GpuMesh) {
        self.inner
            .set_vertex_buffer(0, mesh.vertex_buffer().slice(..));

        self.inner
            .set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);

        self.inner.draw_indexed(0..mesh.index_count(), 0, 0..1);
    }
}
