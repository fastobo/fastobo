use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;

use fastobo_derive_internal::FromStr;
use ordered_float::OrderedFloat;
use pest::iterators::Pair;

use super::IsoTimezone;
use super::Time;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// An ISO-8601 time, with an optional timezone specifier.
#[derive(Clone, Debug, Hash, Eq, FromStr, PartialEq, Ord, PartialOrd)]
pub struct IsoTime {
    hour: u8,
    minute: u8,
    second: u8,
    fraction: Option<OrderedFloat<f32>>,
    timezone: Option<IsoTimezone>,
}

impl IsoTime {
    /// Create a new `IsoDate` with the given day, month and year.
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour,
            minute,
            second,
            fraction: None,
            timezone: None,
        }
    }

    /// Change the fraction component of the `IsoTime`.
    #[must_use]
    pub fn with_fraction<F>(mut self, fraction: F) -> Self
    where
        F: Into<Option<f32>>,
    {
        self.fraction = fraction.into().map(OrderedFloat::from);
        self
    }

    /// Change the timezone component of the `IsoTime`.
    #[must_use]
    pub fn with_timezone<TZ>(mut self, timezone: TZ) -> Self
    where
        TZ: Into<Option<IsoTimezone>>,
    {
        self.timezone = timezone.into();
        self
    }

    /// Get the fraction of the `IsoTime`, if any.
    pub fn fraction(&self) -> Option<f32> {
        self.fraction.as_ref().map(|f| f.0)
    }

    /// Get the timezone of the `IsoTime`, if any.
    pub fn timezone(&self) -> Option<&IsoTimezone> {
        self.timezone.as_ref()
    }
}

impl Time for IsoTime {
    fn hour(&self) -> u8 {
        self.hour
    }
    fn minute(&self) -> u8 {
        self.minute
    }
    fn second(&self) -> u8 {
        self.second
    }
}

impl Display for IsoTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second,)?;
        if let Some(ref frac) = self.fraction() {
            let n = format!("{:.02}", frac);
            f.write_str(&n[1..])?;
        }
        match self.timezone() {
            Some(tz) => tz.fmt(f),
            None => Ok(()),
        }
    }
}

impl<'i> FromPair<'i> for IsoTime {
    const RULE: Rule = Rule::Iso8601Time;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();

        let mut fraction = None;
        let mut timezone = None;
        let hour = inner.next().unwrap().as_str().parse::<u8>().unwrap();
        let minute = inner.next().unwrap().as_str().parse::<u8>().unwrap();
        let second = inner.next().unwrap().as_str().parse::<u8>().unwrap();

        if let Some(pair) = inner.next() {
            match pair.as_rule() {
                Rule::Iso8601Fraction => {
                    fraction = Some(f32::from_str(pair.as_str()).unwrap().into());
                }
                Rule::Iso8601TimeZone => {
                    timezone = IsoTimezone::from_pair_unchecked(pair, cache).map(Some)?;
                }
                _ => unreachable!(),
            }
        }

        if let Some(pair) = inner.next() {
            timezone = IsoTimezone::from_pair_unchecked(pair, cache).map(Some)?;
        }

        Ok(IsoTime {
            hour,
            minute,
            second,
            fraction,
            timezone,
        })
    }
}
