use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::DerefMut;
use std::result::Result;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::OboFrame;
use crate::semantics::Orderable;
use crate::syntax::Rule;

/// The header frame, containing metadata about an OBO document.
#[derive(Clone, Debug, Default, Eq, FromStr, Hash, PartialEq)]
pub struct HeaderFrame {
    clauses: Vec<HeaderClause>,
}

impl HeaderFrame {
    /// Create a new empty `HeaderFrame`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new `HeaderFrame` containing the provided clauses.
    pub fn with_clauses(clauses: Vec<HeaderClause>) -> Self {
        Self { clauses }
    }

    /// Create a new `HeaderFrame` containing only a single clause.
    pub fn from_clause(clause: HeaderClause) -> Self {
        Self::with_clauses(vec![clause])
    }

    /// Get the default namespace of the ontology, if any is declared.
    ///
    /// # Errors
    /// - `CardinalityError::MissingClause`: if the header frame does not
    ///   contain any default namespace definition.
    /// - `CardinalityError::DuplicateClauses` if the header frame does
    ///   contain more than one default namespace definition.
    pub fn default_namespace(&self) -> Result<&NamespaceIdent, CardinalityError> {
        let mut namespace: Option<&NamespaceIdent> = None;
        for clause in &self.clauses {
            if let HeaderClause::DefaultNamespace(ns) = clause {
                match namespace {
                    Some(_) => return Err(CardinalityError::duplicate("default-namespace")),
                    None => namespace = Some(ns),
                }
            }
        }
        namespace.ok_or_else(|| CardinalityError::missing("default-namespace"))
    }

    /// Get the format version of the ontology, if any is declared.
    pub fn format_version(&self) -> Result<&UnquotedString, CardinalityError> {
        let mut version: Option<&UnquotedString> = None;
        for clause in &self.clauses {
            if let HeaderClause::FormatVersion(v) = clause {
                match version {
                    Some(_) => return Err(CardinalityError::duplicate("format-version")),
                    None => version = Some(v),
                }
            }
        }
        version.ok_or_else(|| CardinalityError::missing("format-version"))
    }

    /// Get the data version of the ontology, if any is declared.
    pub fn data_version(&self) -> Result<&UnquotedString, CardinalityError> {
        let mut version: Option<&UnquotedString> = None;
        for clause in &self.clauses {
            if let HeaderClause::DataVersion(v) = clause {
                match version {
                    Some(_) => return Err(CardinalityError::duplicate("data-version")),
                    None => version = Some(v),
                }
            }
        }
        version.ok_or_else(|| CardinalityError::missing("data-version"))
    }

    /// Merge several OWL axioms into a single clause.
    pub fn merge_owl_axioms(&mut self) {
        let mut merged = Vec::new();
        let clauses_new = Vec::with_capacity(self.clauses.len());
        for clause in std::mem::replace(&mut self.clauses, clauses_new) {
            if let HeaderClause::OwlAxioms(axioms) = clause {
                merged.push(axioms.into_string());
            } else {
                self.clauses.push(clause);
            }
        }

        if !merged.is_empty() {
            let s = UnquotedString::new(merged.join("\n"));
            self.clauses.push(HeaderClause::OwlAxioms(Box::new(s)));
        }
    }
}

impl AsMut<[HeaderClause]> for HeaderFrame {
    fn as_mut(&mut self) -> &mut [HeaderClause] {
        &mut self.clauses
    }
}

impl AsMut<Vec<HeaderClause>> for HeaderFrame {
    fn as_mut(&mut self) -> &mut Vec<HeaderClause> {
        &mut self.clauses
    }
}

impl AsRef<[HeaderClause]> for HeaderFrame {
    fn as_ref(&self) -> &[HeaderClause] {
        &self.clauses
    }
}

impl AsRef<Vec<HeaderClause>> for HeaderFrame {
    fn as_ref(&self) -> &Vec<HeaderClause> {
        &self.clauses
    }
}

impl Deref for HeaderFrame {
    type Target = Vec<HeaderClause>;
    fn deref(&self) -> &Self::Target {
        &self.clauses
    }
}

impl DerefMut for HeaderFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clauses
    }
}

impl Display for HeaderFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for clause in self.clauses.iter() {
            clause.fmt(f).and(f.write_char('\n'))?;
        }
        Ok(())
    }
}

impl From<HeaderFrame> for Vec<HeaderClause> {
    fn from(frame: HeaderFrame) -> Self {
        frame.clauses
    }
}

impl From<HeaderClause> for HeaderFrame {
    fn from(clause: HeaderClause) -> Self {
        Self::from_clause(clause)
    }
}

impl From<Vec<HeaderClause>> for HeaderFrame {
    fn from(clauses: Vec<HeaderClause>) -> Self {
        Self::with_clauses(clauses)
    }
}

impl FromIterator<HeaderClause> for HeaderFrame {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = HeaderClause>,
    {
        Self::with_clauses(iter.into_iter().collect())
    }
}

impl<'i> FromPair<'i> for HeaderFrame {
    const RULE: Rule = Rule::HeaderFrame;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut clauses = Vec::new();
        for inner in pair.into_inner() {
            clauses.push(HeaderClause::from_pair_unchecked(inner, cache)?)
        }
        Ok(HeaderFrame::with_clauses(clauses))
    }
}

impl IntoIterator for HeaderFrame {
    type Item = HeaderClause;
    type IntoIter = <Vec<HeaderClause> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl<'a> IntoIterator for &'a HeaderFrame {
    type Item = &'a HeaderClause;
    type IntoIter = <&'a Vec<HeaderClause> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a mut HeaderFrame {
    type Item = &'a mut HeaderClause;
    type IntoIter = <&'a mut Vec<HeaderClause> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.clauses.as_mut_slice().iter_mut()
    }
}

impl Orderable for HeaderFrame {
    fn sort(&mut self) {
        // NB: not `sort_unstable` to avoid shuffling owl-axioms
        self.clauses.sort();
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

impl OboFrame for HeaderFrame {
    type Clause = HeaderClause;

    fn clauses_ref(&self) -> Vec<&Self::Clause> {
        self.clauses.iter().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn data_version() {
        let mut frame = HeaderFrame::new();
        self::assert_eq!(
            frame.data_version(),
            Err(CardinalityError::missing("data-version"))
        );

        let v = UnquotedString::from("1.4");
        frame.push(HeaderClause::DataVersion(Box::new(v.clone())));
        self::assert_eq!(frame.data_version(), Ok(&v));

        frame.push(HeaderClause::DataVersion(Box::new(v)));
        self::assert_eq!(
            frame.data_version(),
            Err(CardinalityError::duplicate("data-version"))
        );
    }

    #[test]
    fn default_namespace() {
        let mut frame = HeaderFrame::new();
        self::assert_eq!(
            frame.default_namespace(),
            Err(CardinalityError::missing("default-namespace"))
        );

        let ns = NamespaceIdent::from(UnprefixedIdent::new("TEST"));
        frame.push(HeaderClause::DefaultNamespace(Box::new(ns.clone())));
        self::assert_eq!(frame.default_namespace(), Ok(&ns));

        frame.push(HeaderClause::DefaultNamespace(Box::new(ns)));
        self::assert_eq!(
            frame.default_namespace(),
            Err(CardinalityError::duplicate("default-namespace"))
        );
    }

    #[test]
    fn format_version() {
        let mut frame = HeaderFrame::new();
        self::assert_eq!(
            frame.format_version(),
            Err(CardinalityError::missing("format-version"))
        );

        let v = UnquotedString::from("1.4");
        frame.push(HeaderClause::FormatVersion(Box::new(v.clone())));
        self::assert_eq!(frame.format_version(), Ok(&v));

        frame.push(HeaderClause::FormatVersion(Box::new(v)));
        self::assert_eq!(
            frame.format_version(),
            Err(CardinalityError::duplicate("format-version"))
        );
    }

    #[test]
    fn from_clause() {
        let clause = HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.0")));

        let frame = HeaderFrame::from_clause(clause.clone());
        self::assert_eq!(frame.clauses, vec![clause.clone()]);
        self::assert_eq!(frame, HeaderFrame::from(clause));
    }

    #[test]
    fn from_str() {
        let actual = HeaderFrame::from_str(
            "format-version: 1.2
            data-version: releases/2019-03-17
            subsetdef: gocheck_do_not_annotate \"Term not to be used for direct annotation\"
            synonymtypedef: syngo_official_label \"label approved by the SynGO project\"
            synonymtypedef: systematic_synonym \"Systematic synonym\" EXACT
            default-namespace: gene_ontology
            remark: cvs version: $Revision: 38972$
            remark: Includes Ontology(OntologyID(OntologyIRI(<http://purl.obolibrary.org/obo/go/never_in_taxon.owl>))) [Axioms: 18 Logical Axioms: 0]
            ontology: go
            property_value: http://purl.org/dc/elements/1.1/license http://creativecommons.org/licenses/by/4.0/"
        ).unwrap();

        assert_eq!(
            actual.clauses[0],
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.2"))),
        );

        assert_eq!(
            actual.clauses[1],
            HeaderClause::DataVersion(Box::new(UnquotedString::new("releases/2019-03-17"))),
        );

        assert_eq!(
            actual.clauses[2],
            HeaderClause::Subsetdef(
                Box::new(SubsetIdent::from(UnprefixedIdent::new(
                    "gocheck_do_not_annotate"
                ))),
                Box::new(QuotedString::new(
                    "Term not to be used for direct annotation"
                )),
            )
        );
    }

    #[test]
    fn new() {
        let frame = HeaderFrame::new();
        self::assert_eq!(frame.clauses, Vec::new());
    }

    #[test]
    fn is_sorted() {
        let frame = HeaderFrame::new();
        assert!(frame.is_sorted());

        let frame = HeaderFrame::with_clauses(vec![
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
            HeaderClause::DataVersion(Box::new(UnquotedString::new("v0.2.0"))),
            HeaderClause::SavedBy(Box::new(UnquotedString::new("Martin Larralde"))),
        ]);
        assert!(frame.is_sorted());

        let frame = HeaderFrame::with_clauses(vec![
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
            HeaderClause::SavedBy(Box::new(UnquotedString::new("Martin Larralde"))),
            HeaderClause::DataVersion(Box::new(UnquotedString::new("v0.2.0"))),
        ]);
        assert!(!frame.is_sorted());
    }

    #[test]
    fn sort() {
        let mut frame = HeaderFrame::with_clauses(vec![
            HeaderClause::SavedBy(Box::new(UnquotedString::new("Martin Larralde"))),
            HeaderClause::DataVersion(Box::new(UnquotedString::new("v0.2.0"))),
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
        ]);
        frame.sort();
        assert_eq!(
            frame,
            HeaderFrame::with_clauses(vec![
                HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
                HeaderClause::DataVersion(Box::new(UnquotedString::new("v0.2.0"))),
                HeaderClause::SavedBy(Box::new(UnquotedString::new("Martin Larralde"))),
            ])
        );
    }

    #[test]
    fn cardinality_check() {
        let frame = HeaderFrame::with_clauses(vec![
            HeaderClause::SavedBy(Box::new(UnquotedString::new("Martin Larralde"))),
            HeaderClause::DataVersion(Box::new(UnquotedString::new("v0.2.0"))),
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
        ]);
        assert!(frame.cardinality_check().is_ok());

        let frame2 = HeaderFrame::with_clauses(vec![
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.4"))),
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.5"))),
        ]);
        assert!(frame2.cardinality_check().is_err());
    }
}
