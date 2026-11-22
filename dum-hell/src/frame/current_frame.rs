use crate::Frame;

#[derive(Debug)]
pub enum CurrentFrame {
    Acquired { frame: Frame, suboptimal: bool },
    Validation,
    Timeout,
    Outdated,
    Occluded,
    Lost,
}

impl CurrentFrame {
    pub fn is_suboptimal(&self) -> bool {
        matches!(
            self,
            Self::Acquired {
                suboptimal: true,
                ..
            }
        )
    }

    pub fn take(self) -> Option<Frame> {
        match self {
            Self::Acquired { frame, .. } => Some(frame),
            _ => None,
        }
    }
}
