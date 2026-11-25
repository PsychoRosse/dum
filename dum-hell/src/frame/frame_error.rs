#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    #[error("Surface validation error")]
    Validation,
    #[error("Surface timeout")]
    Timeout,
    #[error("Surface outdated")]
    Outdated,
    #[error("Surface occluded")]
    Occluded,
    #[error("Surface lost")]
    Lost,
}
