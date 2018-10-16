//! Parsers for OBO header clauses.

pub mod clauses;

use super::ast;
use super::spacing::is_newline;
use super::spacing::is_whitespace;
use super::spacing::nl;
use super::spacing::ws;

/// Parse a header clause into the appropriate [`HeaderClause`] enum value.
pub fn header_clause(i: &str) -> nom::IResult<&str, ast::HeaderClause> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    alt_complete!(i,
            clauses::date
        |   clauses::format_version
        |   clauses::data_version
        |   clauses::saved_by
        |   clauses::auto_generated_by
        |   clauses::remark
        |   clauses::ontology

        // NB: MUST REMAIN LAST
        |   clauses::unreserved
    )
}

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
