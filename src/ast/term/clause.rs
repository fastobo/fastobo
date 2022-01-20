use fastobo_derive_internal::FromStr;
use fastobo_derive_internal::OboClause;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::OboClause;
use crate::syntax::Rule;

/// A clause appearing in a term frame.
///
/// # Comparison
/// `TermClause` implements `PartialOrd` following the semantics of the OBO
/// specification: clauses will compare based on their serialization order
/// rather than on their alphabetic order; clauses of the same kind will be
/// ranked in the alphabetic order.
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, OboClause, PartialEq, PartialOrd)]
pub enum TermClause {
    #[clause(cardinality = "ZeroOrOne")]
    IsAnonymous(bool),
    #[clause(cardinality = "ZeroOrOne")]
    Name(Box<UnquotedString>),
    #[clause(cardinality = "One")]
    Namespace(Box<NamespaceIdent>),
    AltId(Box<Ident>),
    #[clause(cardinality = "ZeroOrOne")]
    Def(Box<Definition>),
    #[clause(cardinality = "ZeroOrOne")]
    Comment(Box<UnquotedString>),
    Subset(Box<SubsetIdent>),
    Synonym(Box<Synonym>),
    Xref(Box<Xref>),
    #[clause(cardinality = "ZeroOrOne")]
    Builtin(bool),
    PropertyValue(Box<PropertyValue>),
    IsA(Box<ClassIdent>),
    #[clause(cardinality = "NotOne")]
    IntersectionOf(Option<Box<RelationIdent>>, Box<ClassIdent>),
    #[clause(cardinality = "NotOne")]
    UnionOf(Box<ClassIdent>),
    EquivalentTo(Box<ClassIdent>),
    DisjointFrom(Box<ClassIdent>),
    Relationship(Box<RelationIdent>, Box<ClassIdent>),
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(Box<UnquotedString>),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(Box<CreationDate>),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(Box<ClassIdent>),
    Consider(Box<ClassIdent>),
}

clause_impl_from!(TermClause);

impl<'i> FromPair<'i> for Line<TermClause> {
    const RULE: Rule = Rule::TermClauseLine;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = TermClause::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol, cache)?.and_inner(clause))
    }
}

impl<'i> FromPair<'i> for TermClause {
    const RULE: Rule = Rule::TermClause;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>, cache: &Cache) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::IsAnonymousTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::IsAnonymous(b))
            }
            Rule::NameTag => {
                let name = UnquotedString::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Name(Box::new(name)))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Namespace(Box::new(ns)))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::AltId(Box::new(id)))
            }
            Rule::DefTag => {
                let def = Definition::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Def(Box::new(def)))
            }
            Rule::CommentTag => {
                let comment = UnquotedString::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Comment(Box::new(comment)))
            }
            Rule::SubsetTag => {
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Subset(Box::new(id)))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Synonym(Box::new(syn)))
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Xref(Box::new(xref)))
            }
            Rule::BuiltinTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Builtin(b))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::PropertyValue(Box::new(pv)))
            }
            Rule::IsATag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::IsA(Box::new(id)))
            }
            Rule::IntersectionOfTag => {
                let id = inner.next().unwrap();
                if id.as_rule() == Rule::ClassId {
                    let classid = ClassIdent::from_pair_unchecked(id, cache)?;
                    Ok(TermClause::IntersectionOf(None, Box::new(classid)))
                } else {
                    let relid = RelationIdent::from_pair_unchecked(id, cache)?;
                    let classid = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                    Ok(TermClause::IntersectionOf(
                        Some(Box::new(relid)),
                        Box::new(classid),
                    ))
                }
            }
            Rule::UnionOfTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::UnionOf(Box::new(id)))
            }
            Rule::EquivalentToTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::EquivalentTo(Box::new(id)))
            }
            Rule::DisjointFromTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::DisjointFrom(Box::new(id)))
            }
            Rule::RelationshipTag => {
                let rel = RelationIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Relationship(Box::new(rel), Box::new(id)))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::ReplacedBy(Box::new(id)))
            }
            Rule::ConsiderTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::Consider(Box::new(id)))
            }
            Rule::CreatedByTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::CreatedBy(Box::new(s)))
            }
            Rule::CreationDateTag => {
                let dt = CreationDate::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(TermClause::CreationDate(Box::new(dt)))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

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
            let clause = TermClause::Name(Box::new(UnquotedString::new("sample name")));
            self::assert_eq!(clause.to_string(), "name: sample name")
        }
    }

    mod name {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("name: sample name").unwrap();
            let expected = TermClause::Name(Box::new(UnquotedString::new("sample name")));
            self::assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let clause = TermClause::Name(Box::new(UnquotedString::new("sample name")));
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
            let expected = TermClause::Def(Box::new(Definition::with_xrefs(
                QuotedString::new(String::from(
                    "A reference string relevant to the sample under study.",
                )),
                XrefList::from(vec![Xref::new(PrefixedIdent::new("PSI", "MS"))]),
            )));
            self::assert_eq!(actual, expected);

            let actual = TermClause::from_str("def: \"OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms.\" [PSI:PI]").unwrap();
            let expected = TermClause::Def(Box::new(Definition::with_xrefs(
                QuotedString::new("OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms."),
                XrefList::from(vec![Xref::new(PrefixedIdent::new("PSI", "PI"))])
            )));
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
            let expected = TermClause::Synonym(Box::new(Synonym::with_xrefs(
                QuotedString::new("chemical entity"),
                SynonymScope::Exact,
                XrefList::from(vec![Xref::new(UnprefixedIdent::new("UniProt"))]),
            )));
            self::assert_eq!(actual, expected);
        }
    }

    mod xref {
        use super::*;

        #[test]
        fn from_str() {
            let actual =
                TermClause::from_str("xref: CAS:22325-47-9 \"NIST Chemistry WebBook\"").unwrap();
            let expected = TermClause::Xref(Box::new(Xref::with_desc(
                Ident::from(PrefixedIdent::new("CAS", "22325-47-9")),
                QuotedString::new("NIST Chemistry WebBook"),
            )));
            self::assert_eq!(actual, expected);

            let actual =
                TermClause::from_str("xref: Wikipedia:https\\://en.wikipedia.org/wiki/Gas")
                    .unwrap();
            let expected = TermClause::Xref(Box::new(Xref::new(PrefixedIdent::new(
                "Wikipedia",
                "https://en.wikipedia.org/wiki/Gas",
            ))));
            self::assert_eq!(actual, expected);
        }
    }

    mod builtin {}

    mod property_value {}

    mod is_a {}

    mod intersection_of {
        use super::*;
        use textwrap_macros::dedent;

        #[test]
        fn from_str() {
            let actual = TermFrame::from_str(dedent!(
                "[Term]
                    id: TST:001
                    intersection_of: part_of PO:0020039 ! leaf lamina
                    "
            ))
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

            let expected = Line::from(TermClause::IntersectionOf(
                Some(Box::new(RelationIdent::from(UnprefixedIdent::new(
                    "part_of",
                )))),
                Box::new(ClassIdent::from(PrefixedIdent::new("PO", "0020039"))),
            ))
            .and_comment(Comment::new("leaf lamina"));
            self::assert_eq!(actual, expected);

            let actual = TermFrame::from_str(dedent!(
                "[Term]
                    id: TST:001
                    intersection_of: PO:0006016 ! leaf epidermis
                    "
            ))
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
            let expected = Line::with_comment(Comment::new("leaf epidermis")).and_inner(
                TermClause::IntersectionOf(
                    None,
                    Box::new(ClassIdent::from(PrefixedIdent::new("PO", "0006016"))),
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
