#![allow(dead_code)]

#[derive(Debug)]
pub struct RawAdapter {
    adapter: wgpu::Adapter,
}

impl RawAdapter {
    pub(crate) fn new(adapter: wgpu::Adapter) -> Self {
        Self { adapter }
    }
}
