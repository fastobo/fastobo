use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::obo14::ast::*;
use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;
use crate::error::Result;

/// An instance frame, describing a particular individual.
pub struct InstanceFrame {
    id: Line<InstanceId>,
    clauses: Vec<Line<InstanceClause>>,
}

impl Display for InstanceFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Instance]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl FromPair for InstanceFrame {
    const RULE: Rule = Rule::InstanceFrame;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let iid = InstanceId::from_pair_unchecked(inner.next().unwrap())?;
        let id = Line::<()>::from_pair_unchecked(inner.next().unwrap())?
            .with_content(iid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<InstanceClause>::from_pair_unchecked(pair)?);
        }

        Ok(InstanceFrame { id, clauses })
    }
}
impl_fromstr!(InstanceFrame);
