use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;
use crate::obo14::ast::ClassId;
use crate::obo14::ast::Id;
use crate::obo14::ast::IsoDate;
use crate::obo14::ast::Line;
use crate::obo14::ast::NamespaceId;
use crate::obo14::ast::PersonId;
use crate::obo14::ast::PropertyValue;
use crate::obo14::ast::QuotedString;
use crate::obo14::ast::RelationId;
use crate::obo14::ast::SubsetId;
use crate::obo14::ast::SynonymScope;
use crate::obo14::ast::SynonymTypeId;
use crate::obo14::ast::UnquotedString;
use crate::obo14::ast::Xref;
use crate::obo14::ast::XrefList;
use crate::error::Result;

/// A clause appearing in a term frame.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TermClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetId),
    // FIXME(@althonos): use `Synonym` struct here.
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, XrefList),
    Xref(Xref),
    Builtin(bool),
    PropertyValue(PropertyValue),
    IsA(ClassId),
    IntersectionOf(Option<RelationId>, ClassId),
    UnionOf(ClassId),
    EquivalentTo(ClassId),
    DisjointFrom(ClassId),
    Relationship(RelationId, ClassId),
    IsObsolete(bool),
    ReplacedBy(ClassId),
    Consider(ClassId),
    CreatedBy(PersonId),
    CreationDate(IsoDate),
    // FIXME(@althonos): in the guide but not in the syntax.
    // ExpandAssertionTo(QuotedString, XrefList),
    // ExpandExpressionTO(QuotedString, XrefList),
    // IsMetadata(bool),
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
            Def(desc, xreflist) =>
                f.write_str("def: ").and(desc.fmt(f)).and(xreflist.fmt(f)),
            Comment(comment) =>
                f.write_str("comment: ").and(comment.fmt(f)),
            Subset(subset) => f.write_str("subset: ").and(subset.fmt(f)),
            Synonym(desc, scope, opttype, xreflist) => {
                f.write_str("synonym: ").and(desc.fmt(f)).and(f.write_char(' '))
                    .and(scope.fmt(f))?;
                if let Some(syntype) = opttype {
                    f.write_char(' ').and(syntype.fmt(f))?;
                }
                f.write_char(' ').and(xreflist.fmt(f))
            }
            Xref(xref) => f.write_str("xref: ").and(xref.fmt(f)),
            Builtin(b) => f.write_str("builtin: ").and(b.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            IsA(id) => f.write_str("is_a: ").and(id.fmt(f)),
            IntersectionOf(Some(rel), id) =>
                f.write_str("intersection_of: ").and(rel.fmt(f)).and(f.write_char(' '))
                    .and(id.fmt(f)),
            IntersectionOf(None, id) =>
                f.write_str("intersection_of: ").and(id.fmt(f)),
            UnionOf(id) => f.write_str("union_of: ").and(id.fmt(f)),
            EquivalentTo(id) => f.write_str("equivalent_to: ").and(id.fmt(f)),
            DisjointFrom(id) => f.write_str("disjoint_from: ").and(id.fmt(f)),
            Relationship(rel, id) => f.write_str("relationship: ").and(rel.fmt(f))
                .and(f.write_char(' ')).and(id.fmt(f)),
            IsObsolete(b) => f.write_str("is_obsolete: ").and(b.fmt(f)),
            ReplacedBy(id) => f.write_str("replaced_by: ").and(id.fmt(f)),
            Consider(id) => f.write_str("consider: ").and(id.fmt(f)),
            CreatedBy(id) => f.write_str("created_by: ").and(id.fmt(f)),
            CreationDate(date) => f.write_str("creation_date: ").and(date.fmt(f)),
        }
    }
}

impl FromPair for Line<TermClause> {
    const RULE: Rule = Rule::TermClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clause = TermClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Line::<()>::from_pair_unchecked(eol)?.with_content(clause))
    }
}
impl_fromstr!(Line<TermClause>);

impl FromPair for TermClause {
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
                let ns = NamespaceId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Namespace(ns))
            }
            Rule::AltIdTag => {
                let id = Id::from_pair_unchecked(inner.next().unwrap())?;
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
                let id = SubsetId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Subset(id))
            }
            Rule::SynonymTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let scope = SynonymScope::from_pair_unchecked(inner.next().unwrap())?;

                let pair = inner.next().unwrap();
                match pair.as_rule() {
                    Rule::SynonymTypeId => {
                        let ty = SynonymTypeId::from_pair_unchecked(pair)?;
                        let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                        Ok(TermClause::Synonym(desc, scope, Some(ty), xrefs))
                    }
                    Rule::XrefList => {
                        let xrefs = XrefList::from_pair_unchecked(pair)?;
                        Ok(TermClause::Synonym(desc, scope, None, xrefs))
                    }
                    _ => unreachable!()
                }
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Xref(xref))
            }
            Rule::BuiltinTag => {
                let b = bool::from_str(inner.next().unwrap().as_str()).unwrap();
                Ok(TermClause::Builtin(b))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::PropertyValue(pv))
            }
            Rule::IsATag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::IsA(id))
            }
            Rule::IntersectionOfTag => {
                let id = inner.next().unwrap();
                if id.as_rule() == Rule::ClassId {
                    let classid = ClassId::from_pair_unchecked(id)?;
                    Ok(TermClause::IntersectionOf(None, classid))
                } else {
                    let relid = RelationId::from_pair_unchecked(id)?;
                    let classid = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                    Ok(TermClause::IntersectionOf(Some(relid), classid))
                }
            }
            Rule::UnionOfTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::UnionOf(id))
            }
            Rule::EquivalentToTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::EquivalentTo(id))
            }
            Rule::DisjointFromTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::DisjointFrom(id))
            }
            Rule::RelationshipTag => {
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Relationship(rel, id))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_str(inner.next().unwrap().as_str()).unwrap();
                Ok(TermClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::ReplacedBy(id))
            }
            Rule::ConsiderTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::Consider(id))
            }
            Rule::CreatedByTag => {
                let id = PersonId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::CreatedBy(id))
            }
            Rule::CreationDateTag => {
                let dt = IsoDate::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TermClause::CreationDate(dt))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(TermClause);


#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::obo14::ast::PrefixedId;
    use crate::obo14::ast::IdPrefix;
    use crate::obo14::ast::IdLocal;
    use crate::obo14::ast::Xref;
    use crate::obo14::ast::UnprefixedId;
    use super::*;

    #[test]
    fn from_str() {
        let actual = TermClause::from_str("name: sample name").unwrap();
        let expected = TermClause::Name(UnquotedString::new("sample name"));
        assert_eq!(actual, expected);

        let actual = TermClause::from_str("def: \"A reference string relevant to the sample under study.\" [PSI:MS]").unwrap();
        let expected = TermClause::Def(
            QuotedString::new("A reference string relevant to the sample under study."),
            XrefList::from(vec![Xref::new(Id::Prefixed(PrefixedId::new(IdPrefix::new("PSI"), IdLocal::new("MS"))))])
        );
        assert_eq!(actual, expected);

        let actual = TermClause::from_str("synonym: \"chemical entity\" EXACT [UniProt]").unwrap();
        let expected = TermClause::Synonym(
            QuotedString::new("chemical entity"),
            SynonymScope::Exact,
            None,
            XrefList::from(vec![
                Xref::new(Id::from(UnprefixedId::new("UniProt")))
            ])
        );
        assert_eq!(actual, expected);

        let actual = TermClause::from_str("xref: CAS:22325-47-9 \"NIST Chemistry WebBook\"").unwrap();
        let expected = TermClause::Xref(
            Xref::with_desc(
                Id::from(PrefixedId::new(IdPrefix::new("CAS"), IdLocal::new("22325-47-9"))),
                QuotedString::new("NIST Chemistry WebBook"),
            )
        );
        assert_eq!(actual, expected);
    }

}