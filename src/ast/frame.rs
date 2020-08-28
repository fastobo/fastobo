use super::EntityFrame;
use super::HeaderFrame;
use super::InstanceFrame;
use super::TermFrame;
use super::TypedefFrame;

/// Any kind of OBO frame.
///
/// This is used by the `crate::parser::FrameReader`, since they iterate on
/// all the frames of the OBO document. This type does however not appear in
/// the `OboDoc` syntax tree since the `HeaderFrame` and `EntityFrame` are
/// properly separated there.
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Header(Box<HeaderFrame>),
    Term(Box<TermFrame>),
    Typedef(Box<TypedefFrame>),
    Instance(Box<InstanceFrame>),
}

impl Frame {
    /// Attempt to convert the frame into a `HeaderFrame`.
    pub fn into_header_frame(self) -> Option<HeaderFrame> {
        if let Frame::Header(h) = self {
            Some(*h)
        } else {
            None
        }
    }

    /// Attempt to convert the frame into an `EntityFrame`.
    pub fn into_entity_frame(self) -> Option<EntityFrame> {
        match self {
            Frame::Term(f) => Some(EntityFrame::Term(f)),
            Frame::Typedef(f) => Some(EntityFrame::Typedef(f)),
            Frame::Instance(f) => Some(EntityFrame::Instance(f)),
            Frame::Header(_) => None,
        }
    }
}

impl From<HeaderFrame> for Frame {
    fn from(frame: HeaderFrame) -> Self {
        Frame::Header(Box::new(frame))
    }
}

impl From<TermFrame> for Frame {
    fn from(frame: TermFrame) -> Self {
        Frame::Term(Box::new(frame))
    }
}

impl From<TypedefFrame> for Frame {
    fn from(frame: TypedefFrame) -> Self {
        Frame::Typedef(Box::new(frame))
    }
}

impl From<InstanceFrame> for Frame {
    fn from(frame: InstanceFrame) -> Self {
        Frame::Instance(Box::new(frame))
    }
}

impl From<EntityFrame> for Frame {
    fn from(frame: EntityFrame) -> Self {
        match frame {
            EntityFrame::Term(f) => Frame::Term(f),
            EntityFrame::Instance(f) => Frame::Instance(f),
            EntityFrame::Typedef(f) => Frame::Typedef(f),
        }
    }
}
