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

/// A typedef clause, describing a relationship.
#[derive(Clone, Debug, Eq, FromStr, Hash, PartialEq)]
pub struct TypedefFrame {
    id: Line<RelationIdent>,
    clauses: Vec<Line<TypedefClause>>,
}

impl TypedefFrame {
    /// Create a new typedef frame with the given identifier.
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Line<RelationIdent>>,
    {
        Self::with_clauses(id, Vec::new())
    }

    /// Create a new typedef frame from an identifier and a vector of clauses.
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

    /// Get the name of the typedef, if exactly one is declared.
    pub fn name(&self) -> Result<&UnquotedString, CardinalityError> {
        let mut name: Option<&UnquotedString> = None;
        for clause in &self.clauses {
            if let TypedefClause::Name(n) = clause.as_inner() {
                match name {
                    Some(_) => return Err(CardinalityError::duplicate("name")),
                    None => name = Some(n),
                }
            }
        }
        name.ok_or_else(|| CardinalityError::missing("name"))
    }

    /// Get the definition of the typedef, if exactly one is declared.
    pub fn definition(&self) -> Result<&Definition, CardinalityError> {
        let mut def: Option<&Definition> = None;
        for clause in &self.clauses {
            if let TypedefClause::Def(n) = clause.as_inner() {
                match def {
                    Some(_) => return Err(CardinalityError::duplicate("def")),
                    None => def = Some(n),
                }
            }
        }
        def.ok_or_else(|| CardinalityError::missing("def"))
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

impl Identified for TypedefFrame {
    /// Get a reference to the identifier of the term.
    fn as_id(&self) -> &Ident {
        self.id.as_inner().as_ref()
    }

    /// Get a mutable reference to the identifier of the term.
    fn as_id_mut(&mut self) -> &mut Ident {
        self.id.as_mut().as_mut()
    }
}

impl<'i> FromPair<'i> for TypedefFrame {
    const RULE: Rule = Rule::TypedefFrame;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap(), cache)?.and_inner(relid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<TypedefClause>::from_pair_unchecked(pair, cache)?);
        }

        Ok(TypedefFrame { id, clauses })
    }
}

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
            if self.clauses[i - 1] > self.clauses[i] {
                return false;
            }
        }
        true
    }
}

impl OboFrame for TypedefFrame {
    type Clause = TypedefClause;

    fn clauses_ref(&self) -> Vec<&Self::Clause> {
        self.clauses.iter().map(Line::as_inner).collect()
    }
}
