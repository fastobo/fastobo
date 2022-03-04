use std::convert::TryFrom;
use std::io::Read;
use std::iter::Iterator;

use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::syntax::Lexer;
use crate::syntax::Rule;

use super::Buffer;
use super::Cache;
use super::FromPair;
use super::Parser;
use super::QuickFind;

lazy_static! {
    static ref FRAME_HEADERS: AhoCorasick = AhoCorasick::new(["[Term]", "[Typedef]", "[Instance]"]);
}

/// An iterator reading entity frames contained in an OBO stream sequentially.
#[derive(Debug)]
pub struct SequentialParser<B: Read> {
    stream: B,
    buffer: Buffer,
    header: Option<Result<HeaderFrame, Error>>,
    cache: Cache,
}

impl<B: Read> AsRef<B> for SequentialParser<B> {
    fn as_ref(&self) -> &B {
        &self.stream
    }
}

impl<B: Read> AsRef<B> for Box<SequentialParser<B>> {
    fn as_ref(&self) -> &B {
        (**self).as_ref()
    }
}

impl<B: Read> AsMut<B> for SequentialParser<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.stream
    }
}

impl<B: Read> AsMut<B> for Box<SequentialParser<B>> {
    fn as_mut(&mut self) -> &mut B {
        (**self).as_mut()
    }
}

impl<B: Read> From<B> for SequentialParser<B> {
    fn from(reader: B) -> Self {
        <Self as Parser<B>>::new(reader)
    }
}

impl<B: Read> From<B> for Box<SequentialParser<B>> {
    fn from(stream: B) -> Self {
        Box::new(SequentialParser::from(stream))
    }
}

impl<B: Read> Iterator for SequentialParser<B> {
    type Item = Result<Frame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // if we still have the header cached, now is the time to yield it
        if let Some(res) = self.header.take() {
            return Some(res.map(Frame::from));
        }

        // feed buffer until we find the end of a frame
        let frame_end = loop {
            // if there is already data in the buffer, we may be able to use it
            if !self.buffer.empty() {
                if let Some(m) = FRAME_HEADERS.earliest_find(&self.buffer.data()[1..]) {
                    break m.start() + 1;
                }
            }
            // otherwise, attempt to read more from the stream
            match self.stream.read(self.buffer.space()) {
                Err(e) => return Some(Err(Error::from(e))),
                Ok(0) if self.buffer.available_space() > 0 => {
                    break self.buffer.available_data();
                }
                Ok(n) => {
                    self.buffer.fill(n);
                    if self.buffer.available_space() * 2 < self.buffer.capacity() {
                        self.buffer.grow(self.buffer.capacity() * 2);
                    }
                }
            }
        };

        // parse a frame if there is one left
        let mut consumed = 0;
        let frame = {
            // decode the buffer data
            let decoded = match std::str::from_utf8(&self.buffer.data()[..frame_end]) {
                Ok(s) => s,
                Err(e) => return Some(Err(Error::from(e))),
            };
            // there's nothing to parse but whitespaces, we can stop here
            if decoded.trim().is_empty() {
                consumed = decoded.as_bytes().len();
                None
            } else {
                // tokenize the entirety of the decoded data
                if let Ok(mut pairs) = Lexer::tokenize_all(Rule::EntityFrame, decoded) {
                    unsafe {
                        consumed = decoded.as_bytes().len();
                        match EntityFrame::from_pair_unchecked(pairs.next().unwrap(), &self.cache) {
                            Err(e) => Some(Err(Error::from(e))),
                            Ok(frame) => Some(Ok(Frame::from(frame))),
                        }
                    }
                } else {
                    let mut offset = self.buffer.consumed();
                    let mut line_offset = self.buffer.consumed_lines();

                    // in case of error, we want accurate reporting of the
                    // error location, so we re-tokenize line-by-line to
                    // find where the error occurred
                    match Lexer::tokenize(Rule::EntityFrame, decoded) {
                        Err(e) => {
                            let err = SyntaxError::from(e).with_offsets(line_offset, offset);
                            Some(Err(Error::from(err)))
                        }
                        Ok(mut pairs) => {
                            // find which kind of frame we are parsing
                            let pair = pairs.next().unwrap().into_inner().next().unwrap();
                            let clause_rule = match pair.as_rule() {
                                Rule::TermFrame => Rule::TermClause,
                                Rule::TypedefFrame => Rule::TypedefClause,
                                Rule::InstanceFrame => Rule::InstanceClause,
                                other => unreachable!("invalid rule: {:?}", other),
                            };

                            //
                            let rest_offset = pair.as_str().trim_end().as_bytes().len();
                            let rest = &decoded[rest_offset..];

                            // record offset into the decoded text
                            let mut lines = rest.split_inclusive('\n');
                            offset += rest_offset;
                            line_offset += (&decoded[..rest_offset]).quickcount(b'\n');

                            // read header line-by-line to find the clause
                            loop {
                                let line = lines.next().unwrap();
                                let trimmed = line.trim_end_matches('\n');
                                if !trimmed.trim().is_empty() {
                                    if let Err(e) = Lexer::tokenize_all(clause_rule, trimmed) {
                                        let err =
                                            SyntaxError::from(e).with_offsets(line_offset, offset);
                                        break Some(Err(Error::from(err)));
                                    }
                                }
                                line_offset += 1;
                                offset += line.as_bytes().len();
                            }
                        }
                    }
                }
            }
        };

        // advance the buffer after we're done parsing (either because of an
        // error, or because we found the entire header frame)
        self.buffer.consume(consumed);

        // return the item, either a frame, an error, or nothing
        frame
    }
}

impl<B: Read> Parser<B> for SequentialParser<B> {
    /// Create a new `SequentialParser` from the given stream.
    ///
    /// The constructor will parse the header frame right away, and return an
    /// error if it fails. The header can then be accessed using the `header`
    /// method.
    fn new(mut stream: B) -> Self {
        let cache = Cache::default();
        let mut buffer = Buffer::with_capacity(4096);

        let mut consumed = 0;
        let frame_end = loop {
            // attempt to read data from the stream
            match stream.read(buffer.space()) {
                Err(e) => break Err(Error::from(e)),
                Ok(0) if buffer.available_space() > 0 => {
                    break Ok(FRAME_HEADERS
                        .earliest_find(&buffer.data())
                        .map(|m| m.start())
                        .unwrap_or(buffer.available_data()));
                }
                Ok(n) => {
                    buffer.fill(n);
                    if buffer.available_space() * 2 < buffer.capacity() {
                        buffer.grow(buffer.capacity() * 2);
                    }
                }
            }
            // stop reading if there is a full frame in here
            if let Some(m) = FRAME_HEADERS.earliest_find(&buffer.data()) {
                break Ok(m.start());
            }
        };

        let header = match frame_end {
            Err(e) => Some(Err(e)),
            Ok(i) => {
                // decode the buffer data
                match std::str::from_utf8(&buffer.data()[..i]) {
                    Err(e) => Some(Err(Error::from(e))),
                    Ok(decoded) if decoded.trim().is_empty() => {
                        consumed = decoded.as_bytes().len();
                        None
                    }
                    Ok(decoded) => {
                        // tokenize the entirety of the decoded data
                        if let Ok(mut pairs) = Lexer::tokenize_all(Rule::HeaderFrame, decoded) {
                            unsafe {
                                consumed = decoded.as_bytes().len();
                                match HeaderFrame::from_pair_unchecked(
                                    pairs.next().unwrap(),
                                    &cache,
                                ) {
                                    Err(e) => Some(Err(Error::from(e))),
                                    Ok(frame) => Some(Ok(frame)),
                                }
                            }
                        } else {
                            // record offset into the decoded text
                            let mut lines = decoded.split_inclusive('\n');
                            let mut offset = buffer.consumed();
                            let mut line_offset = buffer.consumed_lines();
                            // read header line-by-line to find the clause
                            loop {
                                let line = lines.next().unwrap();
                                let trimmed = line.trim_end_matches('\n');
                                if !trimmed.trim().is_empty() {
                                    if let Err(e) = Lexer::tokenize_all(Rule::HeaderClause, trimmed)
                                    {
                                        let err =
                                            SyntaxError::from(e).with_offsets(line_offset, offset);
                                        break Some(Err(Error::from(err)));
                                    }
                                }
                                line_offset += 1;
                                offset += line.as_bytes().len();
                            }
                        }
                    }
                }
            }
        };

        // advance the buffer after we're done parsing (either because of an
        // error, or because we found the entire header frame)
        buffer.consume(consumed);

        // return the parser with the header frame already parsed
        Self {
            stream,
            buffer,
            // line,
            // offset,
            // line_offset,
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

impl<B: Read> Parser<B> for Box<SequentialParser<B>> {
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

impl<B: Read> TryFrom<SequentialParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut parser: SequentialParser<B>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut parser)
    }
}

impl<B: Read> TryFrom<&mut SequentialParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(parser: &mut SequentialParser<B>) -> Result<Self, Self::Error> {
        // take the header if any was parsed successfully, or create an empty one
        let header = parser.header.take().transpose()?.unwrap_or_default();
        // extract the remaining entities
        let entities = parser
            .map(|r| r.map(|f| f.into_entity().unwrap()))
            .collect::<Result<Vec<EntityFrame>, Error>>()?;
        // return the doc
        Ok(OboDoc::with_header(header).and_entities(entities))
    }
}

impl<B: Read> TryFrom<Box<SequentialParser<B>>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: Box<SequentialParser<B>>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut (*reader))
    }
}
