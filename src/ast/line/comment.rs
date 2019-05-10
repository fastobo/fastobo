use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::DerefMut;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An inline comment without semantic value.
#[derive(Clone, Debug, Eq, Hash, PartialEq, OpaqueTypedef)]
#[opaque_typedef(allow_mut_ref)]
#[opaque_typedef(derive(
    AsRef(Inner, Self),
    AsMut(Inner, Self),
    Deref,
    DerefMut,
    Into(Inner),
    FromInner,
    PartialEq(Inner),
))]
pub struct Comment {
    value: String,
}

impl Comment {
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Comment { value: value.into() }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("! ").and(self.value.fmt(f)) // FIXME(@althonos): escape newlines
    }
}

impl<'i> FromPair<'i> for Comment {
    const RULE: Rule = Rule::HiddenComment;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        // FIXME(@althonos): Check for trailing spaces ?
        Ok(Comment::new(pair.as_str()[1..].trim().to_string()))
    }
}
impl_fromstr!(Comment);

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::string::ToString;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_str() {
        let comment = Comment::from_str("! something").unwrap();
        assert_eq!(comment, Comment::new("something"));
    }

    #[test]
    fn to_string() {
        let comment = Comment::new("something");
        assert_eq!(comment.to_string(), "! something");
    }
}
