#[derive(Debug)]
pub struct RawMaterialBindGroupLayout {
    raw: wgpu::BindGroupLayout,
}

impl RawMaterialBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        Self { raw }
    }

    pub(crate) fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.raw
    }
}
