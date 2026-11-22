#[derive(Debug)]
pub struct Graphicsipeline {
    inner: wgpu::RenderPipeline,
}

impl Graphicsipeline {
    pub(crate) fn new(inner: wgpu::RenderPipeline) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wgpu::RenderPipeline {
        &self.inner
    }
}
