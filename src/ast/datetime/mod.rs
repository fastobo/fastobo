mod iso_date;
mod iso_datetime;
mod iso_time;
mod iso_timezone;
mod naive_datetime;

pub use self::iso_date::IsoDate;
pub use self::iso_datetime::IsoDateTime;
pub use self::iso_time::IsoTime;
pub use self::iso_timezone::IsoTimezone;
pub use self::naive_datetime::NaiveDateTime;

/// A trait for common attributes of OBO times.
pub trait Date {
    /// Get the year component of the date.
    fn year(&self) -> u16;

    /// Get the month component of the date.
    fn month(&self) -> u8;

    /// Get the day component of the date.
    fn day(&self) -> u8;

    /// Generate an XML Schema date serialization of the `Date`.
    fn to_xsd_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year(), self.month(), self.day())
    }
}

/// A trait for common attributes of OBO times.
pub trait Time {
    fn hour(&self) -> u8;
    fn minute(&self) -> u8;
    fn second(&self) -> u8;
}

/// A trait for common operations on OBO datetimes.
pub trait DateTime: Date + Time {
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
