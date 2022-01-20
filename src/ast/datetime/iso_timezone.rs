use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// An ISO-8601 timezone.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub enum IsoTimezone {
    Minus(u8, u8),
    Utc,
    Plus(u8, u8),
}

impl Display for IsoTimezone {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::IsoTimezone::*;
        match self {
            Utc => f.write_char('Z'),
            Plus(hh, mm) => write!(f, "+{:02}:{:02}", hh, mm),
            Minus(hh, mm) => write!(f, "-{:02}:{:02}", hh, mm),
        }
    }
}

impl<'i> FromPair<'i> for IsoTimezone {
    const RULE: Rule = Rule::Iso8601TimeZone;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iso8601TimeZoneUtc => Ok(IsoTimezone::Utc),
            Rule::Iso8601TimeZoneOffset => {
                let mut inner = inner.into_inner();
                let sign = inner.next().unwrap();
                let hh = inner.next().unwrap().as_str().parse::<u8>().unwrap();
                let mm = inner.next().unwrap().as_str().parse::<u8>().unwrap();
                match sign.as_str() {
                    "+" => Ok(IsoTimezone::Plus(hh, mm)),
                    "-" | "−" | "–" => Ok(IsoTimezone::Minus(hh, mm)),
                    s => unreachable!("unexpected sign in IsoTimezone::from_pair: {:?}", s),
                }
            }
            rule => unreachable!("unexpected rule in IsoTimezone::from_pair: {:?}", rule),
        }
    }
}
