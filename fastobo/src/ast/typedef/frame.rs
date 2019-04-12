use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::ast::*;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A typedef clause, describing a relationship.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedefFrame {
    pub id: Line<RelationIdent>,
    pub clauses: Vec<Line<TypedefClause>>,
}

impl TypedefFrame {
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Line<RelationIdent>>,
    {
        Self::with_clauses(id, Vec::new())
    }

    pub fn with_clauses<I>(id: I, clauses: Vec<Line<TypedefClause>>) -> Self
    where
        I: Into<Line<RelationIdent>>,
    {
        Self {
            id: id.into(),
            clauses: clauses,
        }
    }
}

impl Display for TypedefFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Typedef]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl<'i> FromPair<'i> for TypedefFrame {
    const RULE: Rule = Rule::TypedefFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
        let id = Line::<()>::from_pair_unchecked(inner.next().unwrap())?.with_content(relid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<TypedefClause>::from_pair_unchecked(pair)?);
        }

        Ok(TypedefFrame { id, clauses })
    }
}
impl_fromstr!(TypedefFrame);
