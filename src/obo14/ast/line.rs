use super::Qualifier;

/// A line in an OBO file, possibly followed by qualifiers and a comment.
pub struct Line<T> {
    inner: T,
    qualifiers: Vec<Qualifier>,  // FIXME(@althonos): use an `IndexMap` ?
    comment: Option<Comment>,
}

impl<T> From<T> for Line<T> {
    fn from(inner: T) -> Line<T> {
        Line {
            inner,
            qualifiers: Vec::new(),
            comment: None,
        }
    }
}

/// An inline comment without semantic value.
pub struct Comment {
    value: String,
}
