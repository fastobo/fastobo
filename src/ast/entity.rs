use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::Identified;
use crate::semantics::Orderable;
use crate::syntax::Rule;

/// An entity frame, describing either a term, an instance, or a typedef.
///
/// # Ordering
/// [Serializer conventions](https://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html#S.3.5.2)
/// dictate that frames should be Serialized first with `[Typedef]` frames, then
/// `[Term]`, and then `[Instance]`, which is reflected here in the order of the
/// variants.
#[derive(Clone, Debug, Hash, FromStr, Eq, PartialEq)]
pub enum EntityFrame {
    Typedef(Box<TypedefFrame>),
    Term(Box<TermFrame>),
    Instance(Box<InstanceFrame>),
}

impl EntityFrame {
    /// Return the [`TermFrame`] if the entity frame is one, or `None`.
    ///
    /// Use this function in conjunction with [`Iterator::flat_map`] to extract
    /// all term frames from an iterator of `EntityFrame` references:
    ///
    /// [`Iterator::flat_map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
    /// [`TermFrame`]: ./struct.TermFrame.html
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
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Return the [`TypedefFrame`] if the entity frame is one, or `None`.
    ///
    /// [`TypedefFrame`]: ./struct.TypedefFrame.html
    pub fn as_typedef_frame(&self) -> Option<&TypedefFrame> {
        if let EntityFrame::Typedef(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Return the [`InstanceFrame`] if the entity frame is one, or `None`.
    ///
    /// [`InstanceFrame`]: ./struct.InstanceFrame.html
    pub fn as_instance_frame(&self) -> Option<&InstanceFrame> {
        if let EntityFrame::Instance(frame) = &self {
            Some(frame.as_ref())
        } else {
            None
        }
    }

    /// Get the name of the entity, if exactly one is declared.
    pub fn name(&self) -> Result<&UnquotedString, CardinalityError> {
        match &self {
            EntityFrame::Term(frame) => frame.name(),
            EntityFrame::Typedef(frame) => frame.name(),
            EntityFrame::Instance(frame) => frame.name(),
        }
    }

    /// Get the definition of the entity, if exactly one is declared.
    pub fn definition(&self) -> Result<&Definition, CardinalityError> {
        match &self {
            EntityFrame::Term(frame) => frame.definition(),
            EntityFrame::Typedef(frame) => frame.definition(),
            EntityFrame::Instance(frame) => frame.definition(),
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
        EntityFrame::Term(Box::new(frame))
    }
}

impl From<TypedefFrame> for EntityFrame {
    fn from(frame: TypedefFrame) -> Self {
        EntityFrame::Typedef(Box::new(frame))
    }
}

impl From<InstanceFrame> for EntityFrame {
    fn from(frame: InstanceFrame) -> Self {
        EntityFrame::Instance(Box::new(frame))
    }
}

impl<'i> FromPair<'i> for EntityFrame {
    const RULE: Rule = Rule::EntityFrame;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::TermFrame => TermFrame::from_pair_unchecked(inner, cache).map(From::from),
            Rule::TypedefFrame => TypedefFrame::from_pair_unchecked(inner, cache).map(From::from),
            Rule::InstanceFrame => InstanceFrame::from_pair_unchecked(inner, cache).map(From::from),
            _ => unreachable!(),
        }
    }
}

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
    use std::str::FromStr;

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
