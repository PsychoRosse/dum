pub struct MaterialBindGroupLayout {
    inner: wgpu::BindGroupLayout,
}

impl MaterialBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let inner = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        Self { inner }
    }

    pub(crate) fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.inner
    }
}
