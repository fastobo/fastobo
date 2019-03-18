use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use chrono::DateTime;
use chrono::FixedOffset;
use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use crate::error::Result;

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


}