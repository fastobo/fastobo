use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::result::Result;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::semantics::OboFrame;
use crate::semantics::Orderable;
use crate::share::Cow;
use crate::share::Redeem;
use crate::share::Share;

/// The header frame, containing metadata about an OBO document.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, OpaqueTypedef)]
#[opaque_typedef(allow_mut_ref)]
#[opaque_typedef(derive(
    AsRef(Inner, Self),
    AsMut(Inner, Self),
    Deref,
    DerefMut,
    Into(Inner),
    FromInner,
    PartialEq(Inner),
))]
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
                    None => version = Some(&v),
                }
            }
        }
        version.ok_or_else(|| CardinalityError::missing("format-version"))
    }

    /// Get the data version of the ontology, if any is declared.
    pub fn data_version(&self) -> Result<&str, CardinalityError> {
        let mut version: Option<&str> = None;
        for clause in &self.clauses {
            if let HeaderClause::DataVersion(v) = clause {
                match version {
                    Some(_) => return Err(CardinalityError::duplicate("data-version")),
                    None => version = Some(v.as_str()),
                }
            }
        }
        version.ok_or_else(|| CardinalityError::missing("data-version"))
    }
}

impl AsRef<[HeaderClause]> for HeaderFrame {
    fn as_ref(&self) -> &[HeaderClause] {
        &self.clauses
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

impl From<HeaderClause> for HeaderFrame {
    fn from(clause: HeaderClause) -> Self {
        Self::from_clause(clause)
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut clauses = Vec::new();
        for inner in pair.into_inner() {
            clauses.push(HeaderClause::from_pair_unchecked(inner)?)
        }
        Ok(HeaderFrame::with_clauses(clauses))
    }
}
impl_fromstr!(HeaderFrame);

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

    #[test]
    fn default_namespace() {
        let mut frame = HeaderFrame::new();
        self::assert_eq!(
            frame.default_namespace(),
            Err(CardinalityError::missing("default-namespace"))
        );

        let ns = NamespaceIdent::from(UnprefixedIdent::new("TEST"));
        frame.push(HeaderClause::DefaultNamespace(ns.clone()));
        self::assert_eq!(frame.default_namespace(), Ok(&ns));

        frame.push(HeaderClause::DefaultNamespace(ns.clone()));
        self::assert_eq!(
            frame.default_namespace(),
            Err(CardinalityError::duplicate("default-namespace"))
        );
    }

    #[test]
    fn from_clause() {
        let clause = HeaderClause::FormatVersion(UnquotedString::new("1.0"));

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
            HeaderClause::FormatVersion(UnquotedString::new("1.2")),
        );

        assert_eq!(
            actual.clauses[1],
            HeaderClause::DataVersion(UnquotedString::new("releases/2019-03-17")),
        );

        assert_eq!(
            actual.clauses[2],
            HeaderClause::Subsetdef(
                SubsetIdent::from(UnprefixedIdent::new("gocheck_do_not_annotate")),
                QuotedString::new("Term not to be used for direct annotation"),
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
            HeaderClause::FormatVersion(UnquotedString::new("1.4")),
            HeaderClause::DataVersion(UnquotedString::new("v0.2.0")),
            HeaderClause::SavedBy(UnquotedString::new("Martin Larralde")),
        ]);
        assert!(frame.is_sorted());

        let frame = HeaderFrame::with_clauses(vec![
            HeaderClause::FormatVersion(UnquotedString::new("1.4")),
            HeaderClause::SavedBy(UnquotedString::new("Martin Larralde")),
            HeaderClause::DataVersion(UnquotedString::new("v0.2.0")),
        ]);
        assert!(!frame.is_sorted());
    }

    #[test]
    fn sort() {
        let mut frame = HeaderFrame::with_clauses(vec![
            HeaderClause::SavedBy(UnquotedString::new("Martin Larralde")),
            HeaderClause::DataVersion(UnquotedString::new("v0.2.0")),
            HeaderClause::FormatVersion(UnquotedString::new("1.4")),
        ]);
        frame.sort();
        assert_eq!(
            frame,
            HeaderFrame::with_clauses(vec![
                HeaderClause::FormatVersion(UnquotedString::new("1.4")),
                HeaderClause::DataVersion(UnquotedString::new("v0.2.0")),
                HeaderClause::SavedBy(UnquotedString::new("Martin Larralde")),
            ])
        );
    }

    #[test]
    fn cardinality_check() {
        let frame = HeaderFrame::with_clauses(vec![
            HeaderClause::SavedBy(UnquotedString::new("Martin Larralde")),
            HeaderClause::DataVersion(UnquotedString::new("v0.2.0")),
            HeaderClause::FormatVersion(UnquotedString::new("1.4")),
        ]);
        assert!(frame.cardinality_check().is_ok());

        let frame2 = HeaderFrame::with_clauses(vec![
            HeaderClause::FormatVersion(UnquotedString::new("1.4")),
            HeaderClause::FormatVersion(UnquotedString::new("1.5")),
        ]);
        assert!(frame2.cardinality_check().is_err());
    }
}
