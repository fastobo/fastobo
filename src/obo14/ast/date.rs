use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use chrono::DateTime;
use chrono::FixedOffset;

/// A naive date, as found in header frames.
pub struct NaiveDate {
    day: u8,
    month: u8,
    year: u16,
    hour: u8,
    minute: u8,
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

/// A comprehensive ISO-8601 date, as found in `creation_date` clauses.
pub struct IsoDate {
    inner: DateTime<FixedOffset>,
}
