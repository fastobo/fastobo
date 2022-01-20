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

use super::Date;
use super::DateTime;
use super::IsoDate;
use super::IsoTime;
use super::IsoTimezone;
use super::Time;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A comprehensive ISO-8601 datetime.
#[derive(Clone, Debug, Hash, Eq, FromStr, PartialEq, Ord, PartialOrd)]
pub struct IsoDateTime {
    date: IsoDate,
    time: IsoTime,
}

impl IsoDateTime {
    /// Create a new `IsoDateTime`.
    pub fn new(date: IsoDate, time: IsoTime) -> Self {
        IsoDateTime { date, time }
    }

    // /// Change the timezone component of the `IsoDateTime`.
    // pub fn with_timezone<I>(mut self, tz: I) -> Self
    // where
    //     I: Into<Option<IsoTimezone>>,
    // {
    //     self.timezone = tz.into();
    //     self
    // }
    //
    // /// Change the date component of the `IsoDateTime`.
    // pub fn with_date(mut self, day: u8, month: u8, year: u16) -> Self {
    //     self.day = day;
    //     self.month = month;
    //     self.year = year;
    //     self
    // }
    //
    // /// Change the time component of the `IsoDateTime`.
    // pub fn with_time(mut self, hour: u8, minute: u8, second: u8) -> Self {
    //     self.hour = hour;
    //     self.minute = minute;
    //     self.second = second;
    //     self
    // }
    //
    // /// Get the fraction of the `IsoDateTime`.
    // pub fn fraction(&self) -> Option<f32> {
    //     self.fraction.as_ref().map(|f| f.0)
    // }
    //
    // /// Get the timezone of the `IsoDateTime`, if any.
    // pub fn timezone(&self) -> Option<&IsoTimezone> {
    //     self.timezone.as_ref()
    // }
}

impl Date for IsoDateTime {
    fn year(&self) -> u16 {
        self.date.year()
    }
    fn month(&self) -> u8 {
        self.date.month()
    }
    fn day(&self) -> u8 {
        self.date.day()
    }
}

impl Display for IsoDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl<'i> FromPair<'i> for IsoDateTime {
    const RULE: Rule = Rule::Iso8601DateTime;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let date = IsoDate::from_pair(inner.next().unwrap(), cache)?;
        let time = IsoTime::from_pair(inner.next().unwrap(), cache)?;
        Ok(Self::new(date, time))
    }
}

impl Time for IsoDateTime {
    fn hour(&self) -> u8 {
        self.time.hour()
    }
    fn minute(&self) -> u8 {
        self.time.minute()
    }
    fn second(&self) -> u8 {
        self.time.second()
    }
}

impl DateTime for IsoDateTime {
    /// Generate an XML Schema datetime serialization of the `IsoDateTime`.
    fn to_xsd_datetime(&self) -> String {
        let tz = match self.time.timezone() {
            None => String::new(),
            Some(dt) => dt.to_string(),
        };
        let fraction = match self.time.fraction() {
            None => String::new(),
            Some(frac) => format!("{:0.02}", frac)[1..].to_string(),
        };
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}{}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            fraction,
            tz,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    macro_rules! assert_date_to_xsd {
        ($x:expr) => {
            assert_date_to_xsd!($x, $x)
        };
        ($x:expr, $y:expr) => {
            match IsoDateTime::from_str($x) {
                Ok(x) => self::assert_eq!(x.to_xsd_datetime(), $y, "{}", x),
                Err(e) => panic!("{}", e),
            }
        };
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
