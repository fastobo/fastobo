use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use super::EntityFrame;
use super::HeaderFrame;
use super::InstanceFrame;
use super::TermFrame;
use super::TypedefFrame;

use crate::semantics::Orderable;

/// Any kind of OBO frame.
///
/// This is used by the `crate::parser::FrameReader`, since they iterate on
/// all the frames of the OBO document. This type does however not appear in
/// the `OboDoc` syntax tree since the `HeaderFrame` and `EntityFrame` are
/// properly separated there.
///
/// # Ordering
/// [Serializer conventions](https://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html#S.3.5.2)
/// dictate that frames should be Serialized first with `[Typedef]` frames, then
/// `[Term]`, and then `[Instance]`, which is reflected here in the order of the
/// variants.
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Header(Box<HeaderFrame>),
    Typedef(Box<TypedefFrame>),
    Term(Box<TermFrame>),
    Instance(Box<InstanceFrame>),
}

impl Frame {
    /// Return the [`HeaderFrame`] if the frame is one, or `None`.
    ///
    /// [`HeaderFrame`]: ./struct.HeaderFrame.html
    pub fn as_header_frame(&self) -> Option<&HeaderFrame> {
        if let Frame::Header(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Return the [`TermFrame`] if the frame is one, or `None`.
    ///
    /// [`TermFrame`]: ./struct.TermFrame.html
    pub fn as_term_frame(&self) -> Option<&TermFrame> {
        if let Frame::Term(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Return the [`TypedefFrame`] if the frame is one, or `None`.
    ///
    /// [`TypedefFrame`]: ./struct.TypedefFrame.html
    pub fn as_typedef_frame(&self) -> Option<&TypedefFrame> {
        if let Frame::Typedef(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Return the [`InstanceFrame`] if the frame is one, or `None`.
    ///
    /// [`InstanceFrame`]: ./struct.InstanceFrame.html
    pub fn as_instance_frame(&self) -> Option<&InstanceFrame> {
        if let Frame::Instance(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

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

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Frame::*;
        match self {
            Header(h) => h.fmt(f),
            Term(t) => t.fmt(f),
            Typedef(t) => t.fmt(f),
            Instance(i) => i.fmt(f),
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

impl Orderable for Frame {
    fn sort(&mut self) {
        use self::Frame::*;
        match self {
            Header(h) => h.sort(),
            Term(t) => t.sort(),
            Typedef(t) => t.sort(),
            Instance(i) => i.sort(),
        }
    }
    fn is_sorted(&self) -> bool {
        use self::Frame::*;
        match self {
            Header(h) => h.is_sorted(),
            Term(t) => t.is_sorted(),
            Typedef(t) => t.is_sorted(),
            Instance(i) => i.is_sorted(),
        }
    }
}
