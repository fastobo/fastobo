//! Parser for clause values as quoted or unquoted strings.

use super::chars::obo_char;
use super::chars::OboChar;

/// Parse a boolean value.
pub fn boolean(i: &str) -> nom::IResult<&str, bool> {
    alt!(i, value!(true, tag!("true")) | value!(false, tag!("false")))
}

/// Parse a quoted string.
pub fn quoted(i: &str) -> nom::IResult<&str, &str> {
    delimited!(
        i,
        tag!("\""),
        recognize!(many0!(verify!(obo_char, |c| c != OboChar::Unescaped('"')))),
        tag!("\"")
    )
}

/// Parse an unquoted string.
pub fn unquoted(i: &str) -> nom::IResult<&str, &str> {
    recognize!(i, many0!(obo_char))
}

/// Parse a quoted string and unescape it.
pub fn quoted_unescape(i: &str) -> nom::IResult<&str, String> {
    delimited!(
        i,
        tag!("\""),
        many0!(verify!(obo_char, |c| c != OboChar::Unescaped('"'))),
        tag!("\"")
    )
    .map(|(r, v)| (r, v.iter().collect()))
}

/// Parse an unquoted string and unescape it.
pub fn unquoted_unescape(i: &str) -> nom::IResult<&str, String> {
    many0!(i, obo_char).map(|(r, v)| (r, v.iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quoted() {
        // no escape characters
        let (r, s) = quoted("\"abc \"\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "abc ");

        // escaped characters
        let (r, s) = quoted("\"\\\"abc\\n\"\n").expect("parser failed");
        assert_eq!(r, "\n");
        assert_eq!(s, "\\\"abc\\n");

        // unescaped newline
        assert!(quoted("\"abc\n\"").is_err());
    }

    #[test]
    fn test_quoted_unescape() {
        // no escape characters
        let (r, s) = quoted_unescape("\"abc \"\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "abc ");

        // escaped characters
        let (r, s) = quoted_unescape("\"\\\"abc\\n\"\n").expect("parser failed");
        assert_eq!(r, "\n");
        assert_eq!(s, "\"abc\n");

        // unescaped newline
        assert!(quoted("\"abc\n\"").is_err());
    }

    #[test]
    fn test_unquoted() {
        let (r, s) = unquoted("something else\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "something else");

        let (r, s) = unquoted("something\\nelse\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "something\\nelse");
    }

    #[test]
    fn test_unquoted_unescape() {
        let (r, s) = unquoted_unescape("something else\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "something else");

        let (r, s) = unquoted_unescape("something\\nelse\n").unwrap();
        assert_eq!(r, "\n");
        assert_eq!(s, "something\nelse");
    }
}
