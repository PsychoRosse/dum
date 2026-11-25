#[derive(Debug)]
pub struct RawObjectBindGroupLayout {
    raw: wgpu::BindGroupLayout,
}

impl RawObjectBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let raw = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Object Bind Group Layout"),
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
