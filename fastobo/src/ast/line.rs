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

/// A line in an OBO file, possibly followed by qualifiers and a comment.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Line<T> {
    inner: T,
    qualifiers: Option<QualifierList>, // FIXME(@althonos): use an `IndexMap` ?
    comment: Option<Comment>,
}

impl<T> Line<T> {
    /// Update the line comment with the given one.
    pub fn and_comment<C>(self, comment: C) -> Self
    where
        C: Into<Option<Comment>>
    {
        Self {
            inner: self.inner,
            qualifiers: self.qualifiers,
            comment: comment.into(),
        }
    }

    /// Update the line qualifier list with the given one.
    pub fn and_qualifiers<Q>(self, qualifiers: Q) -> Self
    where
        Q: Into<Option<QualifierList>>
    {
        Self {
            inner: self.inner,
            qualifiers: qualifiers.into(),
            comment: self.comment,
        }
    }

    /// Get the actual OBO clause wrapped in the line.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> AsRef<T> for Line<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for Line<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Borrow<T> for Line<T> {
    fn borrow(&self) -> &T {
        &self.inner
    }
}

impl<T> BorrowMut<T> for Line<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.inner
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

impl<T> DerefMut for Line<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.inner
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

/// The optional part of a line, holding a qualifier list and a comment.
///
/// It can be used as a builder pattern to create a fully-fledged `Line`.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use std::str::FromStr;
/// # use fastobo::ast::*;
/// let line = Eol::with_comment(Comment::new("ENVO uses 8 digits identifiers"))
///     .and_inner(ClassIdent::from_str("ENVO:00000001").unwrap());
/// let frame = TermFrame::new(line);
/// assert_eq!(frame.to_string(),
/// "[Term]
/// id: ENVO:00000001 ! ENVO uses 8 digits identifiers
/// ");
/// ```
pub type Eol = Line<()>;

impl<'i> FromPair<'i> for Eol {
    const RULE: Rule = Rule::EOL;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let opt1 = inner.next();
        let opt2 = inner.next();
        match (opt1, opt2) {
            (Some(pair1), Some(pair2)) => {
                let comment = Comment::from_pair_unchecked(pair2)?;
                let qualifiers = QualifierList::from_pair_unchecked(pair1)?;
                Ok(Eol::with_qualifiers(qualifiers).and_comment(comment))
            }
            (Some(pair1), None) => match pair1.as_rule() {
                Rule::QualifierList => {
                    QualifierList::from_pair_unchecked(pair1).map(Eol::with_qualifiers)
                }
                Rule::HiddenComment => {
                    Comment::from_pair_unchecked(pair1).map(Eol::with_comment)
                }
                _ => unreachable!(),
            },
            (None, _) => Ok(Eol::new()),
        }
    }
}

impl Default for Eol {
    fn default() -> Self {
        Line {
            inner: (),
            qualifiers: None,
            comment: None,
        }
    }
}

impl Eol {

    // Create a new empty `Eol`.
    pub fn new() -> Self {
        Default::default()
    }

    // Create a new `Eol` with the given comment.
    pub fn with_comment(comment: Comment) -> Self {
        Self::new().and_comment(comment)
    }

    // Create a new `Eol` with the given qualifier list.
    pub fn with_qualifiers(qualifiers: QualifierList) -> Self {
        Self::new().and_qualifiers(qualifiers)
    }

    // Add content to the `Eol` to form a complete line.
    pub fn and_inner<T>(self, inner: T) -> Line<T> {
        Line {
            inner,
            qualifiers: self.qualifiers,
            comment: self.comment,
        }
    }
}

impl From<Comment> for Eol {
    fn from(comment: Comment) -> Self {
        Self::with_comment(comment)
    }
}

impl From<QualifierList> for Eol {
    fn from(qualifiers: QualifierList) -> Self {
        Self::with_qualifiers(qualifiers)
    }
}

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
        Ok(Comment::new(pair.as_str()[1..].to_string()))
    }
}
impl_fromstr!(Comment);
