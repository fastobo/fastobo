//! Owned syntax tree for the [OBO format version 1.4].
//!
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.

mod date;
mod header;
mod id;
mod instance;
mod line;
mod misc;
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
pub use self::qualifier::*;
pub use self::strings::*;
pub use self::synonym::*;
pub use self::term::*;
pub use self::typedef::*;
pub use self::xref::*;

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

        // collect the header frame
        let mut frame_clauses = Vec::new();
        while !l.starts_with('[') && !line.is_empty() {
            if !l.is_empty() {
                let clause = OboParser::parse(Rule::HeaderClause, &line)
                    .map_err(Error::from)
                    .and_then(|mut p| unsafe {
                        let pair = p.next().unwrap();
                        HeaderClause::from_pair_unchecked(pair)
                    })?;
                frame_clauses.push(clause)
            }

            line.clear();
            stream.read_line(&mut line)?;
            l = line.trim();
        }

        // create the OBO document
        let mut obodoc = Self::new(HeaderFrame::new(frame_clauses));

        // read all entity frames
        let mut frame_lines = String::new();
        while !line.is_empty() {
            frame_lines.push_str(&line);
            line.clear();
            stream.read_line(&mut line)?;

            if line.trim_start().starts_with('[') {
                let mut pairs = OboParser::parse(Rule::EntitySingle, &frame_lines)?;
                obodoc
                    .entities
                    .push(EntityFrame::from_pair(pairs.next().unwrap())?);
                frame_lines.clear()
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
pub enum EntityFrame {
    Term(TermFrame),
    Typedef(TypedefFrame),
    Instance(InstanceFrame),
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
