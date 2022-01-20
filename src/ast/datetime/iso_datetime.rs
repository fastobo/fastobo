use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use super::Date;
use super::DateTime;
use super::IsoDate;
use super::IsoTime;
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

    // Get the date component of the `IsoDateTime`.
    pub fn date(&self) -> &IsoDate {
        &self.date
    }

    // Get the date component of the `IsoDateTime`.
    pub fn time(&self) -> &IsoTime {
        &self.time
    }

    /// Change the date component of the `IsoDateTime`.
    #[must_use]
    pub fn with_date(mut self, date: IsoDate) -> Self {
        self.date = date;
        self
    }

    /// Change the time component of the `IsoDateTime`.
    #[must_use]
    pub fn with_time(mut self, time: IsoTime) -> Self {
        self.time = time;
        self
    }
}

impl AsRef<IsoDate> for IsoDateTime {
    fn as_ref(&self) -> &IsoDate {
        self.date()
    }
}

impl AsMut<IsoDate> for IsoDateTime {
    fn as_mut(&mut self) -> &mut IsoDate {
        &mut self.date
    }
}

impl AsRef<IsoTime> for IsoDateTime {
    fn as_ref(&self) -> &IsoTime {
        self.time()
    }
}

impl AsMut<IsoTime> for IsoDateTime {
    fn as_mut(&mut self) -> &mut IsoTime {
        &mut self.time
    }
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
