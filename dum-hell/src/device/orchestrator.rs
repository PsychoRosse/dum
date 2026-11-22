use crate::{
    BridgeDevice, Frame, FrameError, GpuInstance, GpuMesh, GraphicsDevice, Mesh, OrchestratorError,
    Shader, WindowSurface,
};

#[derive(Debug)]
pub struct Orchestrator {
    surface: WindowSurface,
    instance: GpuInstance,
    adapter: BridgeDevice,
    device: GraphicsDevice,
}

impl Orchestrator {
    pub fn block_on_new(
        raw_window_handle: raw_window_handle::RawWindowHandle,
        raw_display_handle: raw_window_handle::RawDisplayHandle,
    ) -> Result<Orchestrator, OrchestratorError> {
        pollster::block_on(Self::new(raw_window_handle, raw_display_handle))
    }

    pub async fn new(
        raw_window_handle: raw_window_handle::RawWindowHandle,
        raw_display_handle: raw_window_handle::RawDisplayHandle,
    ) -> Result<Self, OrchestratorError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::METAL,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });

        let surface = unsafe {
            instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(raw_display_handle),
                    raw_window_handle,
                })
                .map_err(|_| OrchestratorError::SurfaceCreate)?
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                apply_limit_buckets: false,
            })
            .await
            .map_err(|_| OrchestratorError::AdapterRequest)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Graphics Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|_| OrchestratorError::DeviceRequest)?;

        let config = surface
            .get_default_config(
                &adapter,
                plam::WindowSize::hd().width(),
                plam::WindowSize::hd().height(),
            )
            .ok_or(OrchestratorError::GetDefaultConfig)?;

        surface.configure(&device, &config);

        let instance = GpuInstance::new(instance);
        let adapter = BridgeDevice::new(adapter);
        let device = GraphicsDevice::new(device, queue);
        let surface = WindowSurface::new(surface, config);

        Ok(Self {
            surface,
            instance,
            adapter,
            device,
        })
    }
}

impl Orchestrator {
    pub fn begin_frame(&self) -> Result<Frame, FrameError> {
        let (output, suboptimal) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => (frame, false),

            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),

            wgpu::CurrentSurfaceTexture::Timeout => {
                return Err(FrameError::Timeout);
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                return Err(FrameError::Outdated);
            }
            wgpu::CurrentSurfaceTexture::Occluded => {
                return Err(FrameError::Occluded);
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err(FrameError::Lost);
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(FrameError::Validation);
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self.device.create_encoder();

        Ok(Frame::new(output, view, encoder, suboptimal))
    }

    pub fn end_frame(&self, frame: Frame) {
        let suboptimal = frame.is_suboptimal();

        let (texture, encoder) = frame.take_all();

        self.device.present(texture, encoder);

        if suboptimal {
            self.reconfigure();
        }
    }

    pub fn resize(&mut self, size: plam::SurfaceSize) {
        self.surface.set_size(size);
        self.surface.reconfigure(self.device.get());
    }

    pub fn reconfigure(&self) {
        self.surface.reconfigure(self.device.get());
    }

    pub fn create_shader(&self, label: Option<&str>, path: &str) -> Shader {
        self.device.create_shader(label, path)
    }

    pub fn create_gpu_mesh(&self, mesh: &Mesh) -> GpuMesh {
        self.device.create_gpu_mesh(mesh)
    }
}
