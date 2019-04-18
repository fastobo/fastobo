use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in a term frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TermClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceIdent),
    AltId(Ident),
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetIdent),
    Synonym(Synonym),
    Xref(Xref),
    Builtin(bool),
    PropertyValue(PropertyValue),
    IsA(ClassIdent),
    IntersectionOf(Option<RelationIdent>, ClassIdent),
    UnionOf(ClassIdent),
    EquivalentTo(ClassIdent),
    DisjointFrom(ClassIdent),
    Relationship(RelationIdent, ClassIdent),
    IsObsolete(bool),
    ReplacedBy(ClassIdent),
    Consider(ClassIdent),
    CreatedBy(UnquotedString),
    CreationDate(IsoDateTime),
    // FIXME(@althonos): in the guide but not in the syntax.
    // ExpandAssertionTo(QuotedString, XrefList),
    // ExpandExpressionTO(QuotedString, XrefList),
    // IsMetadataTag(bool),
    // IsClassLevel(bool),
}

impl Display for TermClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::TermClause::*;
        match self {
            IsAnonymous(b) => f.write_str("is_anonymous: ").and(b.fmt(f)),
            Name(name) => f.write_str("name: ").and(name.fmt(f)),
            Namespace(id) => f.write_str("namespace: ").and(id.fmt(f)),
            AltId(id) => f.write_str("alt_id: ").and(id.fmt(f)),
            Def(desc, xreflist) => f.write_str("def: ").and(desc.fmt(f)).and(xreflist.fmt(f)),
            Comment(comment) => f.write_str("comment: ").and(comment.fmt(f)),
            Subset(subset) => f.write_str("subset: ").and(subset.fmt(f)),
            Synonym(syn) => f.write_str("synonym: ").and(syn.fmt(f)),
            Xref(xref) => f.write_str("xref: ").and(xref.fmt(f)),
            Builtin(b) => f.write_str("builtin: ").and(b.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            IsA(id) => f.write_str("is_a: ").and(id.fmt(f)),
            IntersectionOf(Some(rel), id) => f
                .write_str("intersection_of: ")
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(id.fmt(f)),
            IntersectionOf(None, id) => f.write_str("intersection_of: ").and(id.fmt(f)),
            UnionOf(id) => f.write_str("union_of: ").and(id.fmt(f)),
            EquivalentTo(id) => f.write_str("equivalent_to: ").and(id.fmt(f)),
            DisjointFrom(id) => f.write_str("disjoint_from: ").and(id.fmt(f)),
            Relationship(rel, id) => f
                .write_str("relationship: ")
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(id.fmt(f)),
            IsObsolete(b) => f.write_str("is_obsolete: ").and(b.fmt(f)),
            ReplacedBy(id) => f.write_str("replaced_by: ").and(id.fmt(f)),
            Consider(id) => f.write_str("consider: ").and(id.fmt(f)),
            CreatedBy(s) => f.write_str("created_by: ").and(s.fmt(f)),
            CreationDate(date) => f.write_str("creation_date: ").and(date.fmt(f)),
        }
    }
}

impl<'i> FromPair<'i> for Line<TermClause> {
    const RULE: Rule = Rule::TermClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clause = TermClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}
impl_fromstr!(Line<TermClause>);

impl<'i> FromPair<'i> for TermClause {
    const RULE: Rule = Rule::TermClause;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
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

    mod is_anonymous {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("is_anonymous: true").unwrap();
            let expected = TermClause::IsAnonymous(true);
            assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let clause = TermClause::Name(UnquotedString::new(String::from("sample name")));
            assert_eq!(clause.to_string(), "name: sample name")
        }
    }

    mod name {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("name: sample name").unwrap();
            let expected = TermClause::Name(UnquotedString::new(String::from("sample name")));
            assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let clause = TermClause::Name(UnquotedString::new(String::from("sample name")));
            assert_eq!(clause.to_string(), "name: sample name")
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
                QuotedString::new(String::from("A reference string relevant to the sample under study.")),
                XrefList::from(vec![Xref::new(Ident::Prefixed(PrefixedIdent::new(
                    IdentPrefix::new(String::from("PSI")),
                    IdentLocal::new(String::from("MS")),
                )))]),
            );
            assert_eq!(actual, expected);

            let actual = TermClause::from_str("def: \"OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms.\" [PSI:PI]").unwrap();
            let expected = TermClause::Def(
                QuotedString::new(String::from("OBSOLETE: There is Phenyx:ScoringModel for Phenyx! Scoring model (more detailed granularity). TODO: add some child terms.")),
                XrefList::from(vec![Xref::new(Ident::from(PrefixedIdent::new(IdentPrefix::new(String::from("PSI")), IdentLocal::new(String::from("PI")))))])
            );
            assert_eq!(actual, expected);
        }
    }

    mod comment {}

    mod subset {}

    mod synonym {
        use super::*;

        #[test]
        fn from_str() {
            let actual = TermClause::from_str("synonym: \"chemical entity\" EXACT [UniProt]").unwrap();
            let expected = TermClause::Synonym(Synonym::with_xrefs(
                QuotedString::new(String::from("chemical entity")),
                SynonymScope::Exact,
                XrefList::from(vec![Xref::new(Ident::from(UnprefixedIdent::new(String::from("UniProt"))))]),
            ));
            assert_eq!(actual, expected);
        }
    }

    mod xref {
        use super::*;

        #[test]
        fn from_str() {
            let actual =
                TermClause::from_str("xref: CAS:22325-47-9 \"NIST Chemistry WebBook\"").unwrap();
            let expected = TermClause::Xref(Xref::with_desc(
                Ident::from(PrefixedIdent::new(
                    IdentPrefix::new(String::from("CAS")),
                    IdentLocal::new(String::from("22325-47-9")),
                )),
                QuotedString::new(String::from("NIST Chemistry WebBook")),
            ));
            assert_eq!(actual, expected);

            let actual =
                TermClause::from_str("xref: Wikipedia:https\\://en.wikipedia.org/wiki/Gas").unwrap();
            let expected = TermClause::Xref(Xref::new(Ident::from(PrefixedIdent::new(
                IdentPrefix::new(String::from("Wikipedia")),
                IdentLocal::new(String::from("https://en.wikipedia.org/wiki/Gas")),
            ))));
            assert_eq!(actual, expected);
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
            let expected =
            Line::from(
                TermClause::IntersectionOf(
                    Some(RelationIdent::from(Ident::from(UnprefixedIdent::new(String::from("part_of"))))),
                    ClassIdent::from(Ident::from(PrefixedIdent::new(
                        IdentPrefix::new(String::from("PO")),
                        IdentLocal::new(String::from("0020039")),
                    ))),
                ),
            ).and_comment(Comment::new("leaf lamina"));
            assert_eq!(actual, expected);

            let actual =
                Line::<TermClause>::from_str("intersection_of: PO:0006016 ! leaf epidermis\n").unwrap();
            let expected = Line::with_comment(Comment::new(String::from("leaf epidermis"))).and_inner(
                TermClause::IntersectionOf(
                    None,
                    ClassIdent::from(Ident::from(PrefixedIdent::new(
                        IdentPrefix::new(String::from("PO")),
                        IdentLocal::new(String::from("0006016")),
                    ))),
                ),
            );
            assert_eq!(actual, expected);
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
