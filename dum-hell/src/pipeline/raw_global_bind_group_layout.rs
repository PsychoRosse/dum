#[derive(Debug)]
pub struct RawGlobalBindGroupLayout {
    raw: wgpu::BindGroupLayout,
}

impl RawGlobalBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Global Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self { raw }
    }

    pub(crate) fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.raw
    }
}
