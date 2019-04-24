use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::IntoIterator;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::error::Error as PestError;
use pest::error::InputLocation;

use crate::ast::*;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A database cross-reference definition.
///
/// Cross-references can be used in `Def` or `Synonym` clauses of entity
/// frames to add sources for the provided definition or evidence to show the
/// actual existence of a synonym. They can also be found in `Xref` clauses
/// when the cross-reference is directly relevant to the annotated entity
/// (e.g. when exporting an ontology from a knowledge-base to add an hyperlink
/// to the original resource).
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Xref {
    pub id: Ident,
    pub desc: Option<QuotedString>,
}

impl Xref {

    /// Create a new `Xref` from the given ID, without description.
    pub fn new<I>(id: I) -> Self
    where
        I: Into<Ident>,
    {
        Self::with_desc(id, None)
    }

    /// Create a new `Xref` with the given ID and optional description.
    pub fn with_desc<I, D>(id: I, desc: D) -> Self
    where
        I: Into<Ident>,
        D: Into<Option<QuotedString>>,
    {
        Self {
            id: id.into(),
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
            let xref = Xref::from_str(inner.as_str())
                .map_err(|e| e.with_span(inner.as_span()))?;
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

    use pretty_assertions::assert_eq;
    use super::*;

    mod list {

        use super::*;

        #[test]
        fn from_str() {
            let actual = XrefList::from_str("[]").unwrap();
            let expected = XrefList::from(vec![]);
            self::assert_eq!(actual, expected);

            let actual = XrefList::from_str("[PSI:MS]").unwrap();
            let expected = XrefList::from(vec![Xref::new(PrefixedIdent::new("PSI", "MS"))]);
            self::assert_eq!(actual, expected);

            let actual = XrefList::from_str(
                "[PSI:MS, reactome:R-HSA-8983680 \"OAS1 produces oligoadenylates\"]",
            )
            .unwrap();
            let expected = XrefList::from(vec![
                Xref::new(PrefixedIdent::new("PSI", "MS")),
                Xref::with_desc(
                    PrefixedIdent::new("reactome", "R-HSA-8983680"),
                    QuotedString::new("OAS1 produces oligoadenylates"),
                ),
            ]);
            self::assert_eq!(actual, expected);
        }
    }
}
