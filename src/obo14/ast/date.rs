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

/// A comprehensive ISO-8601 date, as found in `creation_date` clauses.
pub struct IsoDate {
    inner: DateTime<FixedOffset>,
}
