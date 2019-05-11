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
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::OboParser;
use crate::parser::Rule;

/// An entity frame, either for a term, an instance, or a typedef.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
}

impl EntityFrame {
    /// Get a reference to the identifier of the entity.
    ///
    /// This is made possible because all entities in OBO documents have an
    /// identifier.
    pub fn id(&self) -> &Ident {
        use self::EntityFrame::*;
        match self {
            Term(x) => x.id().as_ref().as_ref(),
            Typedef(x) => x.id().as_ref().as_ref(),
            Instance(x) => x.id().as_ref().as_ref(),
        }
    }

    /// Get a mutable reference to the identifier of the entity.
    pub fn id_mut(&mut self) -> &mut Ident {
        use self::EntityFrame::*;
        match self {
            Term(x) => x.id_mut().as_mut().as_mut(),
            Typedef(x) => x.id_mut().as_mut().as_mut(),
            Instance(x) => x.id_mut().as_mut().as_mut(),
        }
    }
}

impl Display for EntityFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::EntityFrame::*;
        match self {
            Term(t) => t.fmt(f),
            Typedef(t) => t.fmt(f) ,
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::TermFrame => TermFrame::from_pair_unchecked(inner).map(From::from),
            Rule::TypedefFrame => TypedefFrame::from_pair_unchecked(inner).map(From::from),
            Rule::InstanceFrame => unimplemented!(),
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
