use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Parser;
use crate::parser::Rule;

/// A reference to another document to be imported.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Import {
    Iri(Iri),
    Abbreviated(Id), // QUESTION(@althonos): UnprefixedID ?
}

impl From<Iri> for Import {
    fn from(iri: Iri) -> Self {
        Import::Iri(iri)
    }
}

impl From<Id> for Import {
    fn from(id: Id) -> Self {
        Import::Abbreviated(id)
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Import::*;
        match self {
            Iri(iri) => iri.fmt(f),
            Abbreviated(id) => id.fmt(f),
        }
    }
}

impl FromPair for Import {
    const RULE: Rule = Rule::Import;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iri => Iri::from_pair_unchecked(inner).map(From::from),
            Rule::Id => Id::from_pair_unchecked(inner).map(From::from),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Import);
