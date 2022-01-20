use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use super::Date;
use super::DateTime;
use super::Time;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A naive datetime, as found in header frames.
///
/// For historical reasons, OBO headers do not contain ISO datetimes but
/// *day-month-year* dates, which can be confusing for US-based users.
#[derive(Clone, Debug, Hash, Eq, FromStr, Ord, PartialEq, PartialOrd)]
pub struct NaiveDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl NaiveDateTime {
    // FIXME(@althonos): check for date
    pub fn new(day: u8, month: u8, year: u16, hour: u8, minute: u8) -> Self {
        NaiveDateTime {
            day,
            month,
            year,
            hour,
            minute,
        }
    }

    /// Change the date component of the `NaiveDateTime`.
    #[must_use]
    pub fn with_date(mut self, day: u8, month: u8, year: u16) -> Self {
        self.day = day;
        self.month = month;
        self.year = year;
        self
    }

    /// Change the time component of the `NaiveDateTime`.
    #[must_use]
    pub fn with_time(mut self, hour: u8, minute: u8) -> Self {
        self.hour = hour;
        self.minute = minute;
        self
    }
}

impl Date for NaiveDateTime {
    fn year(&self) -> u16 {
        self.year
    }

    fn month(&self) -> u8 {
        self.month
    }

    fn day(&self) -> u8 {
        self.day
    }
}

impl DateTime for NaiveDateTime {
    /// Generate an XML Schema datetime serialization of the `NaiveDateTime`.
    ///
    /// # Note
    /// While `NaiveDateTime` structs do not store seconds, the `xsd:dateTime`
    /// format requires all components to be present in the serialization, so
    /// the date is initialized with seconds set to `0`.
    fn to_xsd_datetime(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:00",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

impl Display for NaiveDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:02}:{:02}:{:04} {:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute
        )
    }
}

impl<'i> FromPair<'i> for NaiveDateTime {
    const RULE: Rule = Rule::NaiveDateTime;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let mut date = inner.next().unwrap().into_inner();
        let mut time = inner.next().unwrap().into_inner();
        Ok(NaiveDateTime {
            day: date.next().unwrap().as_str().parse::<u8>().unwrap(),
            month: date.next().unwrap().as_str().parse::<u8>().unwrap(),
            year: date.next().unwrap().as_str().parse::<u16>().unwrap(),
            hour: time.next().unwrap().as_str().parse::<u8>().unwrap(),
            minute: time.next().unwrap().as_str().parse::<u8>().unwrap(),
        })
    }
}

impl Time for NaiveDateTime {
    fn hour(&self) -> u8 {
        self.hour
    }

    fn minute(&self) -> u8 {
        self.minute
    }

    fn second(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let naive = NaiveDateTime::from_str("12:06:2018 17:13").unwrap();
        self::assert_eq!(naive, NaiveDateTime::new(12, 6, 2018, 17, 13));
    }
}
