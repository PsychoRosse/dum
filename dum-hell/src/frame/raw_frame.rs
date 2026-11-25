use crate::{RawDrawPass, RawRenderPipeline, RenderPassDescriptor};

#[derive(Debug)]
pub struct RawFrame<'f> {
    output: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
    encoder: wgpu::CommandEncoder,
    pipeline: &'f RawRenderPipeline,
    suboptimal: bool,
}

impl<'f> RawFrame<'f> {
    pub(crate) fn new(
        output: wgpu::SurfaceTexture,
        view: wgpu::TextureView,
        encoder: wgpu::CommandEncoder,
        pipeline: &'f RawRenderPipeline,
        suboptimal: bool,
    ) -> Self {
        Self {
            output,
            view,
            encoder,
            pipeline,
            suboptimal,
        }
    }

    pub fn is_suboptimal(&self) -> bool {
        self.suboptimal
    }

    pub fn render_pass<F>(&mut self, descriptor: RenderPassDescriptor, callback: F)
    where
        F: FnOnce(&mut RawDrawPass),
    {
        let inner = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(descriptor.clear_color.to_wgpu()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        let mut pass = RawDrawPass::new(inner);

        pass.set_pipeline(&self.pipeline.raw());

        callback(&mut pass);
    }

    pub(crate) fn take_output_encoder(self) -> (wgpu::SurfaceTexture, wgpu::CommandEncoder) {
        let Self {
            output, encoder, ..
        } = self;

        (output, encoder)
    }
}
