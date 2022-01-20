use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::ops::DerefMut;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::Identified;
use crate::semantics::OboFrame;
use crate::semantics::Orderable;
use crate::syntax::Rule;

/// An instance frame, describing a particular individual.
#[derive(Clone, Debug, Eq, FromStr, Hash, PartialEq)]
pub struct InstanceFrame {
    id: Line<InstanceIdent>,
    clauses: Vec<Line<InstanceClause>>,
}

impl InstanceFrame {
    /// Create a new instance frame with the given ID but without any clause.
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Line<InstanceIdent>>,
    {
        Self::with_clauses(id, Vec::new())
    }

    /// Create a new instance frame with the provided ID and clauses.
    pub fn with_clauses<I>(id: I, clauses: Vec<Line<InstanceClause>>) -> Self
    where
        I: Into<Line<InstanceIdent>>,
    {
        Self {
            id: id.into(),
            clauses,
        }
    }

    /// Get a reference to the identifier of the `InstanceFrame`.
    pub fn id(&self) -> &Line<InstanceIdent> {
        &self.id
    }

    /// Get a mutable reference to the identifier of the `InstanceFrame`.
    pub fn id_mut(&mut self) -> &mut Line<InstanceIdent> {
        &mut self.id
    }

    /// Get a reference to the `InstanceClause`s of the `InstanceFrame`.
    pub fn clauses(&self) -> &Vec<Line<InstanceClause>> {
        &self.clauses
    }

    /// Get a mutable reference to the `InstanceClause`s of the `InstanceFrame`.
    pub fn clauses_mut(&mut self) -> &mut Vec<Line<InstanceClause>> {
        &mut self.clauses
    }

    /// Get the name of the instance, if exactly one is declared.
    pub fn name(&self) -> Result<&UnquotedString, CardinalityError> {
        let mut name: Option<&UnquotedString> = None;
        for clause in &self.clauses {
            if let InstanceClause::Name(n) = clause.as_inner() {
                match name {
                    Some(_) => return Err(CardinalityError::duplicate("name")),
                    None => name = Some(n),
                }
            }
        }
        name.ok_or_else(|| CardinalityError::missing("name"))
    }

    /// Get the definition of the instance, if exactly one is declared.
    pub fn definition(&self) -> Result<&Definition, CardinalityError> {
        let mut def: Option<&Definition> = None;
        for clause in &self.clauses {
            if let InstanceClause::Def(n) = clause.as_inner() {
                match def {
                    Some(_) => return Err(CardinalityError::duplicate("def")),
                    None => def = Some(n),
                }
            }
        }
        def.ok_or_else(|| CardinalityError::missing("def"))
    }
}

impl AsRef<Vec<Line<InstanceClause>>> for InstanceFrame {
    fn as_ref(&self) -> &Vec<Line<InstanceClause>> {
        &self.clauses
    }
}

impl AsRef<[Line<InstanceClause>]> for InstanceFrame {
    fn as_ref(&self) -> &[Line<InstanceClause>] {
        &self.clauses
    }
}

impl Deref for InstanceFrame {
    type Target = Vec<Line<InstanceClause>>;
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for InstanceFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

impl Display for InstanceFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Instance]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl Identified for InstanceFrame {
    /// Get a reference to the identifier of the term.
    fn as_id(&self) -> &Ident {
        self.id.as_inner().as_ref()
    }

    /// Get a mutable reference to the identifier of the term.
    fn as_id_mut(&mut self) -> &mut Ident {
        self.id.as_mut().as_mut()
    }
}

impl<'i> FromPair<'i> for InstanceFrame {
    const RULE: Rule = Rule::InstanceFrame;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let iid = InstanceIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap(), cache)?.and_inner(iid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<InstanceClause>::from_pair_unchecked(pair, cache)?);
        }

        Ok(InstanceFrame { id, clauses })
    }
}

impl IntoIterator for InstanceFrame {
    type Item = Line<InstanceClause>;
    type IntoIter = <Vec<Line<InstanceClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<'a> IntoIterator for &'a InstanceFrame {
    type Item = &'a Line<InstanceClause>;
    type IntoIter = <&'a [Line<InstanceClause>] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_slice().iter()
    }
}

impl Orderable for InstanceFrame {
    fn sort(&mut self) {
        self.clauses.sort_unstable();
    }
    fn is_sorted(&self) -> bool {
        for i in 1..self.clauses.len() {
            if self.clauses[i - 1] > self.clauses[i] {
                return false;
            }
        }
        true
    }
}

impl OboFrame for InstanceFrame {
    type Clause = InstanceClause;

    fn clauses_ref(&self) -> Vec<&Self::Clause> {
        self.clauses.iter().map(Line::as_inner).collect()
    }
}
