use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::syntax::Lexer;
use crate::syntax::Rule;

use super::Cache;
use super::FromPair;
use super::Parser;

/// An iterator reading entity frames contained in an OBO stream sequentially.
pub struct SequentialParser<B: BufRead> {
    stream: B,
    line: String,
    offset: usize,
    line_offset: usize,
    header: Option<Result<Frame, Error>>,
    cache: Cache,
}

impl<B: BufRead> AsRef<B> for SequentialParser<B> {
    fn as_ref(&self) -> &B {
        &self.stream
    }
}

impl<B: BufRead> AsRef<B> for Box<SequentialParser<B>> {
    fn as_ref(&self) -> &B {
        (**self).as_ref()
    }
}

impl<B: BufRead> AsMut<B> for SequentialParser<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.stream
    }
}

impl<B: BufRead> AsMut<B> for Box<SequentialParser<B>> {
    fn as_mut(&mut self) -> &mut B {
        (**self).as_mut()
    }
}

impl<B: BufRead> From<B> for SequentialParser<B> {
    fn from(reader: B) -> Self {
        <Self as Parser<B>>::new(reader)
    }
}

impl<B: BufRead> From<B> for Box<SequentialParser<B>> {
    fn from(stream: B) -> Self {
        Box::new(SequentialParser::from(stream))
    }
}

impl<B: BufRead> Iterator for SequentialParser<B> {
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
                    match Lexer::tokenize(Rule::EntitySingle, &frame_lines) {
                        Ok(mut pairs) => {
                            EntityFrame::from_pair_unchecked(pairs.next().unwrap(), &self.cache)
                                .map_err(Error::from)
                        }
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

impl<B: BufRead> Parser<B> for SequentialParser<B> {
    /// Create a new `SequentialParser` from the given stream.
    ///
    /// The constructor will parse the header frame right away, and return an
    /// error if it fails. The header can then be accessed using the `header`
    /// method.
    fn new(mut stream: B) -> Self {
        let cache = Cache::default();
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
                    // use `fastobo_syntax::Lexer` to tokenize the input
                    let p = match Lexer::tokenize(Rule::HeaderClause, &line) {
                        Ok(mut pairs) => pairs.next().unwrap(),
                        Err(e) => {
                            let err = SyntaxError::from(e).with_offsets(line_offset, offset);
                            break Some(Err(Error::from(err)));
                        }
                    };
                    // produce a header clause from the token stream
                    match HeaderClause::from_pair_unchecked(p, &cache) {
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
                let frame = Frame::from(HeaderFrame::from(frame_clauses));
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
            header,
            cache,
        }
    }

    /// Make the parser yield frames in the order they appear in the document.
    ///
    /// This has no effect on `SequentialParser` since the frames are always
    /// processed in the document order, but this method is provided for
    /// consistency of the [`FrameReader`](./type.FrameReader.html) type.
    fn ordered(&mut self, _ordered: bool) -> &mut Self {
        self
    }

    /// Consume the reader and extract the internal reader.
    fn into_inner(self) -> B {
        self.stream
    }
}

impl<B: BufRead> Parser<B> for Box<SequentialParser<B>> {
    fn new(stream: B) -> Self {
        Box::new(SequentialParser::new(stream))
    }

    fn ordered(&mut self, ordered: bool) -> &mut Self {
        (**self).ordered(ordered);
        self
    }

    fn into_inner(self) -> B {
        (*self).into_inner()
    }
}

impl<B: BufRead> TryFrom<SequentialParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut parser: SequentialParser<B>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut parser)
    }
}

impl<B: BufRead> TryFrom<&mut SequentialParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(parser: &mut SequentialParser<B>) -> Result<Self, Self::Error> {
        // extract the header and create the doc
        let header = parser.next().unwrap()?.into_header_frame().unwrap();

        // extract the remaining entities
        let entities = parser
            .map(|r| r.map(|f| f.into_entity_frame().unwrap()))
            .collect::<Result<Vec<EntityFrame>, Error>>()?;

        // return the doc
        Ok(OboDoc::with_header(header).and_entities(entities))
    }
}

impl<B: BufRead> TryFrom<Box<SequentialParser<B>>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: Box<SequentialParser<B>>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut (*reader))
    }
}

impl From<File> for SequentialParser<BufReader<File>> {
    fn from(f: File) -> Self {
        Self::new(BufReader::new(f))
    }
}

impl From<File> for Box<SequentialParser<BufReader<File>>> {
    fn from(f: File) -> Self {
        Box::new(SequentialParser::new(BufReader::new(f)))
    }
}
