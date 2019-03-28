use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use url::Url;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A synonym scope specifier.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum SynonymScope {
    Exact,
    Broad,
    Narrow,
    Related,
}

impl Display for SynonymScope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::SynonymScope::*;
        match self {
            Exact => f.write_str("EXACT"),
            Broad => f.write_str("BROAD"),
            Narrow => f.write_str("NARROW"),
            Related => f.write_str("RELATED"),
        }
    }
}

impl FromPair for SynonymScope {
    const RULE: Rule = Rule::SynonymScope;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        match pair.as_str() {
            "EXACT" => Ok(SynonymScope::Exact),
            "BROAD" => Ok(SynonymScope::Broad),
            "NARROW" => Ok(SynonymScope::Narrow),
            "RELATED" => Ok(SynonymScope::Related),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(SynonymScope);

/// A synonym, denoting an alternative name for the embedding entity.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Synonym {
    desc: QuotedString,
    scope: SynonymScope,
    ty: Option<SynonymTypeId>,
    xrefs: XrefList,
}

impl Synonym {
    pub fn new(desc: QuotedString, scope: SynonymScope, xrefs: XrefList) -> Self {
        Self {
            desc,
            scope,
            ty: None,
            xrefs,
        }
    }

    pub fn with_type(
        desc: QuotedString,
        scope: SynonymScope,
        ty: SynonymTypeId,
        xrefs: XrefList,
    ) -> Self {
        Self {
            desc,
            scope,
            ty: Some(ty),
            xrefs,
        }
    }
}

impl Display for Synonym {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.desc
            .fmt(f)
            .and(f.write_char(' '))
            .and(self.scope.fmt(f))
            .and(f.write_char(' '))?;

        if let Some(ref syntype) = self.ty {
            syntype.fmt(f).and(f.write_char(' '))?;
        }

        self.xrefs.fmt(f)
    }
}

impl FromPair for Synonym {
    const RULE: Rule = Rule::Synonym;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
        let scope = SynonymScope::from_pair_unchecked(inner.next().unwrap())?;

        let nxt = inner.next().unwrap();
        match nxt.as_rule() {
            Rule::SynonymTypeId => {
                let ty = Some(SynonymTypeId::from_pair_unchecked(nxt)?);
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(Synonym {
                    desc,
                    scope,
                    ty,
                    xrefs,
                })
            }
            Rule::XrefList => {
                let ty = None;
                let xrefs = XrefList::from_pair_unchecked(nxt)?;
                Ok(Synonym {
                    desc,
                    scope,
                    ty,
                    xrefs,
                })
            }
            _ => unreachable!(),
        }
    }
}
