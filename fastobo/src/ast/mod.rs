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
mod misc;
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
pub use self::misc::*;
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
    /// Create a new OBO document.
    pub fn new(header: HeaderFrame) -> Self {
        Self {
            header,
            entities: Vec::new(),
        }
    }

    /// Create a new OBO document with the provided entity frames.
    pub fn with_entities<E>(header: HeaderFrame, entities: E) -> Self
    where
        E: IntoIterator<Item = EntityFrame>,
    {
        Self {
            header,
            entities: entities.into_iter().collect(),
        }
    }

    /// Consume an OBO stream to produce the corresponding AST.
    pub fn from_stream<B>(stream: &mut B) -> Result<Self>
    where
        B: BufRead,
    {
        let mut line = String::from("\n");
        let mut l: &str = &line[..0];
        let mut idx = 0;

        // collect the header frame
        let mut frame_clauses = Vec::new();
        while !l.starts_with('[') && !line.is_empty() {
            // Parse the line if it is not empty.
            if !l.is_empty() {
                unsafe {
                    let mut pairs = OboParser::parse(Rule::HeaderClause, &line)?;
                    let clause = HeaderClause::from_pair_unchecked(pairs.next().unwrap())?;
                    frame_clauses.push(clause);
                }
            }
            // Read the next line
            line.clear();
            stream.read_line(&mut line)?;
            l = line.trim();
            idx += 1;
        }

        // create the OBO document
        let mut obodoc = Self::new(HeaderFrame::new(frame_clauses));

        // read all entity frames
        let mut frame_lines = String::new();
        while !line.is_empty() {
            // Read the next line.
            frame_lines.push_str(&line);
            line.clear();
            stream.read_line(&mut line)?;
            idx += 1;
            // Read the line if we reached the next frame.
            if line.trim_start().starts_with('[') || line.is_empty() {
                unsafe {
                    let mut pairs = OboParser::parse(Rule::EntitySingle, &frame_lines)?;
                    let entity = EntityFrame::from_pair_unchecked(pairs.next().unwrap())?;
                    obodoc.entities.push(entity);
                    frame_lines.clear()
                }
            }
        }

        Ok(obodoc)
    }

    /// Read
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(path).map(BufReader::new)?;
        Self::from_stream(&mut f)
    }

    pub fn header(&self) -> &HeaderFrame {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut HeaderFrame {
        &mut self.header
    }

    pub fn set_header(&mut self, header: HeaderFrame) {
        self.header = header
    }
}

impl Display for OboDoc {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.header.fmt(f).and(f.write_char('\n'))?;
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
