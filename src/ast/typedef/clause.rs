use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;
use crate::semantics::OboClause;

/// A clause appearing in a typedef frame.
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, OboClause, PartialEq, PartialOrd)]
pub enum TypedefClause {
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
    PropertyValue(PropertyValue),
    #[clause(cardinality = "ZeroOrOne")]
    Domain(ClassIdent), // QUESTION(@althonos): Should be ID ?
    #[clause(cardinality = "ZeroOrOne")]
    Range(ClassIdent), // QUESTION(@althonos): same.
    #[clause(cardinality = "ZeroOrOne")]
    Builtin(bool),
    HoldsOverChain(RelationIdent, RelationIdent),
    #[clause(cardinality = "ZeroOrOne")]
    IsAntiSymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsCyclic(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsReflexive(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsSymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsAsymmetric(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsTransitive(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsFunctional(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsInverseFunctional(bool),
    IsA(RelationIdent),
    #[clause(cardinality = "NotOne")]
    IntersectionOf(RelationIdent),
    #[clause(cardinality = "NotOne")]
    UnionOf(RelationIdent),
    EquivalentTo(RelationIdent),
    DisjointFrom(RelationIdent),
    #[clause(cardinality = "ZeroOrOne")]
    InverseOf(RelationIdent),
    TransitiveOver(RelationIdent),
    EquivalentToChain(RelationIdent, RelationIdent),
    DisjointOver(RelationIdent),
    Relationship(RelationIdent, RelationIdent),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(RelationIdent),
    Consider(Ident),
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(UnquotedString),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(IsoDateTime),
    ExpandAssertionTo(QuotedString, XrefList),
    ExpandExpressionTo(QuotedString, XrefList),
    #[clause(cardinality = "ZeroOrOne")]
    IsMetadataTag(bool),
    #[clause(cardinality = "ZeroOrOne")]
    IsClassLevel(bool),
}

impl<'i> FromPair<'i> for Line<TypedefClause> {
    const RULE: Rule = Rule::TypedefClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = TypedefClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}

impl<'i> FromPair<'i> for TypedefClause {
    const RULE: Rule = Rule::TypedefClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
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
            Rule::IsAsymmetricTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(TypedefClause::IsAsymmetric(b))
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
