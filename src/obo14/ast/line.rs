use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use super::Qualifier;

/// A line in an OBO file, possibly followed by qualifiers and a comment.
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

/// An inline comment without semantic value.
pub struct Comment {
    value: String,
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("! ").and(self.value.fmt(f))
    }
}
