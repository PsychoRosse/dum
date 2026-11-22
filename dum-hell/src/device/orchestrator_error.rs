#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorError {
    #[error("Failed to create Surface")]
    SurfaceCreate,
    #[error("Failded to request Adapter")]
    AdapterRequest,
    #[error("Failed to request Device")]
    DeviceRequest,
    #[error("Failde to get default config")]
    GetDefaultConfig,
}
