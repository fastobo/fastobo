use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::DerefMut;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A term frame, describing a class.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TermFrame {
    id: Line<ClassIdent>,
    clauses: Vec<Line<TermClause>>,
}

impl TermFrame {
    /// Create a new term frame with the given ID but without any clause.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use std::str::FromStr;
    /// # use fastobo::ast::*;
    /// let id = ClassIdent::from(PrefixedIdent::new("MS", "1000031"));
    /// let frame = TermFrame::new(id);
    /// assert_eq!(frame.to_string(), "[Term]\nid: MS:1000031\n");
    /// ```
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Line<ClassIdent>>,
    {
        Self::with_clauses(id, Vec::new())
    }

    /// Create a new term frame with the provided ID and clauses.
    pub fn with_clauses<I>(id: I, clauses: Vec<Line<TermClause>>) -> Self
    where
        I: Into<Line<ClassIdent>>,
    {
        Self {
            id: id.into(),
            clauses,
        }
    }

    /// Get a reference to the identifier of the `TermFrame`.
    pub fn id(&self) -> &Line<ClassIdent> {
        &self.id
    }

    /// Get a mutable reference to the identifier of the `TermFrame`.
    pub fn id_mut(&mut self) -> &mut Line<ClassIdent> {
        &mut self.id
    }

    /// Get the `TermClause`s of the `TermFrame`.
    pub fn clauses(&self) -> &Vec<Line<TermClause>> {
        &self.clauses
    }

    /// Get a mutable reference to the `TermClause`s of the `TermFrame`.
    pub fn clauses_mut(&mut self) -> &mut Vec<Line<TermClause>> {
        &mut self.clauses
    }
}

impl AsRef<Vec<Line<TermClause>>> for TermFrame {
    fn as_ref(&self) -> &Vec<Line<TermClause>> {
        &self.clauses
    }
}

impl AsRef<[Line<TermClause>]> for TermFrame {
    fn as_ref(&self) -> &[Line<TermClause>] {
        &self.clauses
    }
}

impl Deref for TermFrame {
    type Target = Vec<Line<TermClause>>;
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for TermFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

impl Display for TermFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Term]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

/// Create a new term frame with the frame ID given as a `Line`.
impl From<Line<ClassIdent>> for TermFrame {
    fn from(line: Line<ClassIdent>) -> Self {
        Self::new(line)
    }
}

/// Create a new term frame with the frame ID given as a `ClassIdent`.
impl From<ClassIdent> for TermFrame {
    fn from(id: ClassIdent) -> Self {
        Self::new(id)
    }
}

impl<'i> FromPair<'i> for TermFrame {
    const RULE: Rule = Rule::TermFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clsid = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap())?.and_inner(clsid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<TermClause>::from_pair_unchecked(pair)?);
        }

        Ok(TermFrame { id, clauses })
    }
}
impl_fromstr!(TermFrame);

impl IntoIterator for TermFrame {
    type Item = Line<TermClause>;
    type IntoIter = <Vec<Line<TermClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<'a> IntoIterator for &'a TermFrame {
    type Item = &'a Line<TermClause>;
    type IntoIter = <&'a Vec<Line<TermClause>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_slice().iter()
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_str() {
        let actual = TermFrame::from_str(
            "[Term]
            id: MS:1000008
            name: ionization type
            def: \"The method by which gas phase ions are generated from the sample.\" [PSI:MS]
            relationship: part_of MS:1000458 ! source\n",
        )
        .unwrap();
        self::assert_eq!(
            actual.id.as_ref(),
            &ClassIdent::from(Ident::from(PrefixedIdent::new("MS", "1000008")))
        );

        assert!(TermFrame::from_str(
            "[Term]
            id: PO:0000067
            name: proteoid root
            namespace: plant_anatomy
            xref: PO_GIT:588
            is_a: PO:0009005 ! root
            created_by: austinmeier
            creation_date: 2015-08-11T15:05:12Z\n",
        ).is_ok());
    }
}
