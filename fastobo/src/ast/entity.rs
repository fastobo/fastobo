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
