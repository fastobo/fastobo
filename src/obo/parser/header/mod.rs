//! Parsers for OBO header clauses.

#[macro_use]
mod _macros;
mod clauses;

use super::ast;
use super::spacing::nl;

pub use self::clauses::header_clause;

/// Parse a header frame into an [`HeaderFrame`] struct.
pub fn header_frame(i: &str) -> nom::IResult<&str, ast::HeaderFrame> {
    many1!(i, complete!(terminated!(header_clause, nl)))
        .map(|(r, clauses)| (r, ast::HeaderFrame { clauses }))
}

#[cfg(test)]
mod tests {

    use chrono::naive::NaiveDate;
    use chrono::naive::NaiveDateTime;

    use super::*;

    #[test]
    fn header_frame() {
        let (rest, frame) = super::header_frame(concat!(
            "format-version: 1.2\n",
            "date: 20:09:2018 09:05\n",
            "saved-by: Gerhard Mayer\n",
        ))
        .expect("parsing failed");

        assert_eq!(rest, "");
        assert_eq!(
            frame,
            ast::HeaderFrame {
                clauses: vec!(
                    ast::HeaderClause::FormatVersion("1.2".to_string()),
                    ast::HeaderClause::Date(NaiveDate::from_ymd(2018, 9, 20).and_hms(9, 5, 0)),
                    ast::HeaderClause::SavedBy("Gerhard Mayer".to_string()),
                )
            }
        );
    }
}
