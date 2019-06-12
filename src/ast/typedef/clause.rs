use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in a typedef frame.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "_derive", derive(OboClause))]
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
    IsAsymmetric(bool),
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

// #[cfg(feature = "ext")]
// impl crate::ext::CardinalityBound for TypedefClause {
//     fn cardinality(&self) -> crate::ext::Cardinality {
//         use self::TypedefClause::*;
//         use crate::ext::Cardinality::*;
//         match self {
//             IsAnonymous(_) => ZeroOrOne,
//             Name(_) => ZeroOrOne,
//             Namespace(_) => One,
//             AltId(_) => Any,
//             Def(_, _) => ZeroOrOne,
//             Comment(_) => ZeroOrOne,
//             Subset(_) => ZeroOrOne,
//             Synonym(_) => Any,
//             Xref(_) => Any,
//             PropertyValue(_) => ZeroOrOne,
//             Domain(_) => ZeroOrOne, // QUESTION(@althonos): Should be ID ?
//             Range(_) => ZeroOrOne,  // QUESTION(@althonos): same.
//             Builtin(_) => ZeroOrOne,
//             HoldsOverChain(_, _) => Any,
//             IsAntiSymmetric(_) => ZeroOrOne,
//             IsCyclic(_) => ZeroOrOne,
//             IsReflexive(_) => ZeroOrOne,
//             IsSymmetric(_) => ZeroOrOne,
//             IsAsymmetric(_) => ZeroOrOne,
//             IsTransitive(_) => ZeroOrOne,
//             IsFunctional(_) => ZeroOrOne,
//             IsInverseFunctional(_) => ZeroOrOne,
//             IsA(_) => Any,
//             IntersectionOf(_) => NotOne,
//             UnionOf(_) => NotOne,
//             EquivalentTo(_) => Any,
//             DisjointFrom(_) => Any,
//             InverseOf(_) => ZeroOrOne,
//             TransitiveOver(_) => Any,
//             EquivalentToChain(_, _) => Any,
//             DisjointOver(_) => Any,
//             Relationship(_, _) => Any,
//             IsObsolete(_) => ZeroOrOne,
//             ReplacedBy(_) => Any,
//             Consider(_) => Any,
//             CreatedBy(_) => ZeroOrOne,
//             CreationDate(_) => ZeroOrOne,
//             ExpandAssertionTo(_, _) => Any,
//             ExpandExpressionTo(_, _) => Any,
//             IsMetadataTag(_) => ZeroOrOne,
//             IsClassLevel(_) => ZeroOrOne,
//         }
//     }
// }

impl<'i> FromPair<'i> for Line<TypedefClause> {
    const RULE: Rule = Rule::TypedefClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = TypedefClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}
impl_fromstr!(Line<TypedefClause>);

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
impl_fromstr!(TypedefClause);
