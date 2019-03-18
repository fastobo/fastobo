use std::fmt::Write;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Display;

use pest::iterators::Pair;

use crate::obo14::ast::RelationId;
use crate::obo14::ast::QuotedString;
use crate::error::Result;
use crate::obo14::parser::FromPair;
use crate::obo14::parser::Rule;

/// A qualifier, possibly used as a trailing modifier.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Qualifier {
    key: RelationId,
    value: QuotedString,
}

impl Display for Qualifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.key
            .fmt(f)
            .and(f.write_char('='))
            .and(self.value.fmt(f))
    }
}

impl FromPair for Qualifier {
    const RULE: Rule = Rule::Qualifier;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let key = RelationId::from_pair_unchecked(inner.next().unwrap())?;
        let value = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
        Ok(Qualifier { key, value })
    }
}
impl_fromstr!(Qualifier);
