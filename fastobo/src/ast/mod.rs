//! Owned syntax tree for the [OBO format version 1.4].
//!
//! [`OboDoc`] is the struct acting as the root of the syntax tree. It can be
//! created from a borrowed string slice with either [`FromStr::from_str`] or
//! [`FromSlice::from_slice`], from a file with [`OboDoc::from_file`], or from
//! a buffered reader with [`OboDoc::from_stream`].
//!
//! [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
//! [`FromSlice::from_slice`]: ../parser/trait.FromSlice.html#method.from_slice
//! [`OboDoc`]: ./struct.OboDoc.html
//! [`OboDoc::from_file`]: ./struct.OboDoc.html#method.from_file
//! [`OboDoc::from_stream`]: ./struct.OboDoc.html#method.from_stream
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.


mod date;
mod header;
mod id;
mod instance;
mod line;
mod pv;
mod qualifier;
mod strings;
mod synonym;
mod term;
mod typedef;
mod xref;

pub use self::date::*;
pub use self::header::*;
pub use self::id::*;
pub use self::instance::*;
pub use self::line::*;
pub use self::pv::*;
pub use self::qualifier::*;
pub use self::strings::*;
pub use self::synonym::*;
pub use self::term::*;
pub use self::typedef::*;
pub use self::xref::*;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::OboParser;
use crate::parser::Rule;

/// A complete OBO document in format version 1.4.
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct OboDoc {
    pub header: HeaderFrame,
    pub entities: Vec<EntityFrame>,
}

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
    pub fn from_stream<B>(stream: &mut B) -> Result<Self>
    where
        B: BufRead,
    {
        let mut line = String::new();
        let mut l: &str;

        let mut offset = 0;
        let mut line_offset = 0;

        // collect the header frame
        let mut frame_clauses = Vec::new();
        loop {
            // Read the next line
            line.clear();
            stream.read_line(&mut line)?;
            l = line.trim();

            // Parse header as long as we didn't reach EOL or first frame.
            if !l.starts_with('[') && !l.is_empty() {
                unsafe {
                    let mut pairs = OboParser::parse(Rule::HeaderClause, &line)
                        .map_err(|e| Error::from(e).with_offsets(line_offset, offset))?;
                    let clause = HeaderClause::from_pair_unchecked(pairs.next().unwrap())?;
                    frame_clauses.push(clause);
                }
            }

            // Update offsets
            line_offset += 1;
            offset += line.len();

            // Bail out if we reached EOL or first frame.
            if l.starts_with('[') || line.is_empty() {
                break
            }
        }

        // create the OBO document
        let mut obodoc = Self::with_header(HeaderFrame::with_clauses(frame_clauses));

        // read all entity frames
        let mut frame_lines = String::new();
        let mut local_line_offset = 0;
        let mut local_offset = 0;
        while !line.is_empty() {

            // Read the next line.
            frame_lines.push_str(&line);
            line.clear();
            stream.read_line(&mut line)?;

            // Read the line if we reached the next frame.
            if line.trim_start().starts_with('[') || line.is_empty() {

                unsafe {
                    let mut pairs = OboParser::parse(Rule::EntitySingle, &frame_lines)
                        .map_err(|e| Error::from(e).with_offsets(line_offset, offset))?;
                    let entity = EntityFrame::from_pair_unchecked(pairs.next().unwrap())?;
                    obodoc.entities.push(entity);
                    frame_lines.clear()
                }

                // Update offsets
                line_offset += local_line_offset;
                offset += local_offset;

                // Reset local offsets
                local_line_offset = 0;
                local_offset = 0;

            }

            // Update local offsets
            local_line_offset += 1;
            local_offset += line.len();
        }

        Ok(obodoc)
    }

    /// Read an OBO file located somwhere in the filesystem.
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let pathref: &Path = path.as_ref();
        File::open(pathref)
            .map_err(Error::from)
            .and_then(|f| Self::from_stream(&mut BufReader::new(f)))
            .map_err(|e| e.with_path(&pathref.to_string_lossy()))
    }

    /// Get a reference to the header of the OBO document.
    pub fn header(&self) -> &HeaderFrame {
        &self.header
    }

    /// Get a mutable reference to the header of the OBO document.
    pub fn header_mut(&mut self) -> &mut HeaderFrame {
        &mut self.header
    }

    /// Set the header of the OBO document.
    pub fn set_header(&mut self, header: HeaderFrame) {
        self.header = header
    }
}

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
    E: Into<EntityFrame>
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = E>
    {
        Self::with_entities(iter.into_iter().map(|i| i.into()).collect())
    }
}

impl<'i> FromPair<'i> for OboDoc {
    const RULE: Rule = Rule::OboDoc;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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

/// An entity frame, either for a term, an instance, or a typedef.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
}

impl Display for EntityFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::EntityFrame::*;
        match self {
            Term(t) => t.fmt(f),
            Typedef(t) => t.fmt(f) ,
            Instance(i) => i.fmt(f),
        }
    }
}

impl From<TermFrame> for EntityFrame {
    fn from(frame: TermFrame) -> Self {
        EntityFrame::Term(frame)
    }
}

impl From<TypedefFrame> for EntityFrame {
    fn from(frame: TypedefFrame) -> Self {
        EntityFrame::Typedef(frame)
    }
}

impl From<InstanceFrame> for EntityFrame {
    fn from(frame: InstanceFrame) -> Self {
        EntityFrame::Instance(frame)
    }
}

impl<'i> FromPair<'i> for EntityFrame {
    const RULE: Rule = Rule::EntityFrame;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::TermFrame => TermFrame::from_pair_unchecked(inner).map(From::from),
            Rule::TypedefFrame => TypedefFrame::from_pair_unchecked(inner).map(From::from),
            Rule::InstanceFrame => unimplemented!(),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(EntityFrame);

#[cfg(test)]
mod tests {

    use super::*;

    mod doc {

        use std::iter::FromIterator;
        use textwrap::dedent;
        use super::*;

        #[test]
        fn from_str() {
            // Empty file should give empty `OboDoc`.
            let doc = OboDoc::from_str("").unwrap();
            assert_eq!(doc, Default::default());

            // Empty lines should be ignored.
            let doc = OboDoc::from_str("\n\n").unwrap();
            assert_eq!(doc, Default::default());

            // A simple file should parse.
            let doc = OboDoc::from_str(&dedent("
                format-version: 1.2

                [Term]
                id: TEST:001
            ")).unwrap();

            let header = HeaderFrame::from_iter(
                vec![HeaderClause::FormatVersion(UnquotedString::new("1.2"))]
            );
            let term = TermFrame::new(
                ClassIdent::from(PrefixedIdent::new("TEST", "001"))
            );
            assert_eq!(doc, OboDoc::from_iter(Some(term)).and_header(header));
        }

        #[test]
        fn to_string() {
            // Empty `OboDoc` should give empty string.
            let doc = OboDoc::default();
            assert_eq!(doc.to_string(), "");

            // `OboDoc` with only header frame should not add newline separator.
            let doc = OboDoc::with_header(
                HeaderFrame::from(vec![
                    HeaderClause::FormatVersion(UnquotedString::new("1.2")),
                    HeaderClause::Remark(UnquotedString::new("this is a test")),
                ])
            );
            assert_eq!(doc.to_string(), dedent("
                format-version: 1.2
                remark: this is a test
                "
            ).trim_start_matches('\n'));
        }
    }
}
