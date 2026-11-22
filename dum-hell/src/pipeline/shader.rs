#[derive(Debug)]
pub struct Shader {
    inner: wgpu::ShaderModule,
}

impl Shader {
    pub(crate) fn new(inner: wgpu::ShaderModule) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wgpu::ShaderModule {
        &self.inner
    }
}
