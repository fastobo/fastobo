use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use chrono::DateTime;
use chrono::FixedOffset;
use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use crate::error::Result;
use crate::error::Error;

/// A naive date, as found in header frames.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct NaiveDate {
    day: u8,
    month: u8,
    year: u16,
    hour: u8,
    minute: u8,
}

impl NaiveDate {
    // FIXME(@althonos): check for date
    pub fn new(day: u8, month: u8, year: u16, hour: u8, minute: u8) -> Self {
        NaiveDate {
            day, month, year, hour, minute
        }
    }
}

impl Display for NaiveDate {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{:02}:{:02}:{:04} {:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute
        )
    }
}

// FIXME(@althonos): ensure the date is somewhat realistic.
impl FromPair for NaiveDate {
    const RULE: Rule = Rule::NaiveDateTime;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let date = inner.next().unwrap();
        let time = inner.next().unwrap();

        let datestr = date.as_str();
        let timestr = time.as_str();

        Ok(NaiveDate {
            day: u8::from_str_radix(&datestr[..2], 10).unwrap(),
            month: u8::from_str_radix(&datestr[3..5], 10).unwrap(),
            year: u16::from_str_radix(&datestr[6..10], 10).unwrap(),
            hour: u8::from_str_radix(&timestr[..2], 10).unwrap(),
            minute: u8::from_str_radix(&timestr[3..5], 10).unwrap(),
        })
    }
}
impl_fromstr!(NaiveDate);

/// A comprehensive ISO-8601 date, as found in `creation_date` clauses.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct IsoDate {
    inner: DateTime<FixedOffset>,
}

impl AsRef<DateTime<FixedOffset>> for IsoDate {
    fn as_ref(&self) -> &DateTime<FixedOffset> {
        &self.inner
    }
}

impl From<DateTime<FixedOffset>> for IsoDate {
    fn from(dt: DateTime<FixedOffset>) -> Self {
        Self {
            inner: dt
        }
    }
}

impl Display for IsoDate {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.fmt(f)
    }
}

impl FromPair for IsoDate {
    const RULE: Rule = Rule::Iso8601DateTime;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // FIXME(@althonos): we could probably create the DateTime ourselves
        //                   using the tokenization from the Obo14 grammar.
        let dt = chrono::DateTime::from_str(pair.as_str()).unwrap();
        Ok(IsoDate::from(dt))
    }
}

impl FromStr for IsoDate {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let dt = chrono::DateTime::from_str(s).unwrap();
        Ok(IsoDate::from(dt))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    mod naive {

        use super::*;

        #[test]
        fn from_str() {
            let naive = NaiveDate::from_str("12:06:2018 17:13").unwrap();
            assert_eq!(naive, NaiveDate::new(12, 6, 2018, 17, 13));
        }

    }

    mod iso {

        use super::*;
        use chrono::Utc;
        use chrono::TimeZone;

        #[test]
        fn from_str() {
            match IsoDate::from_str("2017-1-24T14:41:36Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDate::from_str("2015-08-11T15:05:12Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDate::from_str("2016-10-26T10:51:48Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            match IsoDate::from_str("2017-1-24T14:41:36Z") {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
        }

    }


}
