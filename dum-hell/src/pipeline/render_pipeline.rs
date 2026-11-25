use crate::Vertex;

#[derive(Debug)]
pub struct RenderPipeline {
    pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        layout: &wgpu::PipelineLayout,
    ) -> Self {
        let vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/vertex.wgsl").into()),
        });

        let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/fragment.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &vertex,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::layout())],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self { pipeline }
    }

    pub fn inner(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
