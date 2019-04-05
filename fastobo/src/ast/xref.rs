use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
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
    id: Ident,
    desc: Option<QuotedString>,
}

impl Xref {
    pub fn new(id: Ident) -> Self {
        Self { id, desc: None }
    }

    pub fn with_desc(id: Ident, desc: QuotedString) -> Self {
        Self {
            id,
            desc: Some(desc),
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
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct XrefList {
    xrefs: Vec<Xref>,
}

impl From<Vec<Xref>> for XrefList {
    fn from(v: Vec<Xref>) -> XrefList {
        Self { xrefs: v }
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
