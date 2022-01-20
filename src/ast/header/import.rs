use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

// FIXME(@althonos): Ordering is not based on lexicographic order but will put
//                   Abbreviated before Url. This will probably look nicer
//                   but goes against the specification.
/// A reference to another document to be imported.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub enum Import {
    Abbreviated(Box<Ident>), // QUESTION(@althonos): IdentPrefix ?
    Url(Box<Url>),
}

impl Import {
    /// Convert an import clause value into an URL.
    ///
    /// If the import is already an URL reference, the underlying URL is simply returned. Otherwise,
    /// an URL is built using the default OBO prefix (`http://purl.obolibrary.org/obo/`).
    pub fn into_url(self) -> Url {
        match self {
            Import::Url(u) => *u,
            Import::Abbreviated(id) => {
                Url::from_str(&format!("http://purl.obolibrary.org/obo/{}.owl", id)).unwrap()
            }
        }
    }
}

impl From<Url> for Import {
    fn from(url: Url) -> Self {
        Import::Url(Box::new(url))
    }
}

impl From<Ident> for Import {
    fn from(id: Ident) -> Self {
        Import::Abbreviated(Box::new(id))
    }
}

impl From<Import> for Url {
    fn from(import: Import) -> Self {
        import.into_url()
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Import::*;
        match self {
            Url(url) => url.fmt(f),
            Abbreviated(id) => id.fmt(f),
        }
    }
}

impl<'i> FromPair<'i> for Import {
    const RULE: Rule = Rule::Import;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iri => Ok(Url::from_str(inner.as_str())?.into()), // FIXME
            Rule::Id => Ident::from_pair_unchecked(inner, cache).map(From::from),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn into_url() {
        let i = Import::from(Ident::from(UnprefixedIdent::new("go")));
        assert_eq!(
            i.into_url(),
            Url::from_str("http://purl.obolibrary.org/obo/go.owl").unwrap()
        );

        let i = Import::from(Url::from_str("http://ontologies.berkeleybop.org/ms.obo").unwrap());
        assert_eq!(
            i.into_url(),
            Url::from_str("http://ontologies.berkeleybop.org/ms.obo").unwrap()
        );
    }
}
