use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::IntoIterator;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A database cross-reference definition.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Xref {
    pub id: Ident,
    pub desc: Option<QuotedString>,
}

impl Xref {
    pub fn new(id: Ident) -> Self {
        Self { id, desc: None }
    }

    pub fn with_desc<D>(id: Ident, desc: D) -> Self
    where
        D: Into<Option<QuotedString>>,
    {
        Self {
            id,
            desc: desc.into(),
        }
    }
}

impl Display for Xref {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.id.fmt(f)?;
        match &self.desc {
            Some(desc) => f.write_char(' ').and(desc.fmt(f)),
            None => Ok(()),
        }
    }
}

impl<'i> FromPair<'i> for Xref {
    const RULE: Rule = Rule::Xref;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let id = FromPair::from_pair_unchecked(inner.next().unwrap())?;
        let desc = match inner.next() {
            Some(pair) => Some(QuotedString::from_pair_unchecked(pair)?),
            None => None,
        };
        Ok(Xref { id, desc })
    }
}
impl_fromstr!(Xref);

/// A list of containing zero or more `Xref`s.
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq, OpaqueTypedef)]
#[opaque_typedef(allow_mut_ref)]
#[opaque_typedef(derive(
    AsRef(Inner, Self),
    AsMut(Inner, Self),
    Deref,
    DerefMut,
    Into(Inner),
    FromInner,
    PartialEq(Inner),
))]
pub struct XrefList {
    xrefs: Vec<Xref>,
}

impl XrefList {
    pub fn new(xrefs: Vec<Xref>) -> Self {
        Self { xrefs }
    }
}

impl AsRef<[Xref]> for XrefList {
    fn as_ref(&self) -> &[Xref] {
        &self.xrefs
    }
}

impl Display for XrefList {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('[')?;
        let mut xrefs = self.xrefs.iter().peekable();
        while let Some(xref) = xrefs.next() {
            // FIXME(@althonos): commas in id need escaping.
            xref.id.fmt(f)?;
            if let Some(ref desc) = xref.desc {
                f.write_char(' ').and(desc.fmt(f))?;
            }
            if xrefs.peek().is_some() {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}

impl FromIterator<Xref> for XrefList {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Xref>
    {
        Self::new(iter.into_iter().collect())
    }
}

impl<'i> FromPair<'i> for XrefList {
    const RULE: Rule = Rule::XrefList;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut xrefs = Vec::new();
        for inner in pair.into_inner() {
            // FIXME(@althonos): avoid using FromStr: maybe duplicate rules ?
            let xref = Xref::from_str(inner.as_str())?;
            xrefs.push(xref);
        }
        Ok(Self { xrefs })
    }
}
impl_fromstr!(XrefList);

impl IntoIterator for XrefList {
    type Item = Xref;
    type IntoIter = <Vec<Xref> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.xrefs.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod list {

        use super::*;
        use crate::ast::IdLocal;
        use crate::ast::IdPrefix;
        use crate::ast::PrefixedId;

        #[test]
        fn from_str() {
            let actual = XrefList::from_str("[]").unwrap();
            let expected = XrefList::from(vec![]);
            assert_eq!(actual, expected);

            let actual = XrefList::from_str("[PSI:MS]").unwrap();
            let expected = XrefList::from(vec![Xref::new(
                PrefixedIdent::new(IdentPrefix::new(String::from("PSI")), IdentLocal::new(String::from("MS"))).into(),
            )]);
            assert_eq!(actual, expected);

            let actual = XrefList::from_str(
                "[PSI:MS, reactome:R-HSA-8983680 \"OAS1 produces oligoadenylates\"]",
            )
            .unwrap();
            let expected = XrefList::from(vec![
                Xref::new(PrefixedIdent::new(IdentPrefix::new(String::from("PSI")), IdentLocal::new(String::from("MS"))).into()),
                Xref::with_desc(
                    PrefixedIdent::new(IdentPrefix::new(String::from("reactome")), IdentLocal::new(String::from("R-HSA-8983680")))
                        .into(),
                    QuotedString::new(String::from("OAS1 produces oligoadenylates")),
                ),
            ]);
            assert_eq!(actual, expected);
        }
    }
}
