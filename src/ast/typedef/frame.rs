use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::DerefMut;

use pest::iterators::Pair;

use crate::ast::*;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A typedef clause, describing a relationship.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedefFrame {
    id: Line<RelationIdent>,
    clauses: Vec<Line<TypedefClause>>,
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
            clauses,
        }
    }

    /// Get a reference to the identifier of the `TypedefFrame`.
    pub fn id(&self) -> &Line<RelationIdent> {
        &self.id
    }

    /// Get a mutable reference to the identifier of the `TypedefFrame`.
    pub fn id_mut(&mut self) -> &mut Line<RelationIdent> {
        &mut self.id
    }

    /// Get the `TypedefClause`s of the `TypedefFrame`.
    pub fn clauses(&self) -> &Vec<Line<TypedefClause>> {
        &self.clauses
    }

    /// Get a mutable reference to the `TypedefClause`s of the `TypedefFrame`.
    pub fn clauses_mut(&mut self) -> &mut Vec<Line<TypedefClause>> {
        &mut self.clauses
    }
}

impl AsRef<Vec<Line<TypedefClause>>> for TypedefFrame {
    fn as_ref(&self) -> &Vec<Line<TypedefClause>> {
        &self.clauses
    }
}

impl AsRef<[Line<TypedefClause>]> for TypedefFrame {
    fn as_ref(&self) -> &[Line<TypedefClause>] {
        &self.clauses
    }
}

impl Deref for TypedefFrame {
    type Target = Vec<Line<TypedefClause>>;
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for TypedefFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
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
        let id = Eol::from_pair_unchecked(inner.next().unwrap())?.and_inner(relid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<TypedefClause>::from_pair_unchecked(pair)?);
        }

        Ok(TypedefFrame { id, clauses })
    }
}
impl_fromstr!(TypedefFrame);

impl IntoIterator for TypedefFrame {
    type Item = Line<TypedefClause>;
    type IntoIter = <Vec<Line<TypedefClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<'a> IntoIterator for &'a TypedefFrame {
    type Item = &'a Line<TypedefClause>;
    type IntoIter = <&'a Vec<Line<TypedefClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_slice().iter()
    }
}

impl Orderable for TypedefFrame {
    fn sort(&mut self) {
        self.clauses.sort_unstable();
    }
    fn is_sorted(&self) -> bool {
        for i in 1..self.clauses.len() {
            if self.clauses[i-1] > self.clauses[i] {
                return false;
            }
        }
        true
    }
}
