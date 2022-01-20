use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// The creation date of an OBO entity.
///
/// The OBO format version 1.4 specifies that the value of `creation_date`
/// clauses on entities should be that of an [ISO8601](https://en.wikipedia.org/wiki/ISO_8601)
/// formatted datetime. However, there are many cases in the wild where
/// OBO ontologies only providing a date instead of a datetime:
///
/// ```obo
/// [Typedef]
/// id: RO:0017001
/// name: utilizes
/// holds_over_chain: RO:0002215 RO:0002233
/// creation_date: 2021-11-08
/// ```
///
/// Since the guide is vague on this particular issue, we decided not to
/// make this a hard error, and instead to support incomplete dates, for
/// intercompatibility with older ontologies that are otherwise parsing fine.
///
#[derive(Clone, Debug, FromStr, Hash, PartialOrd, Ord, Eq, PartialEq)]
pub enum CreationDate {
    /// A creation date missing the time component of an ISO8601 datetime.
    Date(Box<IsoDate>),
    /// A creation date specified as a complete ISO8601 datetime string.
    DateTime(Box<IsoDateTime>),
}

impl AsRef<IsoDate> for CreationDate {
    fn as_ref(&self) -> &IsoDate {
        match self {
            CreationDate::Date(d) => d,
            CreationDate::DateTime(dt) => dt.date(),
        }
    }
}

impl Display for CreationDate {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            CreationDate::Date(d) => d.fmt(f),
            CreationDate::DateTime(d) => d.fmt(f),
        }
    }
}

impl Date for CreationDate {
    fn year(&self) -> u16 {
        <Self as AsRef<IsoDate>>::as_ref(self).year()
    }

    fn month(&self) -> u8 {
        <Self as AsRef<IsoDate>>::as_ref(self).month()
    }

    fn day(&self) -> u8 {
        <Self as AsRef<IsoDate>>::as_ref(self).day()
    }
}

impl From<Box<IsoDateTime>> for CreationDate {
    fn from(b: Box<IsoDateTime>) -> Self {
        CreationDate::DateTime(b)
    }
}

impl From<Box<IsoDate>> for CreationDate {
    fn from(b: Box<IsoDate>) -> Self {
        CreationDate::Date(b)
    }
}

impl From<IsoDate> for CreationDate {
    fn from(b: IsoDate) -> Self {
        Self::from(Box::new(b))
    }
}

impl From<IsoDateTime> for CreationDate {
    fn from(b: IsoDateTime) -> Self {
        Self::from(Box::new(b))
    }
}

impl<'i> FromPair<'i> for CreationDate {
    const RULE: Rule = Rule::CreationDate;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iso8601DateTime => IsoDateTime::from_pair(inner, cache).map(From::from),
            Rule::Iso8601Date => IsoDate::from_pair(inner, cache).map(From::from),
            rule => unreachable!("unexpected rule in CreationDate::from_pair: {:?}", rule),
        }
    }
}
