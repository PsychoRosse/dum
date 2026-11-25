#[derive(Debug)]
pub struct RawGraphicsipeline {
    raw: wgpu::RenderPipeline,
}

impl RawGraphicsipeline {
    pub(crate) fn new(raw: wgpu::RenderPipeline) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> &wgpu::RenderPipeline {
        &self.raw
    }
}
