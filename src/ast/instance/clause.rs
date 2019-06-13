use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::semantics::OboClause;

/// A clause appearing in an instance frame.
#[derive(Clone, Debug, Eq, Hash, Ord, OboClause, PartialEq, PartialOrd)]
pub enum InstanceClause {
    #[clause(cardinality = "ZeroOrOne")]
    IsAnonymous(bool),
    #[clause(cardinality = "ZeroOrOne")]
    Name(UnquotedString),
    #[clause(cardinality = "One")]
    Namespace(NamespaceIdent),
    AltId(Ident),
    #[clause(cardinality = "ZeroOrOne")]
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetIdent),
    Synonym(Synonym),
    Xref(Xref),
    PropertyValue(PropertyValue),
    InstanceOf(ClassIdent),
    Relationship(RelationIdent, Ident), // QUESTION(@althonos): InstanceId ?
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(UnquotedString),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(IsoDateTime),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(InstanceIdent),
    Consider(Ident),
}

impl<'i> FromPair<'i> for InstanceClause {
    const RULE: Rule = Rule::InstanceClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        match inner.next().unwrap().as_rule() {
            Rule::IsAnonymousTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::IsAnonymous(b))
            }
            Rule::NameTag => {
                let n = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Name(n))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Namespace(ns))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::AltId(id))
            }
            Rule::DefTag => {
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Def(desc, xrefs))
            }
            Rule::CommentTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Comment(s))
            }
            Rule::SubsetTag => {
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Subset(id))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Synonym(syn))
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Xref(xref))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::PropertyValue(pv))
            }
            Rule::InstanceOfTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::InstanceOf(id))
            }
            Rule::RelationshipTag => {
                let r = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Relationship(r, id))
            }
            Rule::CreatedByTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::CreatedBy(s))
            }
            Rule::CreationDateTag => {
                let dt = IsoDateTime::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::CreationDate(dt))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = InstanceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::ReplacedBy(id))
            }
            Rule::ConsiderTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Consider(id))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(InstanceClause);

impl<'i> FromPair<'i> for Line<InstanceClause> {
    const RULE: Rule = Rule::InstanceClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = InstanceClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}
impl_fromstr!(Line<InstanceClause>);
