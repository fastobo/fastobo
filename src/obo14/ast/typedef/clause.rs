use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use crate::error::Result;
use crate::obo14::ast::*;
use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;

/// A clause appearing in a typedef frame.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TypedefClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetId),
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, XrefList),
    Xref(Xref),
    PropertyValue(PropertyValue),
    Domain(ClassId), // QUESTION(@althonos): Should be ID ?
    Range(ClassId),  // QUESTION(@althonos): same.
    Builtin(bool),
    HoldsOverChain(RelationId, RelationId),
    IsAntiSymmetric(bool),
    IsCyclic(bool),
    IsReflexive(bool),
    IsSymmetric(bool),
    IsTransitive(bool),
    IsFunctional(bool),
    IsInverseFunctional(bool),
    IsA(RelationId),
    IntersectionOf(RelationId),
    UnionOf(RelationId),
    EquivalentTo(RelationId),
    DisjointFrom(RelationId),
    InverseOf(RelationId),
    TransitiveOver(RelationId),
    EquivalentToChain(RelationId, RelationId),
    DisjointOver(RelationId),
    Relationship(RelationId, RelationId),
    IsObsolete(bool),
    ReplacedBy(RelationId),
    Consider(Id),
    CreatedBy(UnquotedString),
    CreationDate(IsoDate),
    ExpandAssertionTo(QuotedString, XrefList),
    ExpandExpressionTo(QuotedString, XrefList),
    IsMetadataTag(bool),
    IsClassLevel(bool),
}

impl Display for TypedefClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::TypedefClause::*;
        match self {
            IsAnonymous(b) => f.write_str("is_anonymous: ").and(b.fmt(f)),
            Name(name) => f.write_str("name: ").and(name.fmt(f)),
            Namespace(ns) => f.write_str("namespace: ").and(ns.fmt(f)),
            AltId(id) => f.write_str("alt_id: ").and(id.fmt(f)),
            Def(desc, xrefs) => f
                .write_str("def: ")
                .and(desc.fmt(f))
                .and(f.write_char(' '))
                .and(xrefs.fmt(f)),
            Comment(comment) => f.write_str("comment: ").and(comment.fmt(f)),
            Subset(id) => f.write_str("subset: ").and(id.fmt(f)),
            Synonym(desc, scope, optid, xrefs) => {
                f.write_str("synonym: ")
                    .and(desc.fmt(f))
                    .and(f.write_char(' '))
                    .and(scope.fmt(f))
                    .and(f.write_char(' '))?;
                if let Some(tyid) = optid {
                    tyid.fmt(f).and(f.write_char(' '))?;
                }
                xrefs.fmt(f)
            }
            Xref(xref) => f.write_str("xref: ").and(xref.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            Domain(id) => f.write_str("domain: ").and(id.fmt(f)),
            Range(id) => f.write_str("range: ").and(id.fmt(f)),
            Builtin(b) => f.write_str("builtin: ").and(b.fmt(f)),
            HoldsOverChain(r1, r2) => f
                .write_str("holds_over_chain: ")
                .and(r1.fmt(f))
                .and(f.write_char(' '))
                .and(r2.fmt(f)),
            IsAntiSymmetric(b) => f.write_str("is_anti_symmetric: ").and(b.fmt(f)),
            IsCyclic(b) => f.write_str("is_cyclic: ").and(b.fmt(f)),
            IsReflexive(b) => f.write_str("is_reflexive: ").and(b.fmt(f)),
            IsSymmetric(b) => f.write_str("is_symmetric: ").and(b.fmt(f)),
            IsTransitive(b) => f.write_str("is_transitive: ").and(b.fmt(f)),
            IsFunctional(b) => f.write_str("is_functional: ").and(b.fmt(f)),
            IsInverseFunctional(b) => f.write_str("is_inverse_functional: ").and(b.fmt(f)),
            IsA(r) => f.write_str("is_a: ").and(r.fmt(f)),
            IntersectionOf(r) => f.write_str("intersection_of: ").and(r.fmt(f)),
            UnionOf(r) => f.write_str("union_of: ").and(r.fmt(f)),
            EquivalentTo(r) => f.write_str("equivalent_to: ").and(r.fmt(f)),
            DisjointFrom(r) => f.write_str("disjoint_from: ").and(r.fmt(f)),
            InverseOf(r) => f.write_str("inverse_of: ").and(r.fmt(f)),
            TransitiveOver(r) => f.write_str("transitive_over: ").and(r.fmt(f)),
            EquivalentToChain(r1, r2) => f
                .write_str("equivalent_to_chain: ")
                .and(r1.fmt(f))
                .and(f.write_char(' '))
                .and(r2.fmt(f)),
            DisjointOver(r) => f.write_str("disjoint_over: ").and(r.fmt(f)),
            Relationship(r1, r2) => f
                .write_str("relationship: ")
                .and(r1.fmt(f))
                .and(f.write_char(' '))
                .and(r2.fmt(f)),
            IsObsolete(b) => f.write_str("is_obsolete: ").and(b.fmt(f)),
            ReplacedBy(r) => f.write_str("replaced_by: ").and(r.fmt(f)),
            Consider(id) => f.write_str("consider: ").and(id.fmt(f)),
            CreatedBy(s) => f.write_str("created_by: ").and(s.fmt(f)),
            CreationDate(date) => f.write_str("creation_date: ").and(date.fmt(f)),
            ExpandAssertionTo(desc, xrefs) => f
                .write_str("expand_assertion_to: ")
                .and(desc.fmt(f))
                .and(f.write_char(' '))
                .and(xrefs.fmt(f)),
            ExpandExpressionTo(desc, xrefs) => f
                .write_str("expand_expression_to: ")
                .and(desc.fmt(f))
                .and(f.write_char(' '))
                .and(xrefs.fmt(f)),
            IsMetadataTag(b) => f.write_str("is_metadata_tag: ").and(b.fmt(f)),
            IsClassLevel(b) => f.write_str("is_class_level: ").and(b.fmt(f)),
        }
    }
}

impl FromPair for Line<TypedefClause> {
    const RULE: Rule = Rule::TypedefClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clause = TypedefClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Line::<()>::from_pair_unchecked(eol)?.with_content(clause))
    }
}
impl_fromstr!(Line<TypedefClause>);

impl FromPair for TypedefClause {
    const RULE: Rule = Rule::TypedefClause;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::IsAnonymousTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAnonymous(b))
            }
            Rule::NameTag => {
                let n = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Name(n))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Namespace(ns))
            }
            Rule::AltIdTag => {
                let id = Id::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::AltId(id))
            }
            Rule::DefTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Def(desc, xrefs))
            }
            Rule::CommentTag => {
                let comment = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Comment(comment))
            }
            Rule::SubsetTag => {
                let id = SubsetId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Subset(id))
            }
            Rule::SynonymTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let scope = SynonymScope::from_pair_unchecked(inner.next().unwrap())?;

                let pair = inner.next().unwrap();
                match pair.as_rule() {
                    Rule::SynonymTypeId => {
                        let ty = SynonymTypeId::from_pair_unchecked(pair)?;
                        let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                        Ok(TypedefClause::Synonym(desc, scope, Some(ty), xrefs))
                    }
                    Rule::XrefList => {
                        let xrefs = XrefList::from_pair_unchecked(pair)?;
                        Ok(TypedefClause::Synonym(desc, scope, None, xrefs))
                    }
                    _ => unreachable!(),
                }
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Xref(xref))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::PropertyValue(pv))
            }
            Rule::DomainTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Domain(id))
            }
            Rule::RangeTag => {
                let id = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Range(id))
            }
            Rule::BuiltinTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Builtin(b))
            }
            Rule::HoldsOverChainTag => {
                let r1 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::HoldsOverChain(r1, r2))
            }
            Rule::IsAntiSymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAntiSymmetric(b))
            }
            Rule::IsCyclicTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsCyclic(b))
            }
            Rule::IsReflexiveTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsReflexive(b))
            }
            Rule::IsSymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsSymmetric(b))
            }
            Rule::IsTransitiveTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsTransitive(b))
            }
            Rule::IsFunctionalTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsFunctional(b))
            }
            Rule::IsInverseFunctionalTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsInverseFunctional(b))
            }
            Rule::IsATag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsA(id))
            }
            Rule::IntersectionOfTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IntersectionOf(id))
            }
            Rule::UnionOfTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::UnionOf(id))
            }
            Rule::EquivalentToTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentTo(id))
            }
            Rule::DisjointFromTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointFrom(id))
            }
            Rule::InverseOfTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::InverseOf(id))
            }
            Rule::TransitiveOverTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::TransitiveOver(id))
            }
            Rule::EquivalentToChainTag => {
                let r1 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentToChain(r1, r2))
            }
            Rule::DisjointOverTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointOver(id))
            }
            Rule::RelationshipTag => {
                let r1 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Relationship(r1, r2))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ReplacedBy(id))
            }
            Rule::ConsiderTag => {
                let id = Id::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Consider(id))
            }
            Rule::CreatedByTag => {
                let person = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::CreatedBy(person))
            }
            Rule::CreationDateTag => {
                let date = IsoDate::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::CreationDate(date))
            }
            Rule::ExpandAssertionToTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ExpandAssertionTo(desc, xrefs))
            }
            Rule::ExpandExpressionToTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ExpandExpressionTo(desc, xrefs))
            }
            Rule::IsMetadataTagTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsMetadataTag(b))
            }
            Rule::IsClassLevelTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsClassLevel(b))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(TypedefClause);
