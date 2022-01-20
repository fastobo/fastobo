use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

use super::PrefixedIdent;
use super::UnprefixedIdent;
use super::Url;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, FromStr, Hash, Eq, Ord)]
pub enum Ident {
    Prefixed(Box<PrefixedIdent>),
    Unprefixed(Box<UnprefixedIdent>),
    Url(Box<Url>),
}

impl AsRef<Ident> for Ident {
    fn as_ref(&self) -> &Self {
        self
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
        Ident::Prefixed(Box::new(id))
    }
}

impl From<UnprefixedIdent> for Ident {
    fn from(id: UnprefixedIdent) -> Self {
        Ident::Unprefixed(Box::new(id))
    }
}

impl From<Url> for Ident {
    fn from(url: Url) -> Self {
        Ident::Url(Box::new(url))
    }
}

impl<'i> FromPair<'i> for Ident {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedIdent::from_pair_unchecked(inner, cache).map(From::from),
            Rule::UnprefixedId => {
                UnprefixedIdent::from_pair_unchecked(inner, cache).map(From::from)
            }
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Url::from_pair_unchecked(inner, cache).map(From::from),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use self::Ident::*;
        match (self, other) {
            (Prefixed(l), Prefixed(r)) => l.partial_cmp(r),
            (Unprefixed(l), Unprefixed(r)) => l.partial_cmp(r),
            (Url(l), Url(r)) => l.partial_cmp(r),
            (l, r) => l.to_string().partial_cmp(&r.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let actual = Ident::from_str("http://purl.obolibrary.org/obo/po.owl")
            .map(Ident::from)
            .unwrap();
        let expected = Url::from_str("http://purl.obolibrary.org/obo/po.owl")
            .map(Ident::from)
            .unwrap();
        assert_eq!(actual, expected);

        let actual = Ident::from_str("GO:0046154").unwrap();
        let expected = Ident::from(PrefixedIdent::new("GO", "0046154"));
        assert_eq!(actual, expected);

        let actual = Ident::from_str("goslim_plant").unwrap();
        let expected = Ident::from(UnprefixedIdent::new("goslim_plant"));
        assert_eq!(actual, expected);

        let actual = Ident::from_str(r#"PDBeChem:Copper(II)\ chloride"#).unwrap();
        let expected = Ident::from(PrefixedIdent::new("PDBeChem", "Copper(II) chloride"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn partial_cmp() {
        let lp = Ident::from(PrefixedIdent::new("GO", "001"));
        let rp = Ident::from(PrefixedIdent::new("GO", "002"));
        assert!(lp < rp);

        let lu = Ident::from(UnprefixedIdent::new("has_part"));
        let ru = Ident::from(UnprefixedIdent::new("part_of"));
        assert!(lu < ru);

        let lurl = Url::from_str("http://doi.org/").map(Ident::from).unwrap();
        let rurl = Url::from_str("http://nih.org").map(Ident::from).unwrap();
        assert!(lurl < rurl);

        assert!(lp < ru);
        assert!(lurl < ru);
    }
}
