use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::DerefMut;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::Identified;
use crate::semantics::Orderable;
use crate::syntax::Lexer;
use crate::syntax::Rule;

/// A database cross-reference definition.
///
/// Cross-references can be used in [`Definition`] or [`Synonym`] clauses of entity
/// frames to add sources for the provided definition or evidence to show the
/// actual existence of a synonym. They can also be found in `Xref` clauses
/// when the cross-reference is directly relevant to the annotated entity
/// (e.g. when exporting an ontology from a knowledge-base to add an hyperlink
/// to the original resource).
///
/// [`Definition`]: ./struct.Definition.html
/// [`Synonym`]: ./struct.Synonym.html
#[derive(Clone, Debug, Hash, Eq, FromStr, Ord, PartialEq, PartialOrd)]
pub struct Xref {
    id: Ident,
    desc: Option<Box<QuotedString>>,
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
            desc: desc.into().map(Box::new),
        }
    }

    /// Get a mutable reference to the identifier of the xref.
    pub fn id(&self) -> &Ident {
        &self.id
    }

    /// Get a reference to the identifier of the xref.
    pub fn id_mut(&mut self) -> &mut Ident {
        &mut self.id
    }

    /// Get a reference to the description of the xref, if any.
    pub fn description(&self) -> Option<&QuotedString> {
        self.desc.as_deref()
    }

    /// Get a mutable reference to the description of the xref, if any.
    pub fn description_mut(&mut self) -> Option<&mut QuotedString> {
        self.desc.as_deref_mut()
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
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let id = FromPair::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let desc = match inner.next() {
            Some(pair) => Some(QuotedString::from_pair_unchecked(pair, cache).map(Box::new)?),
            None => None,
        };
        Ok(Xref { id, desc })
    }
}

impl From<Ident> for Xref {
    fn from(id: Ident) -> Self {
        Self::new(id)
    }
}

impl Identified for Xref {
    fn as_id(&self) -> &Ident {
        &self.id
    }
    fn as_id_mut(&mut self) -> &mut Ident {
        &mut self.id
    }
}

/// A list of containing zero or more `Xref`s.
#[derive(Clone, Default, Debug, Hash, FromStr, Eq, Ord, PartialOrd, PartialEq)]
pub struct XrefList {
    xrefs: Vec<Xref>,
}

impl XrefList {
    pub fn new(xrefs: Vec<Xref>) -> Self {
        Self { xrefs }
    }
}

impl AsMut<[Xref]> for XrefList {
    fn as_mut(&mut self) -> &mut [Xref] {
        &mut self.xrefs
    }
}

impl AsMut<Vec<Xref>> for XrefList {
    fn as_mut(&mut self) -> &mut Vec<Xref> {
        &mut self.xrefs
    }
}

impl AsRef<[Xref]> for XrefList {
    fn as_ref(&self) -> &[Xref] {
        &self.xrefs
    }
}

impl AsRef<Vec<Xref>> for XrefList {
    fn as_ref(&self) -> &Vec<Xref> {
        &self.xrefs
    }
}

impl Deref for XrefList {
    type Target = Vec<Xref>;
    fn deref(&self) -> &Self::Target {
        &self.xrefs
    }
}

impl DerefMut for XrefList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.xrefs
    }
}

impl Display for XrefList {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('[')?;
        let mut xrefs = self.xrefs.iter().peekable();
        while let Some(xref) = xrefs.next() {
            // FIXME(@althonos): commas in id need escaping.
            xref.id().fmt(f)?;
            if let Some(ref desc) = xref.description() {
                f.write_char(' ').and(desc.fmt(f))?;
            }
            if xrefs.peek().is_some() {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}

impl From<XrefList> for Vec<Xref> {
    fn from(list: XrefList) -> Self {
        list.xrefs
    }
}

impl From<Vec<Xref>> for XrefList {
    fn from(xrefs: Vec<Xref>) -> Self {
        Self { xrefs }
    }
}

impl FromIterator<Xref> for XrefList {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Xref>,
    {
        Self::new(iter.into_iter().collect())
    }
}

impl<'i> FromPair<'i> for XrefList {
    const RULE: Rule = Rule::XrefList;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut xrefs = Vec::new();
        for inner in pair.into_inner() {
            // We have to re-tokenize here because the identifier
            // rule would allow some identifiers that could break
            // the parsing of the xref list.
            match Lexer::tokenize(Rule::Xref, inner.as_str())
                .map_err(SyntaxError::from)
                .map(|mut pairs| pairs.next().unwrap())
                .and_then(|pair| Xref::from_pair(pair, cache))
            {
                Ok(xref) => xrefs.push(xref),
                Err(error) => return Err(error.with_span(inner.as_span())),
            }
        }
        Ok(Self { xrefs })
    }
}

impl IntoIterator for XrefList {
    type Item = Xref;
    type IntoIter = <Vec<Xref> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.xrefs.into_iter()
    }
}

impl Orderable for XrefList {
    fn sort(&mut self) {
        self.xrefs.sort_unstable();
    }
    fn is_sorted(&self) -> bool {
        for i in 1..self.xrefs.len() {
            if self.xrefs[i - 1] > self.xrefs[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    mod xref {

        use super::*;

        #[test]
        fn as_id() {
            let actual = Xref::from_str("PMID:26585518").unwrap();
            self::assert_eq!(
                actual.as_id(),
                &Ident::from(PrefixedIdent::new("PMID", "26585518"))
            );
        }

        #[test]
        fn new() {
            let actual = Xref::from_str("PMID:26585518").unwrap();
            let mut expected = Xref::new(PrefixedIdent::new("PMID", "26585518"));
            self::assert_eq!(actual, expected);
            assert!(expected.description().is_none());
            assert!(expected.description_mut().is_none());
        }

        #[test]
        fn with_desc() {
            let actual = Xref::from_str("PMID:26585518 \"OrthoANI paper\"").unwrap();
            let mut expected = Xref::with_desc(
                PrefixedIdent::new("PMID", "26585518"),
                QuotedString::from("OrthoANI paper"),
            );
            self::assert_eq!(actual, expected);
            assert!(expected.description().is_some());
            assert!(expected.description_mut().is_some());
        }

        #[test]
        fn display() {
            let repr1 = "PMID:26585518";
            let actual1 = Xref::from_str(repr1).unwrap();
            self::assert_eq!(actual1.to_string(), repr1);

            let repr2 = "PMID:26585518 \"OrthoANI paper\"";
            let actual2 = Xref::from_str(repr2).unwrap();
            self::assert_eq!(actual2.to_string(), repr2);
        }
    }

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

            let actual = XrefList::from_str(
                r#"[DOI:10.1086/522843 "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007"]"#
            ).unwrap();
            let expected = XrefList::from(vec![Xref::with_desc(
                PrefixedIdent::new("DOI", "10.1086/522843"),
                QuotedString::new(
                    "Gordon, Deborah. American Naturalist: Natural History Note. Dec. 2007",
                ),
            )]);
            self::assert_eq!(actual, expected);
        }
    }
}
