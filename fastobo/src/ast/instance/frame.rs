use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An instance frame, describing a particular individual.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InstanceFrame {
    pub id: Line<InstanceIdent>,
    pub clauses: Vec<Line<InstanceClause>>,
}

impl Display for InstanceFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Instance]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl<'i> FromPair<'i> for InstanceFrame {
    const RULE: Rule = Rule::InstanceFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let iid = InstanceIdent::from_pair_unchecked(inner.next().unwrap())?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap())?.and_inner(iid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<InstanceClause>::from_pair_unchecked(pair)?);
        }

        Ok(InstanceFrame { id, clauses })
    }
}
impl_fromstr!(InstanceFrame);
