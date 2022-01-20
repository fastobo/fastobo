use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use super::Date;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A comprehensive ISO-8601 date.
#[derive(Clone, Debug, Hash, Eq, FromStr, PartialEq, Ord, PartialOrd)]
pub struct IsoDate {
    year: u16,
    month: u8,
    day: u8,
}

impl IsoDate {
    /// Create a new `IsoDate` with the given day, month and year.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        IsoDate { day, month, year }
    }
}

impl Date for IsoDate {
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

impl Display for IsoDate {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day,)
    }
}

impl<'i> FromPair<'i> for IsoDate {
    const RULE: Rule = Rule::Iso8601Date;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let year = inner.next().unwrap().as_str().parse::<u16>().unwrap();
        let month = inner.next().unwrap().as_str().parse::<u8>().unwrap();
        let day = inner.next().unwrap().as_str().parse::<u8>().unwrap();
        Ok(IsoDate::new(year, month, day))
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
            match IsoDate::from_str($x) {
                Ok(x) => self::assert_eq!(x.to_xsd_date(), $y, "{}", x),
                Err(e) => panic!("{}", e),
            }
        };
    }

    #[test]
    fn from_str() {
        assert_date_to_xsd!("2015-08-11");
        assert_date_to_xsd!("2016-10-26");
        assert_date_to_xsd!("2016-10-3", "2016-10-03");
        assert_date_to_xsd!("2017-1-24", "2017-01-24");
    }
}
