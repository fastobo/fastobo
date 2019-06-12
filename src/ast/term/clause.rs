use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in a term frame.
///
/// # Comparison
/// `TermClause` implements `PartialOrd` following the semantics of the OBO
/// specification: clauses will compare based on their serialization order
/// rather than on their alphabetic order; clauses of the same kind will be
/// ranked in the alphabetic order.
#[derive(Clone, Debug, Eq, Hash, Ord, OboClause, PartialEq, PartialOrd)]
pub enum TermClause {
    #[clause(cardinality = "ZeroOrOne")]
    IsAnonymous(bool),
    #[clause(cardinality = "ZeroOrOne")]
    Name(UnquotedString),
    #[clause(cardinality = "One")]
    Namespace(NamespaceIdent),
    AltId(Ident),
    #[clause(cardinality = "ZeroOrOne")]
    Def(QuotedString, XrefList),
    #[clause(cardinality = "ZeroOrOne")]
    Comment(UnquotedString),
    Subset(SubsetIdent),
    Synonym(Synonym),
    Xref(Xref),
    #[clause(cardinality = "ZeroOrOne")]
    Builtin(bool),
    #[clause(cardinality = "ZeroOrOne")]
    PropertyValue(PropertyValue),
    IsA(ClassIdent),
    #[clause(cardinality = "NotOne")]
    IntersectionOf(Option<RelationIdent>, ClassIdent),
    #[clause(cardinality = "NotOne")]
    UnionOf(ClassIdent),
    EquivalentTo(ClassIdent),
    DisjointFrom(ClassIdent),
    Relationship(RelationIdent, ClassIdent),
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(UnquotedString),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(IsoDateTime),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(ClassIdent),
    Consider(ClassIdent),
    // FIXME(@althonos): in the guide but not in the syntax.
    // ExpandAssertionTo(QuotedString, XrefList),
    // ExpandExpressionTO(QuotedString, XrefList),
    // IsMetadataTag(bool),
    // IsClassLevel(bool),
}

impl<'i> FromPair<'i> for Line<TermClause> {
    const RULE: Rule = Rule::TermClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = TermClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}
impl_fromstr!(Line<TermClause>);

impl<'i> FromPair<'i> for TermClause {
    const RULE: Rule = Rule::TermClause;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::IsAnonymousTag => {
                let b = bool::from_str(inner.next().unwrap().as_str()).unwrap();
                Ok(TermClause::IsAnonymous(b))
            }
            Rule::NameTag => {
                let name = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Name(name))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Namespace(ns))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::AltId(id))
            }
            Rule::DefTag => {
                let def = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Def(def, xrefs))
            }
            Rule::CommentTag => {
                let comment = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Comment(comment))
            }
            Rule::SubsetTag => {
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Subset(id))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Synonym(syn))
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Xref(xref))
            }
            Rule::BuiltinTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Builtin(b))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::PropertyValue(pv))
            }
            Rule::IsATag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::IsA(id))
            }
            Rule::IntersectionOfTag => {
                let id = inner.next().unwrap();
                if id.as_rule() == Rule::ClassId {
                    let classid = ClassIdent::from_pair_unchecked(id)?;
                    Ok(TermClause::IntersectionOf(None, classid))
                } else {
                    let relid = RelationIdent::from_pair_unchecked(id)?;
                    let classid = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                    Ok(TermClause::IntersectionOf(Some(relid), classid))
                }
            }
            Rule::UnionOfTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::UnionOf(id))
            }
            Rule::EquivalentToTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::EquivalentTo(id))
            }
            Rule::DisjointFromTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::DisjointFrom(id))
            }
            Rule::RelationshipTag => {
                let rel = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Relationship(rel, id))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::ReplacedBy(id))
            }
            Rule::ConsiderTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Consider(id))
            }
            Rule::CreatedByTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::CreatedBy(s))
            }
            Rule::CreationDateTag => {
                let dt = IsoDateTime::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::CreationDate(dt))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(TermClause);

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    mod is_anonymous {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("is_anonymous: true").unwrap();
            let expected = TermClause::IsAnonymous(true);
            self::assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let clause = TermClause::Name(UnquotedString::new("sample name"));
            self::assert_eq!(clause.to_string(), "name: sample name")
        }
    }

    mod name {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("name: sample name").unwrap();
            let expected = TermClause::Name(UnquotedString::new("sample name"));
            self::assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let clause = TermClause::Name(UnquotedString::new("sample name"));
            self::assert_eq!(clause.to_string(), "name: sample name")
        }
    }

    mod namespace {}

    mod alt_id {}

    mod def {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str(
                "def: \"A reference string relevant to the sample under study.\" [PSI:MS]",
            )
            .unwrap();
            let expected = TermClause::Def(
                QuotedString::new(String::from(
                    "A reference string relevant to the sample under study.",
                )),
                XrefList::from(vec![Xref::new(PrefixedIdent::new("PSI", "MS"))]),
            );
            self::assert_eq!(actual, expected);

            let actual = TermClause::from_str("def: \"OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms.\" [PSI:PI]").unwrap();
            let expected = TermClause::Def(
                QuotedString::new("OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms."),
                XrefList::from(vec![Xref::new(PrefixedIdent::new("PSI", "PI"))])
            );
            self::assert_eq!(actual, expected);
        }
    }

    mod comment {}

    mod subset {}

    mod synonym {
        use super::*;

        #[test]
        fn from_str() {
            let actual =
                TermClause::from_str("synonym: \"chemical entity\" EXACT [UniProt]").unwrap();
            let expected = TermClause::Synonym(Synonym::with_xrefs(
                QuotedString::new("chemical entity"),
                SynonymScope::Exact,
                XrefList::from(vec![Xref::new(UnprefixedIdent::new("UniProt"))]),
            ));
            self::assert_eq!(actual, expected);
        }
    }

    mod xref {
        use super::*;

        #[test]
        fn from_str() {
            let actual =
                TermClause::from_str("xref: CAS:22325-47-9 \"NIST Chemistry WebBook\"").unwrap();
            let expected = TermClause::Xref(Xref::with_desc(
                Ident::from(PrefixedIdent::new("CAS", "22325-47-9")),
                QuotedString::new("NIST Chemistry WebBook"),
            ));
            self::assert_eq!(actual, expected);

            let actual =
                TermClause::from_str("xref: Wikipedia:https\\://en.wikipedia.org/wiki/Gas")
                    .unwrap();
            let expected = TermClause::Xref(Xref::new(PrefixedIdent::new(
                "Wikipedia",
                "https://en.wikipedia.org/wiki/Gas",
            )));
            self::assert_eq!(actual, expected);
        }
    }

    mod builtin {}

    mod property_value {}

    mod is_a {}

    mod intersection_of {
        use super::*;

        #[test]
        fn from_str() {
            let actual =
                Line::<TermClause>::from_str("intersection_of: part_of PO:0020039 ! leaf lamina\n")
                    .unwrap();
            let expected = Line::from(TermClause::IntersectionOf(
                Some(RelationIdent::from(UnprefixedIdent::new("part_of"))),
                ClassIdent::from(PrefixedIdent::new("PO", "0020039")),
            ))
            .and_comment(Comment::new("leaf lamina"));
            self::assert_eq!(actual, expected);

            let actual =
                Line::<TermClause>::from_str("intersection_of: PO:0006016 ! leaf epidermis\n")
                    .unwrap();
            let expected = Line::with_comment(Comment::new("leaf epidermis")).and_inner(
                TermClause::IntersectionOf(
                    None,
                    ClassIdent::from(PrefixedIdent::new("PO", "0006016")),
                ),
            );
            self::assert_eq!(actual, expected);
        }
    }

    mod union_of {}

    mod equivalent_to {}

    mod disjoint_from {}

    mod relationship {}

    mod is_obsolete {}

    mod replaced_by {}

    mod consider {}

    mod created_by {}

    mod creation_date {}
}
