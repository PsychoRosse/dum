#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct RawShader {
    raw: wgpu::ShaderModule,
}

impl RawShader {
    pub(crate) fn new(raw: wgpu::ShaderModule) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> &wgpu::ShaderModule {
        &self.raw
    }
}
