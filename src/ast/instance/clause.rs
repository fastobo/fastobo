use fastobo_derive_internal::FromStr;
use fastobo_derive_internal::OboClause;
use pest::iterators::Pair;

use crate::ast::*;

use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::semantics::OboClause;
use crate::syntax::Rule;

/// A clause appearing in an instance frame.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, OboClause, PartialEq, PartialOrd)]
pub enum InstanceClause {
    #[clause(cardinality = "ZeroOrOne")]
    IsAnonymous(bool),
    #[clause(cardinality = "ZeroOrOne")]
    Name(Box<UnquotedString>),
    #[clause(cardinality = "One")]
    Namespace(Box<NamespaceIdent>),
    AltId(Box<Ident>),
    #[clause(cardinality = "ZeroOrOne")]
    Def(Box<Definition>),
    Comment(Box<UnquotedString>),
    Subset(Box<SubsetIdent>),
    Synonym(Box<Synonym>),
    Xref(Box<Xref>),
    PropertyValue(Box<PropertyValue>),
    InstanceOf(Box<ClassIdent>),
    Relationship(Box<RelationIdent>, Box<Ident>), // QUESTION(@althonos): InstanceId ?
    #[clause(cardinality = "ZeroOrOne")]
    CreatedBy(Box<UnquotedString>),
    #[clause(cardinality = "ZeroOrOne")]
    CreationDate(Box<IsoDateTime>),
    #[clause(cardinality = "ZeroOrOne")]
    IsObsolete(bool),
    ReplacedBy(Box<InstanceIdent>),
    Consider(Box<Ident>),
}

impl From<Definition> for InstanceClause {
    fn from(d: Definition) -> Self {
        InstanceClause::Def(Box::new(d))
    }
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
                Ok(InstanceClause::Name(Box::new(n)))
            }
            Rule::NamespaceTag => {
                let ns = NamespaceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Namespace(Box::new(ns)))
            }
            Rule::AltIdTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::AltId(Box::new(id)))
            }
            Rule::DefTag => {
                let def = Definition::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Def(Box::new(def)))
            }
            Rule::CommentTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Comment(Box::new(s)))
            }
            Rule::SubsetTag => {
                let id = SubsetIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Subset(Box::new(id)))
            }
            Rule::SynonymTag => {
                let syn = Synonym::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Synonym(Box::new(syn)))
            }
            Rule::XrefTag => {
                let xref = Xref::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Xref(Box::new(xref)))
            }
            Rule::PropertyValueTag => {
                let pv = PropertyValue::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::PropertyValue(Box::new(pv)))
            }
            Rule::InstanceOfTag => {
                let id = ClassIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::InstanceOf(Box::new(id)))
            }
            Rule::RelationshipTag => {
                let r = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Relationship(Box::new(r), Box::new(id)))
            }
            Rule::CreatedByTag => {
                let s = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::CreatedBy(Box::new(s)))
            }
            Rule::CreationDateTag => {
                let dt = IsoDateTime::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::CreationDate(Box::new(dt)))
            }
            Rule::IsObsoleteTag => {
                let b = bool::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::IsObsolete(b))
            }
            Rule::ReplacedByTag => {
                let id = InstanceIdent::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::ReplacedBy(Box::new(id)))
            }
            Rule::ConsiderTag => {
                let id = Ident::from_pair_unchecked(inner.next().unwrap())?;
                Ok(InstanceClause::Consider(Box::new(id)))
            }
            _ => unreachable!(),
        }
    }
}

impl<'i> FromPair<'i> for Line<InstanceClause> {
    const RULE: Rule = Rule::InstanceClauseLine;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let clause = InstanceClause::from_pair_unchecked(inner.next().unwrap())?;
        let eol = inner.next().unwrap();
        Ok(Eol::from_pair_unchecked(eol)?.and_inner(clause))
    }
}
