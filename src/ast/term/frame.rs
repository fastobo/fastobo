use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::DerefMut;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::Identified;
use crate::semantics::OboFrame;
use crate::semantics::Orderable;
use crate::syntax::Rule;

/// A term frame, describing a class.
#[derive(Clone, Debug, Eq, FromStr, Hash, PartialEq)]
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

    /// Check if the class has a *genus-differentia* definition.
    ///
    /// *Genus-differentia* definition is a method of intensional definition
    /// which uses an existing definition (the *genus*) and portions of the
    /// new definition not provided by the *genera* (the *differentia*).
    ///
    /// A frame has such a definition if it contains some `intersection_of`
    /// clauses, but only one in the form `intersection_of: <ClassIdent>`.
    ///
    /// # See also
    /// - [Genus differentia definition](https://en.wikiversity.org/wiki/Dominant_group/Genus_differentia_definition)
    ///   on [Wikiversity](https://en.wikiversity.org/).
    pub fn is_genus_differentia(&self) -> bool {
        let mut has_differentia = false;
        let mut genus_count = 0;

        for clause in &self.clauses {
            if let TermClause::IntersectionOf(r, _) = clause.as_ref() {
                match r {
                    Some(_) => has_differentia = true,
                    None => genus_count += 1,
                }
            }
        }

        genus_count == 1 && has_differentia
    }

    /// Get the name (label) of the term, if exactly one is declared.
    pub fn name(&self) -> Result<&UnquotedString, CardinalityError> {
        let mut name: Option<&UnquotedString> = None;
        for clause in &self.clauses {
            if let TermClause::Name(n) = clause.as_inner() {
                match name {
                    Some(_) => return Err(CardinalityError::duplicate("name")),
                    None => name = Some(n),
                }
            }
        }
        name.ok_or_else(|| CardinalityError::missing("name"))
    }

    /// Get the definition of the term, if exactly one is declared.
    pub fn definition(&self) -> Result<&Definition, CardinalityError> {
        let mut def: Option<&Definition> = None;
        for clause in &self.clauses {
            if let TermClause::Def(n) = clause.as_inner() {
                match def {
                    Some(_) => return Err(CardinalityError::duplicate("def")),
                    None => def = Some(n),
                }
            }
        }
        def.ok_or_else(|| CardinalityError::missing("def"))
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

impl Identified for TermFrame {
    /// Get a reference to the identifier of the term.
    fn as_id(&self) -> &Ident {
        self.id.as_inner().as_ref()
    }

    /// Get a mutable reference to the identifier of the term.
    fn as_id_mut(&mut self) -> &mut Ident {
        self.id.as_mut().as_mut()
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
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        use crate::parser::QuickFind;
        let n = pair.as_str().quickcount(b'\n');

        let mut inner = pair.into_inner();
        let clsid = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let id = Eol::from_pair_unchecked(inner.next().unwrap(), cache)?.and_inner(clsid);

        let mut clauses = Vec::with_capacity(n - 1);

        for pair in inner {
            clauses.push(Line::<TermClause>::from_pair_unchecked(pair, cache)?);
        }

        Ok(TermFrame { id, clauses })
    }
}

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

impl Orderable for TermFrame {
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

impl OboFrame for TermFrame {
    type Clause = TermClause;

    fn clauses_ref(&self) -> Vec<&Self::Clause> {
        self.clauses.iter().map(Line::as_inner).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn is_genus_differentia() {
        // Genus w/ 1 differentia
        let term = TermFrame::from_str(
            "[Term]
            id: TEST:01
            intersection_of: TEST:02
            intersection_of: part_of TEST:03\n",
        )
        .unwrap();
        assert!(term.is_genus_differentia());

        // Genus w/ 1+ differentia
        let term = TermFrame::from_str(
            "[Term]
            id: TEST:01
            intersection_of: TEST:02
            intersection_of: part_of TEST:03
            intersection_of: has_part TEST:04\n",
        )
        .unwrap();
        assert!(term.is_genus_differentia());

        // Genus w/o differentia (cardinality error)
        let term = TermFrame::from_str(
            "[Term]
            id: TEST:01
            intersection_of: TEST:02\n",
        )
        .unwrap();
        assert!(!term.is_genus_differentia());

        // Differentia w/o genus (cardinality error)
        let term = TermFrame::from_str(
            "[Term]
            id: TEST:01
            intersection_of: part_of TEST:03\n",
        )
        .unwrap();
        assert!(!term.is_genus_differentia());

        // No intersection_of clause
        let term = TermFrame::from_str("[Term]\nid: TEST:01\n").unwrap();
        assert!(!term.is_genus_differentia());
    }

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
        )
        .is_ok());
    }
}
