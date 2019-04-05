use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause appearing in an instance frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InstanceClause {
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
    InstanceOf(ClassIdent),
    Relationship(RelationIdent, Ident), // QUESTION(@althonos): InstanceId ?
    CreatedBy(UnquotedString),
    CreationDate(IsoDateTime),
    IsObsolete(bool),
    ReplacedBy(InstanceIdent),
    Consider(Ident),
}

impl Display for InstanceClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::InstanceClause::*;
        match self {
            IsAnonymous(b) => f.write_str("is_anonymous: ").and(b.fmt(f)),
            Name(n) => f.write_str("name: ").and(n.fmt(f)),
            Namespace(ns) => f.write_str("namespace: ").and(ns.fmt(f)),
            AltId(id) => f.write_str("alt_id: ").and(id.fmt(f)),
            Def(desc, xrefs) => f
                .write_str("def: ")
                .and(desc.fmt(f))
                .and(f.write_char(' '))
                .and(xrefs.fmt(f)),
            Comment(s) => f.write_str("comment: ").and(s.fmt(f)),
            Subset(id) => f.write_str("subset: ").and(id.fmt(f)),
            Synonym(syn) => f.write_str("synonym: ").and(syn.fmt(f)),
            Xref(xref) => f.write_str("xref: ").and(xref.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            InstanceOf(id) => f.write_str("instance_of: ").and(id.fmt(f)),
            Relationship(r, id) => f
                .write_str("relationship: ")
                .and(r.fmt(f))
                .and(f.write_char(' '))
                .and(id.fmt(f)),
            CreatedBy(s) => f.write_str("created_by: ").and(s.fmt(f)),
            CreationDate(dt) => f.write_str("creation_date: ").and(dt.fmt(f)),
            IsObsolete(b) => f.write_str("is_obsolete: ").and(b.fmt(f)),
            ReplacedBy(id) => f.write_str("replaced_by: ").and(id.fmt(f)),
            Consider(id) => f.write_str("consider: ").and(id.fmt(f)),
        }
    }
}

impl<'i> FromPair<'i> for InstanceClause {
    const RULE: Rule = Rule::InstanceClause;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clause = InstanceClause::from_pair_unchecked(inner.next().unwrap())?;
        Line::<()>::from_pair_unchecked(inner.next().unwrap()).map(|line| line.with_content(clause))
    }
}
impl_fromstr!(Line<InstanceClause>);
