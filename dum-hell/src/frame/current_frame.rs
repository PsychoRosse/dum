use crate::Frame;

#[derive(Debug)]
pub enum CurrentFrame<'f> {
    Acquired { frame: Frame<'f>, suboptimal: bool },
    Validation,
    Timeout,
    Outdated,
    Occluded,
    Lost,
}

impl<'f> CurrentFrame<'f> {
    pub fn is_suboptimal(&self) -> bool {
        matches!(
            self,
            Self::Acquired {
                suboptimal: true,
                ..
            }
        )
    }

    pub fn take(self) -> Option<Frame<'f>> {
        match self {
            Self::Acquired { frame, .. } => Some(frame),
            _ => None,
        }
    }
}
