use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A naive datetime, as found in header frames.
///
/// For historical reasons, OBO headers do not contain ISO datetimes but
/// *day-month-year* dates, which can be confusing for US-based users.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NaiveDateTime {
    day: u8,
    month: u8,
    year: u16,
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

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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
impl_fromstr!(NaiveDateTime);

/// A comprehensive ISO-8601 datetime, as found in `creation_date` clauses.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IsoDateTime {
    day: u8,
    month: u8,
    year: u16,
    hour: u8,
    minute: u8,
    second: u8,
    timezone: Option<IsoTimezone>,
}

impl Display for IsoDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:02}-{:02}-{:04}T{:02}:{:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute, self.second,
        )?;
        match self.timezone {
            Some(ref tz) => tz.fmt(f),
            None => Ok(()),
        }
    }
}

impl<'i> FromPair<'i> for IsoDateTime {
    const RULE: Rule = Rule::Iso8601DateTime;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let mut date = inner.next().unwrap().into_inner();
        let mut time = inner.next().unwrap().into_inner();

        let year = u16::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();
        let month = u8::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();
        let day = u8::from_str_radix(date.next().unwrap().as_str(), 10).unwrap();

        let hour = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();
        let minute = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();
        let second = u8::from_str_radix(time.next().unwrap().as_str(), 10).unwrap();

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
            timezone,
        })
    }
}
impl_fromstr!(IsoDateTime);

/// An ISO-8601 timezone.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum IsoTimezone {
    Utc,
    Plus(u8, u8),
    Minus(u8, u8),
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        use self::IsoTimezone::*;

        let tag = pair.as_str().chars().next().unwrap();
        if tag == 'Z' {
            return Ok(Utc);
        }

        let mut inner = pair.into_inner();
        let hh = u8::from_str_radix(inner.next().unwrap().as_str(), 10).unwrap();
        let mm = u8::from_str_radix(inner.next().unwrap().as_str(), 10).unwrap();

        match tag {
            '+' => Ok(Plus(hh, mm)),
            '-' => Ok(Minus(hh, mm)),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(IsoTimezone);

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use super::*;

    mod naive {

        use super::*;

        #[test]
        fn from_str() {
            let naive = NaiveDateTime::from_str("12:06:2018 17:13").unwrap();
            assert_eq!(naive, NaiveDateTime::new(12, 6, 2018, 17, 13));
        }

    }

    mod iso {

        use super::*;

        #[test]
        fn from_str() {
            match IsoDateTime::from_str("2017-1-24T14:41:36Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDateTime::from_str("2015-08-11T15:05:12Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDateTime::from_str("2016-10-26T10:51:48Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDateTime::from_str("2017-1-24T14:41:36Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDateTime::from_str("2017-1-24T14:41:36+01:30") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
        }

    }

}
