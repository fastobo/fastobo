use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FrameReader;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::share::Cow;
use crate::share::Redeem;
use crate::share::Share;

/// A complete OBO document in format version 1.4.
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct OboDoc {
    pub(crate) header: HeaderFrame,
    pub(crate) entities: Vec<EntityFrame>,
}

/// Constructors and builder methods.
///
/// # Parser
/// Use `from_file` to parse a file on the local filesystem, or `from_stream`
/// to parse a `BufRead` implementor (`BufRead` is needed instead of `Read` as
/// the parser is line-based):
/// ```rust
/// # extern crate fastobo;
/// # use std::io::BufReader;
/// # use std::fs::File;
/// # use fastobo::ast::*;
/// let doc1 = OboDoc::from_file("tests/data/ms.obo").unwrap();
///
/// // This is equivalent to (but with the file path set in eventual errors):
/// let mut r = BufReader::new(File::open("tests/data/ms.obo").unwrap());
/// let doc2 = OboDoc::from_stream(&mut r).unwrap();
///
/// assert_eq!(doc1, doc2);
/// ```
///
/// # Builder Pattern
/// The builder pattern makes it easy to create an `OboDoc` from an interator
/// of `EntityFrame`, in order to add an `HeaderFrame` after all the entities
/// where collected:
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::*;
/// use std::iter::FromIterator;
///
/// let entities = vec![TermFrame::new(ClassIdent::from(PrefixedIdent::new("TEST", "001")))];
/// let doc = OboDoc::from_iter(entities.into_iter())
///     .and_header(HeaderFrame::from(HeaderClause::FormatVersion("1.4".into())));
/// ```
impl OboDoc {
    /// Create a new empty OBO document.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new OBO document with the provided frame.
    pub fn with_header(header: HeaderFrame) -> Self {
        Self {
            header,
            entities: Default::default(),
        }
    }

    /// Use the provided frame as the header of the OBO document.
    pub fn and_header(mut self, header: HeaderFrame) -> Self {
        self.header = header;
        self
    }

    /// Create a new OBO document with the provided entity frames.
    pub fn with_entities(entities: Vec<EntityFrame>) -> Self {
        Self {
            header: Default::default(),
            entities,
        }
    }

    /// Use the provided entity frames as the content of the OBO document.
    pub fn and_entities(mut self, entities: Vec<EntityFrame>) -> Self {
        self.entities = entities;
        self
    }

    /// Consume a buffered stream containing an OBO document into an AST.
    pub fn from_stream<B>(stream: &mut B) -> Result<Self, Error>
    where
        B: BufRead,
    {
        let mut reader = FrameReader::new(stream)?;
        Self::try_from(reader)
    }

    /// Read an OBO file located somwhere in the filesystem.
    pub fn from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let pathref: &Path = path.as_ref();
        File::open(pathref)
            .map_err(Error::from)
            .and_then(|f| Self::from_stream(&mut BufReader::new(f)))
            .map_err(|e| {
                if let Error::SyntaxError { error } = e {
                    error.with_path(&pathref.to_string_lossy()).into()
                } else {
                    e
                }
            })
    }

    /// Get a reference to the header of the OBO document.
    pub fn header(&self) -> &HeaderFrame {
        &self.header
    }

    /// Get a mutable reference to the header of the OBO document.
    pub fn header_mut(&mut self) -> &mut HeaderFrame {
        &mut self.header
    }

    /// Get a reference to the entities of the OBO document.
    pub fn entities(&self) -> &Vec<EntityFrame> {
        &self.entities
    }

    /// Get a reference to the entities of the OBO document.
    pub fn entities_mut(&mut self) -> &mut Vec<EntityFrame> {
        &mut self.entities
    }
}

impl AsRef<[EntityFrame]> for OboDoc {
    fn as_ref(&self) -> &[EntityFrame] {
        self.entities.as_slice()
    }
}

impl AsRef<Vec<EntityFrame>> for OboDoc {
    fn as_ref(&self) -> &Vec<EntityFrame> {
        &self.entities
    }
}

impl AsMut<Vec<EntityFrame>> for OboDoc {
    fn as_mut(&mut self) -> &mut Vec<EntityFrame> {
        &mut self.entities
    }
}

#[cfg(feature = "display")]
impl Display for OboDoc {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.header.fmt(f)?;
        if !self.header.is_empty() && !self.entities.is_empty() {
            f.write_char('\n')?;
        }

        let mut entities = self.entities.iter().peekable();
        while let Some(entity) = entities.next() {
            entity.fmt(f)?;
            if entities.peek().is_some() {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl<E> FromIterator<E> for OboDoc
where
    E: Into<EntityFrame>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = E>,
    {
        Self::with_entities(iter.into_iter().map(Into::into).collect())
    }
}

impl Orderable for OboDoc {
    /// Sort the document in the right serialization order.
    fn sort(&mut self) {
        self.header.sort_unstable();
        // FIXME(@althonos): should probably not require cloning here.
        self.entities.sort_unstable_by_key(|e| e.as_id().clone());
        for entity in &mut self.entities {
            entity.sort()
        }
    }

    /// Check if the document is sorted in the right serialization order.
    fn is_sorted(&self) -> bool {
        // Check entities are sorted on their identifier.
        for i in 1..self.entities.len() {
            if self.entities[i - 1].as_id() > self.entities[i].as_id() {
                return false;
            }
        }

        // Check every entity is sorted.
        for entity in &self.entities {
            if !entity.is_sorted() {
                return false;
            }
        }

        // Check the header is sorted.
        self.header.is_sorted()
    }
}

impl<'i> FromPair<'i> for OboDoc {
    const RULE: Rule = Rule::OboDoc;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();

        let mut entities = Vec::new();
        let header = HeaderFrame::from_pair_unchecked(inner.next().unwrap())?;

        let mut pair = inner.next().unwrap();
        while pair.as_rule() != Rule::EOI {
            entities.push(EntityFrame::from_pair_unchecked(pair)?);
            pair = inner.next().unwrap();
        }
        Ok(OboDoc { header, entities })
    }
}
impl_fromstr!(OboDoc);

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::iter::FromIterator;
    use textwrap::dedent;

    #[test]
    fn from_str() {
        // Empty file should give empty `OboDoc`.
        let doc = OboDoc::from_str("").unwrap();
        self::assert_eq!(doc, Default::default());

        // Empty lines should be ignored.
        let doc = OboDoc::from_str("\n\n").unwrap();
        self::assert_eq!(doc, Default::default());

        // A simple file should parse.
        let doc = OboDoc::from_str(&dedent(
            "
            format-version: 1.2

            [Term]
            id: TEST:001
        ",
        ))
        .unwrap();

        let header = HeaderFrame::from_iter(vec![HeaderClause::FormatVersion(
            UnquotedString::new("1.2"),
        )]);
        let term = TermFrame::new(ClassIdent::from(PrefixedIdent::new("TEST", "001")));
        self::assert_eq!(doc, OboDoc::from_iter(Some(term)).and_header(header));
    }

    #[test]
    fn to_string() {
        // Empty `OboDoc` should give empty string.
        let doc = OboDoc::default();
        self::assert_eq!(doc.to_string(), "");

        // `OboDoc` with only header frame should not add newline separator.
        let doc = OboDoc::with_header(HeaderFrame::from(vec![
            HeaderClause::FormatVersion(UnquotedString::new("1.2")),
            HeaderClause::Remark(UnquotedString::new("this is a test")),
        ]));
        self::assert_eq!(
            doc.to_string(),
            dedent(
                "
            format-version: 1.2
            remark: this is a test
            "
            )
            .trim_start_matches('\n')
        );
    }
}
