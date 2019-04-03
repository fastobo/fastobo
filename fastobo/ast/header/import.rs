use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A reference to another document to be imported.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Import {
    Url(Url),
    Abbreviated(Identifier), // QUESTION(@althonos): UnprefixedID ?
}

impl From<Url> for Import {
    fn from(url: Url) -> Self {
        Import::Url(url)
    }
}

impl From<Identifier> for Import {
    fn from(id: Identifier) -> Self {
        Import::Abbreviated(id)
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Import::*;
        match self {
            Url(url) => url.fmt(f),
            Abbreviated(id) => id.fmt(f),
        }
    }
}

impl<'i> FromPair<'i> for Import {
    const RULE: Rule = Rule::Import;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iri => Ok(Url::parse(inner.as_str()).unwrap().into()), // FIXME
            Rule::Id => Identifier::from_pair_unchecked(inner).map(From::from),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Import);
