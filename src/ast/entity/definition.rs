use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// An entity textual definition, with optional cross-references supporting it.
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, PartialEq, PartialOrd)]
pub struct Definition {
    text: QuotedString,
    xrefs: XrefList,
}

impl Definition {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<QuotedString>,
    {
        Self::with_xrefs(text, XrefList::default())
    }

    pub fn with_xrefs<T, L>(text: T, xrefs: L) -> Self
    where
        T: Into<QuotedString>,
        L: Into<XrefList>,
    {
        Self {
            text: text.into(),
            xrefs: xrefs.into(),
        }
    }

    pub fn text(&self) -> &QuotedString {
        &self.text
    }

    pub fn text_mut(&mut self) -> &mut QuotedString {
        &mut self.text
    }

    pub fn xrefs(&self) -> &XrefList {
        &self.xrefs
    }

    pub fn xrefs_mut(&mut self) -> &mut XrefList {
        &mut self.xrefs
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.text
            .fmt(f)
            .and_then(|_| f.write_char(' '))
            .and_then(|_| self.xrefs.fmt(f))
    }
}

impl From<Definition> for QuotedString {
    fn from(d: Definition) -> Self {
        d.text
    }
}

impl From<QuotedString> for Definition {
    fn from(s: QuotedString) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for Definition {
    const RULE: Rule = Rule::Definition;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let text = QuotedString::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap(), cache)?;
        Ok(Self::with_xrefs(text, xrefs))
    }
}
