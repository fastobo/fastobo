mod prefixed;
mod subclasses;
mod unprefixed;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use url::Url;

pub use self::prefixed::IdLcl;
pub use self::prefixed::IdLocal;
pub use self::prefixed::IdPrefix;
pub use self::prefixed::IdPrf;
pub use self::prefixed::PrefixedId;
pub use self::prefixed::PrefixedIdentifier;

pub use self::subclasses::ClassId;
pub use self::subclasses::InstanceId;
pub use self::subclasses::NamespaceId;
pub use self::subclasses::RelationId;
pub use self::subclasses::SubsetId;
pub use self::subclasses::SynonymTypeId;

pub use self::unprefixed::UnprefixedId;
pub use self::unprefixed::UnprefixedIdentifier;

use crate::borrow::Cow;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

use self::Identifier::*;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Identifier {
    Prefixed(PrefixedIdentifier),
    Unprefixed(UnprefixedIdentifier),
    Url(Url),
}

impl From<PrefixedIdentifier> for Identifier {
    fn from(id: PrefixedIdentifier) -> Self {
        Identifier::Prefixed(id)
    }
}

impl From<UnprefixedIdentifier> for Identifier {
    fn from(id: UnprefixedIdentifier) -> Self {
        Identifier::Unprefixed(id)
    }
}

impl From<Url> for Identifier {
    fn from(url: Url) -> Self {
        Identifier::Url(url)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl<'i> FromPair<'i> for Identifier {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedIdentifier::from_pair_unchecked(inner).map(From::from),
            Rule::UnprefixedId => UnprefixedIdentifier::from_pair_unchecked(inner).map(From::from),
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Ok(Identifier::Url(Url::parse(inner.as_str()).unwrap())),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Identifier);

/// A borrowed `Identifier`.
pub enum Id<'a> {
    Prefixed(Cow<'a, PrefixedId<'a>>),
    Unprefixed(Cow<'a, &'a UnprefixedId>),
    Url(Cow<'a, &'a Url>),
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = Id::from_str("http://purl.obolibrary.org/obo/po.owl").unwrap();
        let expected = Id::Url(Url::parse("http://purl.obolibrary.org/obo/po.owl").unwrap());
        assert_eq!(actual, expected);

        let actual = Id::from_str("GO:0046154").unwrap();
        let expected = Id::Prefixed(PrefixedId::new(
            IdPrefix::new("GO"),
            IdLocal::new("0046154"),
        ));
        assert_eq!(actual, expected);

        let actual = Id::from_str("goslim_plant").unwrap();
        let expected = Id::Unprefixed(UnprefixedId::new("goslim_plant"));
        assert_eq!(actual, expected);
    }
}
