//! Parser and parsing-related traits for the OBO format.

use std::convert::TryFrom;
use std::io::BufRead;
use std::iter::Iterator;
use std::str::FromStr;

use pest::Parser;

use crate::ast::EntityFrame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;

#[macro_use]
mod macros;
mod from_pair;
mod from_slice;
mod quickfind;

#[doc(inline)]
pub use fastobo_syntax::OboParser;
#[doc(inline)]
pub use fastobo_syntax::Rule;

pub use self::from_pair::FromPair;
pub use self::from_slice::FromSlice;
pub use self::quickfind::QuickFind;

// ---

/// An iterator reading entity frames contained in an OBO stream.
pub struct FrameReader<B: BufRead> {
    stream: B,
    line: String,
    offset: usize,
    line_offset: usize,
    header: HeaderFrame,
}

impl<B: BufRead> FrameReader<B> {
    /// Create a new `FrameReader` from the given stream.
    ///
    /// The constructor will parse the header frame right away, and return an
    /// error if it fails. The header can then be accessed using the `header`
    /// method.
    pub fn new(mut stream: B) -> Result<Self, Error> {
        let mut line = String::new();
        let mut l: &str;
        let mut offset = 0;
        let mut line_offset = 0;
        let mut frame_clauses = Vec::new();

        loop {
            // Read the next line
            line.clear();
            stream.read_line(&mut line)?;
            l = line.trim();

            // Parse header as long as we didn't reach EOL or first frame.
            if !l.starts_with('[') && !l.is_empty() {
                unsafe {
                    let mut pairs = OboParser::parse(Rule::HeaderClause, &line).map_err(|e| {
                        Error::from(SyntaxError::from(e).with_offsets(line_offset, offset))
                    })?;
                    let clause = HeaderClause::from_pair_unchecked(pairs.next().unwrap())?;
                    frame_clauses.push(clause);
                }
            }

            // Update offsets
            line_offset += 1;
            offset += line.len();

            // Bail out if we reached EOL or first frame.
            if l.starts_with('[') || line.is_empty() {
                break;
            }
        }

        Ok(Self {
            stream,
            line,
            offset,
            line_offset,
            header: HeaderFrame::from(frame_clauses),
        })
    }

    /// Get a reference to the parsed header frame.
    pub fn header(&self) -> &HeaderFrame {
        &self.header
    }

    /// Get a mutable reference to the parsed header frame.
    pub fn header_mut(&mut self) -> &mut HeaderFrame {
        &mut self.header
    }
}

impl<B: BufRead> Iterator for FrameReader<B> {
    type Item = Result<EntityFrame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut l: &str = &self.line;
        let mut frame_lines = String::new();
        let mut local_line_offset = 0;
        let mut local_offset = 0;

        while !self.line.is_empty() {
            // Read the next line.
            frame_lines.push_str(l);
            self.line.clear();

            if let Err(e) = self.stream.read_line(&mut self.line) {
                return Some(Err(Error::from(e)));
            }

            l = self.line.trim_start();

            // Read the line if we reached the next frame.
            if l.starts_with('[') || self.line.is_empty() {
                let res = unsafe {
                    match OboParser::parse(Rule::EntitySingle, &frame_lines) {
                        Ok(mut pairs) => {
                            EntityFrame::from_pair_unchecked(pairs.next().unwrap())
                                .map_err(Error::from)
                        }
                        Err(e) => {
                            Err(Error::from(
                                SyntaxError::from(e).with_offsets(self.line_offset, self.offset),
                            ))
                        }
                    }
                };

                // Update offsets
                self.line_offset += local_line_offset;
                self.offset += local_offset;

                return Some(res);
            }

            // Update local offsets
            local_line_offset += 1;
            local_offset += self.line.len();
        }

        None
    }
}

impl<B: BufRead> TryFrom<FrameReader<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: FrameReader<B>) -> Result<OboDoc, Error> {
        let mut doc = OboDoc::new();
        std::mem::swap(reader.header_mut(), doc.header_mut());

        for result in reader {
            doc.entities_mut().push(result?);
        }

        Ok(doc)
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use textwrap::dedent;

    use super::*;
    use crate::ast::Identified;

    #[test]
    fn frame_reader_empty() {
        let mut reader = FrameReader::new(Cursor::new("")).unwrap();
        assert!(reader.next().is_none());
        assert!(reader.header().is_empty());
    }

    #[test]
    fn frame_reader_iter() {
        let mut reader = FrameReader::new(Cursor::new(dedent(
            r#"
            format-version: 1.2

            [Term]
            id: TST:001

            [Term]
            id: TST:002
        "#,
        )))
        .expect("could not parse frame header");

        assert_eq!(
            reader.next().unwrap().unwrap().as_id().to_string(),
            "TST:001"
        );
        assert_eq!(
            reader.next().unwrap().unwrap().as_id().to_string(),
            "TST:002"
        );
        assert!(reader.next().is_none());
    }
}
