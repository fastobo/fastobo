use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;
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
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct HeaderFrame {
    pub clauses: Vec<HeaderClause>,
}

impl HeaderFrame {
    pub fn new(clauses: Vec<HeaderClause>) -> Self {
        Self { clauses }
    }
}

impl Display for HeaderFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut clauses = self.clauses.iter().peekable();
        while let Some(clause) = clauses.next() {
            clause.fmt(f).and(f.write_char('\n'))?;
        }
        Ok(())
    }
}

impl FromIterator<HeaderClause> for HeaderFrame {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = HeaderClause>
    {
        Self::new(iter.into_iter().collect())
    }
}

impl<'i> FromPair<'i> for HeaderFrame {
    const RULE: Rule = Rule::HeaderFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut clauses = Vec::new();
        for inner in pair.into_inner() {
            clauses.push(HeaderClause::from_pair_unchecked(inner)?)
        }
        Ok(HeaderFrame { clauses })
    }
}
impl_fromstr!(HeaderFrame);

// WIP(@althonos)
// pub struct HeaderFrameRef<'a> {
//     pub clauses: Cow<'a, &'a [HeaderClauseRef<'a>]>
// }

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::UnquotedString;

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
            HeaderClause::FormatVersion(UnquotedString::new(String::from("1.2"))),
        );
    }
}
