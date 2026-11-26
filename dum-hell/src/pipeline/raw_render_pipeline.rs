use crate::{
    RawGlobalBindGroupLayout, RawGraphicsipeline, RawMaterialBindGroupLayout,
    RawObjectBindGroupLayout, RawRenderPipelineLayout, Vertex,
};

#[derive(Debug)]
pub struct RawRenderPipeline {
    pipeline: RawGraphicsipeline,
    global_layout: RawGlobalBindGroupLayout,
    material_layout: RawMaterialBindGroupLayout,
    object_layout: RawObjectBindGroupLayout,
}

impl RawRenderPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        global_layout: RawGlobalBindGroupLayout,
        material_layout: RawMaterialBindGroupLayout,
        object_layout: RawObjectBindGroupLayout,
    ) -> Self {
        let layout =
            RawRenderPipelineLayout::new(&device, &global_layout, &material_layout, &object_layout);

        let vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/vertex.wgsl").into()),
        });

        let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/fragment.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout.get()),
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

        let pipeline = RawGraphicsipeline::new(pipeline);

        Self {
            pipeline,
            global_layout,
            material_layout,
            object_layout,
        }
    }
}

impl RawRenderPipeline {
    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.pipeline.raw()
    }

    pub fn global_layout(&self) -> &wgpu::BindGroupLayout {
        &self.global_layout.raw()
    }

    pub fn material_layout(&self) -> &wgpu::BindGroupLayout {
        &self.material_layout.raw()
    }

    pub fn object_layout(&self) -> &wgpu::BindGroupLayout {
        &self.object_layout.raw()
    }
}
