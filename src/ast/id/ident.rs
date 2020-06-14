use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::Rule;

use super::PrefixedIdent;
use super::UnprefixedIdent;
use super::Url;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Clone, Debug, PartialEq, Hash, Eq, Ord)]
pub enum Ident {
    Prefixed(PrefixedIdent),
    Unprefixed(UnprefixedIdent),
    Url(Url),
}

impl AsRef<Ident> for Ident {
  fn as_ref(&self) -> &Self {
    &self
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
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
    use crate::parser::FromSlice;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = Ident::from_str("http://purl.obolibrary.org/obo/po.owl").unwrap();
        let expected = Ident::Url(Url::parse("http://purl.obolibrary.org/obo/po.owl").unwrap());
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
        let lp = Ident::Prefixed(PrefixedIdent::new("GO", "001"));
        let rp = Ident::Prefixed(PrefixedIdent::new("GO", "002"));
        assert!(lp < rp);

        let lu = Ident::Unprefixed(UnprefixedIdent::new("has_part"));
        let ru = Ident::Unprefixed(UnprefixedIdent::new("part_of"));
        assert!(lu < ru);

        let lurl = Ident::Url(Url::parse("http://doi.org/").unwrap());
        let rurl = Ident::Url(Url::parse("http://nih.org").unwrap());
        assert!(lurl < rurl);

        assert!(lp < ru);
        assert!(lurl < ru);
    }
}
