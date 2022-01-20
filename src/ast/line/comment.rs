use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// An inline comment without semantic value.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub struct Comment {
    value: StringType,
}

impl Comment {
    pub fn new<S>(value: S) -> Self
    where
        S: Into<StringType>,
    {
        Comment {
            value: value.into(),
        }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("! ").and(self.value.fmt(f)) // FIXME(@althonos): escape newlines
    }
}

impl<'i> FromPair<'i> for Comment {
    const RULE: Rule = Rule::Comment;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let txt = pair
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .trim()
            .to_string();
        Ok(Comment::new(txt))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use std::string::ToString;

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
