use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

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

    /// Create a new `HeaderFrame` containing only the provided clause.
    pub fn from_clause(clause: HeaderClause) -> Self {
        Self::with_clauses(vec![clause])
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
        T: IntoIterator<Item = HeaderClause>
    {
        Self::with_clauses(iter.into_iter().collect())
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

impl<'i> FromPair<'i> for HeaderFrame {
    const RULE: Rule = Rule::HeaderFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut clauses = Vec::new();
        for inner in pair.into_inner() {
            clauses.push(HeaderClause::from_pair_unchecked(inner)?)
        }
        Ok(HeaderFrame::with_clauses(clauses))
    }
}
impl_fromstr!(HeaderFrame);

// WIP(@althonos)
// pub struct HeaderFrameRef<'a> {
//     pub clauses: Cow<'a, &'a [HeaderClauseRef<'a>]>
// }

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use super::*;

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
}
