use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A line in an OBO file, possibly followed by qualifiers and a comment.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Line<T> {
    inner: T,
    qualifiers: Option<QualifierList>, // FIXME(@althonos): use an `IndexMap` ?
    comment: Option<Comment>,
}

impl<T> AsRef<T> for Line<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> From<T> for Line<T> {
    fn from(inner: T) -> Self {
        Line {
            inner,
            qualifiers: None,
            comment: None,
        }
    }
}

impl<T> Deref for Line<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> Display for Line<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.fmt(f)?;

        if let Some(ref qualifiers) = self.qualifiers {
            f.write_char(' ').and(qualifiers.fmt(f))?;
        }

        if let Some(ref comment) = self.comment {
            f.write_char(' ').and(comment.fmt(f))?;
        }

        f.write_char('\n')
    }
}

pub type Eol = Line<()>;

impl FromPair for Line<()> {
    const RULE: Rule = Rule::EOL;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let opt1 = inner.next();
        let opt2 = inner.next();
        match (opt1, opt2) {
            (Some(pair1), Some(pair2)) => {
                let comment = Comment::from_pair_unchecked(pair2)?;
                let qualifiers = QualifierList::from_pair_unchecked(pair1)?;
                Ok(Line::new(qualifiers, comment))
            }
            (Some(pair1), None) => match pair1.as_rule() {
                Rule::QualifierList => {
                    QualifierList::from_pair_unchecked(pair1).map(Line::with_qualifiers)
                }
                Rule::HiddenComment => Comment::from_pair_unchecked(pair1).map(Line::with_comment),
                _ => unreachable!(),
            },
            (None, _) => Ok(Line {
                inner: (),
                qualifiers: None,
                comment: None,
            }),
        }
    }
}

impl Default for Line<()> {
    fn default() -> Self {
        Line {
            inner: (),
            qualifiers: None,
            comment: None,
        }
    }
}

impl Line<()> {
    pub fn new(qualifiers: QualifierList, comment: Comment) -> Self {
        Line {
            inner: (),
            qualifiers: Some(qualifiers),
            comment: Some(comment),
        }
    }

    pub fn with_comment(comment: Comment) -> Self {
        Line {
            inner: (),
            qualifiers: None,
            comment: Some(comment),
        }
    }

    pub fn with_qualifiers(qualifiers: QualifierList) -> Self {
        Line {
            inner: (),
            qualifiers: Some(qualifiers),
            comment: None,
        }
    }

    pub fn with_content<T>(self, content: T) -> Line<T> {
        Line {
            inner: content,
            qualifiers: self.qualifiers,
            comment: self.comment,
        }
    }
}

/// An inline comment without semantic value.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Comment {
    value: String,
}

impl Comment {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Comment { value: s.into() }
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("! ").and(self.value.fmt(f)) // FIXME(@althonos): escape newlines
    }
}

impl FromPair for Comment {
    const RULE: Rule = Rule::HiddenComment;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // FIXME(@althonos): Check for trailing spaces ?
        Ok(Comment::new(pair.as_str()[1..].to_string()))
    }
}
