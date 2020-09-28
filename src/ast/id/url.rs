use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

#[derive(Clone, Debug, FromStr, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Url(StringType);

impl Url {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<'i> FromPair<'i> for Url {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        Ok(Url(StringType::from(pair.as_str())))
    }
}
