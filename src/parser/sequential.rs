use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::str::FromStr;

use pest::Parser;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;

use super::OboParser;
use super::Rule;
use super::FromPair;

/// An iterator reading entity frames contained in an OBO stream sequentially.
pub struct SequentialReader<B: BufRead> {
    stream: B,
    line: String,
    offset: usize,
    line_offset: usize,
    header: Option<Result<Frame, Error>>,
}

impl<B: BufRead> SequentialReader<B> {
    /// Create a new `SequentialReader` from the given stream.
    ///
    /// The constructor will parse the header frame right away, and return an
    /// error if it fails. The header can then be accessed using the `header`
    /// method.
    pub fn new(mut stream: B) -> Self {
        let mut line = String::new();
        let mut l: &str;
        let mut offset = 0;
        let mut line_offset = 0;
        let mut frame_clauses = Vec::new();

        let header = loop {
            // Read the next line
            line.clear();
            if let Err(e) = stream.read_line(&mut line) {
                break Some(Err(Error::from(e)));
            };
            l = line.trim_start();

            // Parse header as long as we didn't reach EOL or first frame.
            if !l.starts_with('[') && !l.is_empty() {
                unsafe {
                    // use `OboParser` to tokenize the input
                    let p = match OboParser::parse(Rule::HeaderClause, &line) {
                        Ok(mut pairs) => pairs.next().unwrap(),
                        Err(e) => {
                            let err = SyntaxError::from(e).with_offsets(line_offset, offset);
                            break Some(Err(Error::from(err)));
                        }
                    };
                    // produce a header clause from the token stream
                    match HeaderClause::from_pair_unchecked(p) {
                        Ok(clause) => frame_clauses.push(clause),
                        Err(e) => {
                            let err = e.with_offsets(line_offset, offset);
                            break Some(Err(Error::from(err)));
                        }
                    }
                }
            }

            if l.starts_with('[') || line.is_empty() {
                // Bail out if we reached EOL or first frame.
                let frame = Frame::Header(HeaderFrame::from(frame_clauses));
                break Some(Ok(frame));
            } else {
                // Update offsets
                line_offset += 1;
                offset += line.len();
            }
        };

        Self {
            stream,
            line,
            offset,
            line_offset,
            header
        }
    }

    /// Make the parser yield frames in the order they appear in the document.
    ///
    /// This has no effect on `SequentialReader` since the frames are always
    /// processed in the document order, but this method is provided for
    /// consistency of the [`FrameReader`](./type.FrameReader.html) type.
    pub fn ordered(&mut self, _ordered: bool) -> &mut Self {
        self
    }
}

impl<B: BufRead> AsRef<B> for SequentialReader<B> {
    fn as_ref(&self) -> &B {
        &self.stream
    }
}

impl<B: BufRead> AsMut<B> for SequentialReader<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.stream
    }
}

impl From<File> for SequentialReader<BufReader<File>> {
    fn from(f: File) -> Self {
        Self::new(BufReader::new(f))
    }
}

impl<B: BufRead> Iterator for SequentialReader<B> {
    type Item = Result<Frame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut l: &str;
        let mut frame_lines = String::new();
        let mut local_line_offset = 0;
        let mut local_offset = 0;

        if let Some(res) = self.header.take() {
            return Some(res);
        }

        while !self.line.is_empty() {
            // Store the line in the frame lines and clear the buffer.
            frame_lines.push_str(&self.line);
            self.line.clear();

            // Read the next line.
            if let Err(e) = self.stream.read_line(&mut self.line) {
                return Some(Err(Error::from(e)));
            }

            // Process the frame if we reached the next frame.
            l = self.line.trim_start();
            if l.starts_with('[') || self.line.is_empty() {
                let res = unsafe {
                    match OboParser::parse(Rule::EntitySingle, &frame_lines) {
                        Ok(mut pairs) => EntityFrame::from_pair_unchecked(pairs.next().unwrap())
                            .map_err(Error::from),
                        Err(e) => Err(Error::from(
                            SyntaxError::from(e).with_offsets(self.line_offset, self.offset),
                        )),
                    }
                };

                // Update offsets
                self.line_offset += local_line_offset + 1;
                self.offset += local_offset + self.line.len();
                return Some(res.map(Frame::from));
            }

            // Update local offsets
            local_line_offset += 1;
            local_offset += self.line.len();
        }

        None
    }
}

impl<B: BufRead> TryFrom<&mut SequentialReader<B>> for OboDoc {
    type Error = Error;
    fn try_from(reader: &mut SequentialReader<B>) -> Result<Self, Self::Error> {
        // extract the header and create the doc
        let header = reader.next().unwrap()?.into_header_frame().unwrap();

        // extract the remaining entities
        let entities = reader
            .map(|r| r.map(|f| f.into_entity_frame().unwrap()))
            .collect::<Result<Vec<EntityFrame>, Error>>()?;

        // return the doc
        Ok(OboDoc::with_header(header).and_entities(entities))
    }
}

impl<B: BufRead> TryFrom<SequentialReader<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: SequentialReader<B>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut reader)
    }
}
