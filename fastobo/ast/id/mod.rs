mod prefixed;
mod subclasses;
mod unprefixed;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use url::Url;

pub use self::prefixed::IdLocal;
pub use self::prefixed::IdPrefix;
pub use self::prefixed::PrefixedId;
pub use self::subclasses::ClassId;
pub use self::subclasses::InstanceId;
pub use self::subclasses::NamespaceId;
pub use self::subclasses::RelationId;
pub use self::subclasses::SubsetId;
pub use self::subclasses::SynonymTypeId;
pub use self::unprefixed::UnprefixedId;

use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Id {
    Prefixed(PrefixedId),
    Unprefixed(UnprefixedId),
    Url(Url),
}

impl From<PrefixedId> for Id {
    fn from(id: PrefixedId) -> Self {
        Id::Prefixed(id)
    }
}

impl From<UnprefixedId> for Id {
    fn from(id: UnprefixedId) -> Self {
        Id::Unprefixed(id)
    }
}

impl From<Url> for Id {
    fn from(url: Url) -> Self {
        Id::Url(url)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Id::*;
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl FromPair for Id {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedId::from_pair_unchecked(inner).map(From::from),
            Rule::UnprefixedId => UnprefixedId::from_pair_unchecked(inner).map(From::from),
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Ok(Id::Url(Url::parse(inner.as_str()).unwrap())),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Id);

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
