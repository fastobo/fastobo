use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in a typedef frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TypedefClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceIdent),
    AltId(Ident),
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetIdent),
    Synonym(Synonym),
    Xref(Xref),
    PropertyValue(PropertyValue),
    Domain(ClassIdent), // QUESTION(@althonos): Should be ID ?
    Range(ClassIdent),  // QUESTION(@althonos): same.
    Builtin(bool),
    HoldsOverChain(RelationIdent, RelationIdent),
    IsAntiSymmetric(bool),
    IsCyclic(bool),
    IsReflexive(bool),
    IsSymmetric(bool),
    IsTransitive(bool),
    IsFunctional(bool),
    IsInverseFunctional(bool),
    IsA(RelationIdent),
    IntersectionOf(RelationIdent),
    UnionOf(RelationIdent),
    EquivalentTo(RelationIdent),
    DisjointFrom(RelationIdent),
    InverseOf(RelationIdent),
    TransitiveOver(RelationIdent),
    EquivalentToChain(RelationIdent, RelationIdent),
    DisjointOver(RelationIdent),
    Relationship(RelationIdent, RelationIdent),
    IsObsolete(bool),
    ReplacedBy(RelationIdent),
    Consider(Ident),
    CreatedBy(UnquotedString),
    CreationDate(IsoDateTime),
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
            Synonym(syn) => f.write_str("synonym: ").and(syn.fmt(f)),
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

impl<'i> FromPair<'i> for Line<TypedefClause> {
    const RULE: Rule = Rule::TypedefClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clause = TypedefClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Line::<()>::from_pair_unchecked(eol)?.with_content(clause))
    }
}
impl_fromstr!(Line<TypedefClause>);

impl<'i> FromPair<'i> for TypedefClause {
    const RULE: Rule = Rule::TypedefClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Namespace(ns))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
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
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Subset(id))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Synonym(syn))
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
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Domain(id))
            }
            Rule::RangeTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Range(id))
            }
            Rule::BuiltinTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Builtin(b))
            }
            Rule::HoldsOverChainTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
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
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsA(id))
            }
            Rule::IntersectionOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IntersectionOf(id))
            }
            Rule::UnionOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::UnionOf(id))
            }
            Rule::EquivalentToTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentTo(id))
            }
            Rule::DisjointFromTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointFrom(id))
            }
            Rule::InverseOfTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::InverseOf(id))
            }
            Rule::TransitiveOverTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::TransitiveOver(id))
            }
            Rule::EquivalentToChainTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::EquivalentToChain(r1, r2))
            }
            Rule::DisjointOverTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::DisjointOver(id))
            }
            Rule::RelationshipTag => {
                let r1 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let r2 = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Relationship(r1, r2))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::ReplacedBy(id))
            }
            Rule::ConsiderTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::Consider(id))
            }
            Rule::CreatedByTag => {
                let person = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::CreatedBy(person))
            }
            Rule::CreationDateTag => {
                let date = IsoDateTime::from_pair_unchecked(inner.next().unwrap())?;
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
