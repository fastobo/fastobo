//! Identifiers used in OBO documents.
//!
//! `Ident` refers to an *owned* identifier, while `Id` refers to its *borrowed*
//! counterpart.

mod local;
mod prefix;
mod prefixed;
mod subclasses;
mod unprefixed;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use url::Url;

pub use self::local::IdentLocal;
pub use self::local::IdLocal;
pub use self::prefix::IdentPrefix;
pub use self::prefix::IdPrefix;
pub use self::prefixed::PrefixedId;
pub use self::prefixed::PrefixedIdent;
pub use self::subclasses::ClassIdent;
pub use self::subclasses::ClassId;
pub use self::subclasses::InstanceIdent;
pub use self::subclasses::InstanceId;
pub use self::subclasses::NamespaceIdent;
pub use self::subclasses::NamespaceId;
pub use self::subclasses::RelationIdent;
pub use self::subclasses::RelationId;
pub use self::subclasses::SubsetIdent;
pub use self::subclasses::SubsetId;
pub use self::subclasses::SynonymTypeIdent;
pub use self::subclasses::SynonymTypeId;
pub use self::unprefixed::UnprefixedIdent;
pub use self::unprefixed::UnprefixedId;

use crate::borrow::Borrow;
use crate::borrow::ToOwned;
use crate::borrow::Cow;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Ident {
    Prefixed(PrefixedIdent),
    Unprefixed(UnprefixedIdent),
    Url(Url),
}

impl<'a> Borrow<'a, Id<'a>> for Ident {
    fn borrow(&'a self) -> Id<'a> {
        use self::Ident::*;
        match self {
            Ident::Prefixed(ref id) => Id::Prefixed(Cow::Borrowed(id.borrow())),
            Ident::Unprefixed(ref id) => Id::Unprefixed(Cow::Borrowed(id)),
            Ident::Url(ref url) => Id::Url(Cow::Borrowed(url)),
        }
    }
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
            Rule::UrlId => Ok(Ident::Url(Url::parse(inner.as_str()).unwrap())),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Ident);

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

impl<'a> ToOwned<'a> for Id<'a> {
    type Owned = Ident;
    fn to_owned(&'a self) -> Ident {
        match self {
            Id::Prefixed(Cow::Owned(id)) => Ident::Prefixed(id.clone()),
            Id::Prefixed(Cow::Borrowed(id)) => Ident::Prefixed(<PrefixedId as crate::borrow::ToOwned>::to_owned(id)),
            Id::Unprefixed(Cow::Owned(id)) => Ident::Unprefixed(id.clone()),
            Id::Unprefixed(Cow::Borrowed(id)) => Ident::Unprefixed(<&UnprefixedId as crate::borrow::ToOwned>::to_owned(id)),
            Id::Url(Cow::Owned(url)) => Ident::Url(url.clone()),
            Id::Url(Cow::Borrowed(url)) => Ident::Url((*url).clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = Ident::from_str("http://purl.obolibrary.org/obo/po.owl").unwrap();
        let expected = Ident::Url(Url::parse("http://purl.obolibrary.org/obo/po.owl").unwrap());
        assert_eq!(actual, expected);

        let actual = Ident::from_str("GO:0046154").unwrap();
        let expected = Ident::Prefixed(PrefixedIdent::new(
            IdentPrefix::new(String::from("GO")),
            IdentLocal::new(String::from("0046154")),
        ));
        assert_eq!(actual, expected);

        let actual = Ident::from_str("goslim_plant").unwrap();
        let expected = Ident::Unprefixed(UnprefixedIdent::new(String::from("goslim_plant")));
        assert_eq!(actual, expected);
    }
}
