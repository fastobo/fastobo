use std::cmp::Ord;

use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use fastobo_derive_internal::FromStr;
use ordered_float::OrderedFloat;
use pest::iterators::Pair;


use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A trait for common operations on OBO datetimes.
pub trait DateTime {
    /// Generate an XML Schema datetime serialization of the `DateTime`.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let dt = NaiveDateTime::new(8, 5, 2019, 13, 2);
    /// assert_eq!(dt.to_xsd_datetime(), "2019-05-08T13:02:00");
    /// ```
    fn to_xsd_datetime(&self) -> String;
}

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
    pub fn with_date(mut self, day: u8, month: u8, year: u16) -> Self {
        self.day = day;
        self.month = month;
        self.year = year;
        self
    }

    /// Change the time component of the `NaiveDateTime`.
    pub fn with_time(mut self, hour: u8, minute: u8) -> Self {
        self.hour = hour;
        self.minute = minute;
        self
    }

    /// Get the day of the `NaiveDateTime`.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Get the month of the `NaiveDateTime`.
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Get the year of the `NaiveDateTime`.
    pub fn year(&self) -> u16 {
        self.year
    }

    /// Get the hour of the `NaiveDateTime`.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute of the `NaiveDateTime`.
    pub fn minute(&self) -> u8 {
        self.minute
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let date = inner.next().unwrap();
        let time = inner.next().unwrap();

        let datestr = date.as_str();
        let timestr = time.as_str();

        Ok(NaiveDateTime {
            day: u8::from_str_radix(&datestr[..2], 10).unwrap(),
            month: u8::from_str_radix(&datestr[3..5], 10).unwrap(),
            year: u16::from_str_radix(&datestr[6..10], 10).unwrap(),
            hour: u8::from_str_radix(&timestr[..2], 10).unwrap(),
            minute: u8::from_str_radix(&timestr[3..5], 10).unwrap(),
        })
    }
}

/// A comprehensive ISO-8601 datetime, as found in `creation_date` clauses.
#[derive(Clone, Debug, Hash, Eq, FromStr, PartialEq, Ord, PartialOrd)]
pub struct IsoDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    fraction: Option<OrderedFloat<f32>>,
    timezone: Option<IsoTimezone>,
}

impl IsoDateTime {
    // FIXME(@althonos): check for date
    /// Create a new `IsoDateTime` without a timezone.
    pub fn new(day: u8, month: u8, year: u16, hour: u8, minute: u8, second: u8) -> Self {
        IsoDateTime {
            day,
            month,
            year,
            hour,
            minute,
            second,
            fraction: None,
            timezone: None,
        }
    }

    /// Change the timezone component of the `IsoDateTime`.
    pub fn with_timezone<I>(mut self, tz: I) -> Self
    where
        I: Into<Option<IsoTimezone>>,
    {
        self.timezone = tz.into();
        self
    }

    /// Change the date component of the `IsoDateTime`.
    pub fn with_date(mut self, day: u8, month: u8, year: u16) -> Self {
        self.day = day;
        self.month = month;
        self.year = year;
        self
    }

    /// Change the time component of the `IsoDateTime`.
    pub fn with_time(mut self, hour: u8, minute: u8, second: u8) -> Self {
        self.hour = hour;
        self.minute = minute;
        self.second = second;
        self
    }

    /// Get the day of the `IsoDateTime`.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Get the month of the `IsoDateTime`.
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Get the year of the `IsoDateTime`.
    pub fn year(&self) -> u16 {
        self.year
    }

    /// Get the hour of the `IsoDateTime`.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Get the minute of the `IsoDateTime`.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Get the second of the `IsoDateTime`.
    pub fn second(&self) -> u8 {
        self.second
    }

    /// Get the fraction of the `IsoDateTime`.
    pub fn fraction(&self) -> Option<f32> {
        self.fraction.as_ref().map(|f| f.0)
    }

    /// Get the timezone of the `IsoDateTime`, if any.
    pub fn timezone(&self) -> Option<&IsoTimezone> {
        self.timezone.as_ref()
    }
}

impl Display for IsoDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second,
        )?;
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

impl<'i> FromPair<'i> for IsoDateTime {
    const RULE: Rule = Rule::Iso8601DateTime;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let mut date = inner.next().unwrap().into_inner();
        let mut time = inner.next().unwrap().into_inner();

        let year = u16::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();
        let month = u8::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();
        let day = u8::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();

        let hour = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();
        let minute = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();
        let second = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();
        let fraction = time.next().map(|p| f32::from_str(p.as_str()).unwrap().into());

        let timezone = match inner.next() {
            Some(pair) => Some(IsoTimezone::from_pair_unchecked(pair)?),
            None => None,
        };

        Ok(IsoDateTime {
            day,
            month,
            year,
            hour,
            minute,
            second,
            fraction,
            timezone,
        })
    }
}

/// An ISO-8601 timezone.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub enum IsoTimezone {
    Minus(u8, Option<u8>),
    Utc,
    Plus(u8, Option<u8>),
}

impl DateTime for IsoDateTime {
    /// Generate an XML Schema datetime serialization of the `IsoDateTime`.
    fn to_xsd_datetime(&self) -> String {
        let tz = match self.timezone() {
            None => String::new(),
            Some(dt) => dt.to_string(),
        };
        let fraction = match self.fraction() {
            None => String::new(),
            Some(frac) => format!("{:0.02}", frac)[1..].to_string(),
        };
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, fraction, tz,
        )
    }
}

impl Display for IsoTimezone {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::IsoTimezone::*;
        match self {
            Utc => f.write_char('Z'),
            Plus(hh, Some(mm)) => write!(f, "+{:02}:{:02}", hh, mm),
            Minus(hh, Some(mm)) => write!(f, "-{:02}:{:02}", hh, mm),
            Plus(hh, None) => write!(f, "-{:02}", hh),
            Minus(hh, None) => write!(f, "-{:02}", hh),
        }
    }
}

impl<'i> FromPair<'i> for IsoTimezone {
    const RULE: Rule = Rule::Iso8601TimeZone;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        use self::IsoTimezone::*;

        let tag = pair.as_str().chars().next().unwrap();
        if tag == 'Z' {
            return Ok(Utc);
        }

        let mut inner = pair.into_inner();
        let hh = u8::from_str_radix(inner.next().unwrap().as_str(), 10).unwrap();
        let mm = inner.next().map(|p| u8::from_str_radix(p.as_str(), 10).unwrap());

        match tag {
            '+' => Ok(Plus(hh, mm)),
            '-' => Ok(Minus(hh, mm)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    mod naive {

        use super::*;

        #[test]
        fn from_str() {
            let naive = NaiveDateTime::from_str("12:06:2018 17:13").unwrap();
            self::assert_eq!(naive, NaiveDateTime::new(12, 6, 2018, 17, 13));
        }

    }

    mod iso {

        use super::*;

        macro_rules! assert_date_to_xsd {
            ($x:expr) => {assert_date_to_xsd!($x, $x)};
            ($x:expr, $y:expr) => {
                match IsoDateTime::from_str($x) {
                    Ok(x) => self::assert_eq!(x.to_xsd_datetime(), $y, "{}", x),
                    Err(e) => panic!("{}", e),
                }
            }
        }

        #[test]
        fn from_str() {
            assert_date_to_xsd!("2017-1-24T14:41:36Z", "2017-01-24T14:41:36Z");
            assert_date_to_xsd!("2015-08-11T15:05:12Z");
            assert_date_to_xsd!("2016-10-26T10:51:48Z");
            assert_date_to_xsd!("2017-1-24T14:41:36Z", "2017-01-24T14:41:36Z");
            assert_date_to_xsd!("2017-1-24T14:41:36.05Z", "2017-01-24T14:41:36.05Z");
            assert_date_to_xsd!("2017-1-24T14:41:36+01:30", "2017-01-24T14:41:36+01:30");
        }

    }

}
