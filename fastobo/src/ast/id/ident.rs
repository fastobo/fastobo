use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::share::Share;
use crate::share::Redeem;
use crate::share::Cow;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

use super::PrefixedId;
use super::PrefixedIdent;
use super::UnprefixedId;
use super::UnprefixedIdent;
use super::Url;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ident {
    Prefixed(PrefixedIdent),
    Unprefixed(UnprefixedIdent),
    Url(Url),
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Ident::*;
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl From<PrefixedIdent> for Ident {
    fn from(id: PrefixedIdent) -> Self {
        Ident::Prefixed(id)
    }
}

impl From<UnprefixedIdent> for Ident {
    fn from(id: UnprefixedIdent) -> Self {
        Ident::Unprefixed(id)
    }
}

impl From<Url> for Ident {
    fn from(url: Url) -> Self {
        Ident::Url(url)
    }
}

impl<'i> FromPair<'i> for Ident {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedIdent::from_pair_unchecked(inner).map(From::from),
            Rule::UnprefixedId => UnprefixedIdent::from_pair_unchecked(inner).map(From::from),
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Url::from_pair_unchecked(inner).map(Ident::Url),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Ident);

impl<'a> Share<'a, Id<'a>> for Ident {
    fn share(&'a self) -> Id<'a> {
        use self::Ident::*;
        match self {
            Ident::Prefixed(ref id) => Id::Prefixed(Cow::Borrowed(id.share())),
            Ident::Unprefixed(ref id) => Id::Unprefixed(Cow::Borrowed(id)),
            Ident::Url(ref url) => Id::Url(Cow::Borrowed(url)),
        }
    }
}

/// A borrowed `Identifier`.
pub enum Id<'a> {
    Prefixed(Cow<'a, PrefixedId<'a>>),
    Unprefixed(Cow<'a, &'a UnprefixedId>),
    Url(Cow<'a, &'a Url>),
}

impl<'a> Display for Id<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Id::*;
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl<'a> From<PrefixedId<'a>> for Id<'a> {
     fn from(id: PrefixedId<'a>) -> Self {
         Id::Prefixed(Cow::Borrowed(id))
     }
}

impl<'a> From<&'a UnprefixedId> for Id<'a> {
    fn from(id: &'a UnprefixedId) -> Self {
        Id::Unprefixed(Cow::Borrowed(id))
    }
}

impl<'a> From<&'a Url> for Id<'a> {
    fn from(url: &'a Url) -> Self {
        Id::Url(Cow::Borrowed(url))
    }
}

impl<'i> FromPair<'i> for Id<'i> {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => Cow::<PrefixedId>::from_pair_unchecked(inner).map(Id::Prefixed),
            Rule::UnprefixedId => Cow::<&UnprefixedId>::from_pair_unchecked(inner).map(Id::Unprefixed),
            Rule::UrlId => Url::from_pair_unchecked(inner).map(Cow::Owned).map(Id::Url),
            _ => unreachable!()
        }
    }
}
impl_fromslice!('i, Id<'i>);

impl<'a> Redeem<'a> for Id<'a> {
    type Owned = Ident;
    fn redeem(&'a self) -> Ident {
        match self {
            Id::Prefixed(cow) => Ident::Prefixed(cow.redeem()),
            Id::Unprefixed(cow) => Ident::Unprefixed(cow.redeem()),
            Id::Url(cow) => Ident::Url(cow.redeem())
        }
    }
}
