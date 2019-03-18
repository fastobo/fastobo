use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use iri_string::AbsoluteIriStr;
use iri_string::AbsoluteIriString;
use iri_string::RelativeIriString;
use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use super::Id;
use super::QuotedString;
use super::RelationId;
use crate::error::Error;
use crate::error::Result;


/// A database cross-reference definition.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Xref {
    id: Id,
    desc: Option<QuotedString>,
}

impl Display for Xref {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.id.fmt(f)?;
        match &self.desc {
            Some(desc) => f.write_char(' ')).and(desc.fmt(f)),
            None => Ok(()),
        }
    }
}

impl FromPair for Xref {
    const RULE: Rule = Rule::Xref;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let id = Id::from_pair_unchecked(inner.next().unwrap())?;
        let desc = match inner.next() {
            Some(pair) => Some(QuotedString::from_pair_unchecked(pair)?),
            None => None,
        };
        Ok(Xref { id, desc })
    }
}
impl_fromstr!(Xref);


/// A list of containing zero or more `Xref`s.
pub struct XrefList {
    xrefs: Vec<Xref>,
}

impl Display
