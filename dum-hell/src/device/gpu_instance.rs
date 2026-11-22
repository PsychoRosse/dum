#[derive(Debug)]
pub struct GpuInstance {
    instance: wgpu::Instance,
}

impl GpuInstance {
    pub(crate) fn new(instance: wgpu::Instance) -> Self {
        Self { instance }
    }
}
