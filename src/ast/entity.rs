use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::OboParser;
use crate::parser::Rule;
use crate::semantics::Identified;
use crate::semantics::Orderable;

/// An entity frame, either for a term, an instance, or a typedef.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
}

impl EntityFrame {
    /// Return the `TermFrame` if the entity frame is one, or `None`.
    ///
    /// Use this function in conjunction with `Iterator::flat_map` to extract
    /// all term frames from an iterator of `EntityFrame` references:
    ///
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let doc = fastobo::from_file("tests/data/ms.obo").unwrap();
    /// let terms: Vec<&TermFrame> = doc
    ///     .entities()
    ///     .iter()
    ///     .flat_map(EntityFrame::as_term_frame)
    ///     .collect();
    /// ```
    pub fn as_term_frame(&self) -> Option<&TermFrame> {
        if let EntityFrame::Term(frame) = &self {
            Some(frame)
        } else {
            None
        }
    }
    /// Return the `TypedefFrame` if the entity frame is one, or `None`.
    pub fn as_typedef_frame(&self) -> Option<&TypedefFrame> {
        if let EntityFrame::Typedef(frame) = &self {
            Some(frame)
        } else {
            None
        }
    }

    /// Return the `InstanceFrame` if the entity frame is one, or `None`.
    pub fn as_instance_frame(&self) -> Option<&InstanceFrame> {
        if let EntityFrame::Instance(frame) = &self {
            Some(frame)
        } else {
            None
        }
    }
}

impl Identified for EntityFrame {
    fn as_id(&self) -> &Ident {
        use self::EntityFrame::*;
        match self {
            Term(x) => x.as_id(),
            Typedef(x) => x.as_id(),
            Instance(x) => x.as_id(),
        }
    }

    fn as_id_mut(&mut self) -> &mut Ident {
        use self::EntityFrame::*;
        match self {
            Term(x) => x.as_id_mut(),
            Typedef(x) => x.as_id_mut(),
            Instance(x) => x.as_id_mut(),
        }
    }
}

impl Display for EntityFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::EntityFrame::*;
        match self {
            Term(t) => t.fmt(f),
            Typedef(t) => t.fmt(f),
            Instance(i) => i.fmt(f),
        }
    }
}

impl From<TermFrame> for EntityFrame {
    fn from(frame: TermFrame) -> Self {
        EntityFrame::Term(frame)
    }
}

impl From<TypedefFrame> for EntityFrame {
    fn from(frame: TypedefFrame) -> Self {
        EntityFrame::Typedef(frame)
    }
}

impl From<InstanceFrame> for EntityFrame {
    fn from(frame: InstanceFrame) -> Self {
        EntityFrame::Instance(frame)
    }
}

impl<'i> FromPair<'i> for EntityFrame {
    const RULE: Rule = Rule::EntityFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::TermFrame => TermFrame::from_pair_unchecked(inner).map(From::from),
            Rule::TypedefFrame => TypedefFrame::from_pair_unchecked(inner).map(From::from),
            Rule::InstanceFrame => InstanceFrame::from_pair_unchecked(inner).map(From::from),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(EntityFrame);

impl Orderable for EntityFrame {
    fn sort(&mut self) {
        use self::EntityFrame::*;
        match self {
            Term(t) => t.sort(),
            Typedef(t) => t.sort(),
            Instance(i) => i.sort(),
        }
    }
    fn is_sorted(&self) -> bool {
        use self::EntityFrame::*;
        match self {
            Term(t) => t.is_sorted(),
            Typedef(t) => t.is_sorted(),
            Instance(i) => i.is_sorted(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_term_frame() {
        let frame = EntityFrame::from_str("[Term]\nid: TEST:001\n").unwrap();
        assert!(frame.as_term_frame().is_some());
        let frame = EntityFrame::from_str("[Typedef]\nid: TEST:002\n").unwrap();
        assert!(frame.as_term_frame().is_none());
        let frame = EntityFrame::from_str("[Instance]\nid: TEST:002\n").unwrap();
        assert!(frame.as_term_frame().is_none());
    }

    #[test]
    fn as_typedef_frame() {
        let frame = EntityFrame::from_str("[Term]\nid: TEST:001\n").unwrap();
        assert!(frame.as_typedef_frame().is_none());
        let frame = EntityFrame::from_str("[Typedef]\nid: TEST:002\n").unwrap();
        assert!(frame.as_typedef_frame().is_some());
        let frame = EntityFrame::from_str("[Instance]\nid: TEST:002\n").unwrap();
        assert!(frame.as_typedef_frame().is_none());
    }

    #[test]
    fn as_instance_frame() {
        let frame = EntityFrame::from_str("[Term]\nid: TEST:001\n").unwrap();
        assert!(frame.as_instance_frame().is_none());
        let frame = EntityFrame::from_str("[Typedef]\nid: TEST:002\n").unwrap();
        assert!(frame.as_instance_frame().is_none());
        let frame = EntityFrame::from_str("[Instance]\nid: TEST:002\n").unwrap();
        assert!(frame.as_instance_frame().is_some());
    }

    #[test]
    fn as_id() {
        let frame = EntityFrame::from_str("[Term]\nid: TEST:001\n").unwrap();
        assert_eq!(frame.as_id(), &Ident::from_str("TEST:001").unwrap());
        let frame = EntityFrame::from_str("[Typedef]\nid: TEST:002\n").unwrap();
        assert_eq!(frame.as_id(), &Ident::from_str("TEST:002").unwrap());
        let frame = EntityFrame::from_str("[Instance]\nid: TEST:003\n").unwrap();
        assert_eq!(frame.as_id(), &Ident::from_str("TEST:003").unwrap());
    }
}
