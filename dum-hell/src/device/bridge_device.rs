#[derive(Debug)]
pub struct BridgeDevice {
    adapter: wgpu::Adapter,
}

impl BridgeDevice {
    pub(crate) fn new(adapter: wgpu::Adapter) -> Self {
        Self { adapter }
    }
}

impl BridgeDevice {
    #[inline]
    pub(crate) const fn get(&self) -> &wgpu::Adapter {
        &self.adapter
    }
}
