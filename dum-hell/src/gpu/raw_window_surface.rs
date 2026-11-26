#![allow(dead_code)]

#[derive(Debug)]
pub struct RawWindowSurface {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

impl RawWindowSurface {
    pub(crate) fn new(surface: wgpu::Surface<'static>, config: wgpu::SurfaceConfiguration) -> Self {
        Self { surface, config }
    }
}

impl RawWindowSurface {
    #[inline]
    pub(crate) fn size(&self) -> plam::Extent {
        plam::Extent::clamped(self.config.width, self.config.height)
    }

    #[inline]
    pub(crate) fn set_size(&mut self, size: plam::Extent) {
        self.config.width = size.width();
        self.config.height = size.height();
    }

    #[inline]
    pub(crate) fn resize(&mut self, size: plam::Extent, device: &wgpu::Device) {
        self.config.width = size.width();
        self.config.height = size.height();

        self.surface.configure(device, &self.config);
    }

    #[inline]
    pub(crate) fn reconfigure(&self, device: &wgpu::Device) {
        self.surface.configure(device, &self.config);
    }

    #[inline]
    pub(crate) fn get_current_texture(&self) -> wgpu::CurrentSurfaceTexture {
        self.surface.get_current_texture()
    }
}
