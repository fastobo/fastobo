//! Parser for individual character classes.

use super::spacing::is_newline;
use super::spacing::is_whitespace;

/// An enum to differentiate between *escaped* and *unescaped* characters.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OboChar {
    /// An escaped character, e.g. `\:`
    Escaped(char),
    /// An unescaped character, e.g. `:`.
    Unescaped(char),
}

impl AsRef<char> for OboChar {
    fn as_ref(&self) -> &char {
        use self::OboChar::*;
        match self {
            Escaped(c) | Unescaped(c) => c,
        }
    }
}

impl From<char> for OboChar {
    fn from(c: char) -> Self {
        OboChar::Unescaped(c)
    }
}

// Helper to produce an unescaped string from a vector of `OboChar`.
impl<'a> ::std::iter::FromIterator<&'a OboChar> for String {
    fn from_iter<I: IntoIterator<Item = &'a OboChar>>(iter: I) -> String {
        let it = iter.into_iter();
        let mut s = match it.size_hint() {
            (_, Some(n)) => String::with_capacity(n),
            _ => String::new(),
        };
        for &c in it {
            s.push(*c.as_ref())
        }
        s
    }
}

/// Parser either a unicode character, or an escaped character.
pub fn obo_char(i: &str) -> nom::IResult<&str, OboChar> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    alt!(i, value!(OboChar::Escaped('\n'), tag!("\\n"))
         |  value!(OboChar::Escaped(' '), tag!("\\W"))
         |  value!(OboChar::Escaped('\t'), tag!("\\t"))
         |  value!(OboChar::Escaped(':'), tag!("\\:"))
         |  value!(OboChar::Escaped(','), tag!("\\,"))
         |  value!(OboChar::Escaped('"'), tag!("\\\""))
         |  value!(OboChar::Escaped('\\'), tag!("\\\\"))
         |  value!(OboChar::Escaped('('), tag!("\\("))
         |  value!(OboChar::Escaped(')'), tag!("\\)"))
         |  value!(OboChar::Escaped('['), tag!("\\["))
         |  value!(OboChar::Escaped(']'), tag!("\\]"))
         |  value!(OboChar::Escaped('}'), tag!("\\{"))
         |  value!(OboChar::Escaped('{'), tag!("\\}"))
         |  map!(
                verify!(nom::anychar, |c| !is_newline(c) && c != '\\'),
                |c| OboChar::Unescaped(c)
            )
    )
}

/// Parse a non-whitespace obo character.
pub fn nonws_char(i: &str) -> nom::IResult<&str, OboChar> {
    verify!(i, obo_char, |ref c| match c {
        OboChar::Unescaped(x) if is_whitespace(*x) => false,
        _ => true,
    })
}
