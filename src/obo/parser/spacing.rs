//! Parsers for whitespace and newline characters.

/// Tests if character is whitespace: [`' '`], [`\t`]
///
/// Equivalent to the [**WhiteSpaceChar**] production rule in the OBO 1.4
/// syntax.
///
/// [`' '`]: https://en.wikipedia.org/wiki/Whitespace_character
/// [`\t`]: https://en.wikipedia.org/wiki/Tab_key#Tab_characters
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Tests if character is a newline: [`\r`], [`\n`], [`\f`]
///
/// Equivalent to the [**NewlineChar**] production rule in the OBO 1.4
/// syntax.
///
/// [`\r`]: https://en.wikipedia.org/wiki/Carriage_return
/// [`\n`]: https://en.wikipedia.org/wiki/Newline
/// [`\f`]: https://en.wikipedia.org/wiki/Page_break#Form_feed
pub fn is_newline(c: char) -> bool {
    c == '\r' || c == '\n' || c == '\u{00C}'
}

/// Parse one or more whitespace characters.
pub fn ws(i: &str) -> nom::IResult<&str, &str> {
    take_while1!(i, is_whitespace)
}

/// Parse a newline character preceded by zero or more whitespaces.
pub fn nl(i: &str) -> nom::IResult<&str, &str> {
    recognize!(
        i,
        terminated!(many0!(ws), verify!(nom::anychar, is_newline))
    )
}
