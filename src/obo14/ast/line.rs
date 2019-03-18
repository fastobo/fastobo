use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use pest::iterators::Pair;

use crate::error::Result;
use crate::obo14::parser::FromPair;
use crate::obo14::parser::Rule;
use super::Qualifier;

/// A line in an OBO file, possibly followed by qualifiers and a comment.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Line<T> {
    inner: T,
    qualifiers: Option<Vec<Qualifier>>, // FIXME(@althonos): use an `IndexMap` ?
    comment: Option<Comment>,
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
            f.write_str(" {")?;
            let mut quals = qualifiers.iter().peekable();
            while let Some(qual) = quals.next() {
                qual.fmt(f)?;
                if quals.peek().is_some() {
                    f.write_str(", ")?;
                }
            }
            f.write_char('}')?;
        }

        if let Some(ref comment) = self.comment {
            f.write_char(' ').and(comment.fmt(f))?;
        }

        Ok(())
    }
}

impl FromPair for Line<()> {
    const RULE: Rule = Rule::EOL;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let opt1 = inner.next();
        let opt2 = inner.next();
        if let Some(pair2) = opt2 {
            let comment = Comment::from_pair_unchecked(pair2)?;
            // let qualifier = QualifierList::from_pair_unchecked(pair1)?;
            unimplemented!()
        } else if opt1.is_none() {
            Ok(Line {
                inner: (),
                qualifiers: None,
                comment: None,
            })
        } else {
            let pair1 = opt1.unwrap();
            unimplemented!()
        }
    }
}

impl Line<()> {
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

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("! ").and(self.value.fmt(f))
    }
}
