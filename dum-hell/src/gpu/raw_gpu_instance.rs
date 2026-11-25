#![allow(dead_code)]

#[derive(Debug)]
pub struct RawGpuInstance {
    instance: wgpu::Instance,
}

impl RawGpuInstance {
    pub(crate) fn new(instance: wgpu::Instance) -> Self {
        Self { instance }
    }
}
