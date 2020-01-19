use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::str::FromStr;

use pest::Parser;

use crate::ast::EntityFrame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;

use super::OboParser;
use super::Rule;
use super::FromPair;

/// An iterator reading entity frames contained in an OBO stream.
pub struct SequentialReader<B: BufRead> {
    stream: B,
    line: String,
    offset: usize,
    line_offset: usize,
    header: Result<HeaderFrame, Error>
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
        let mut header_err = None;

        loop {
            // Read the next line
            line.clear();
            if let Err(e) = stream.read_line(&mut line) {
                header_err = Some(Error::from(e));
                break;
            };
            l = line.trim();

            // Parse header as long as we didn't reach EOL or first frame.
            if !l.starts_with('[') && !l.is_empty() {
                unsafe {
                    // use `OboParser` to tokenize the input
                    let p = match OboParser::parse(Rule::HeaderClause, &line) {
                        Ok(mut pairs) => pairs.next().unwrap(),
                        Err(e) => {
                            let err = SyntaxError::from(e).with_offsets(line_offset, offset);
                            header_err = Some(Error::from(err));
                            break;
                        }
                    };
                    // produce a header clause from the token stream
                    match HeaderClause::from_pair_unchecked(p) {
                        Ok(clause) => frame_clauses.push(clause),
                        Err(e) => {
                            let err = SyntaxError::from(e).with_offsets(line_offset, offset);
                            header_err = Some(Error::from(err));
                            break;
                        }
                    }
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

        Self {
            stream,
            line,
            offset,
            line_offset,
            header: match header_err {
                Some(e) => Err(e),
                None => Ok(HeaderFrame::from(frame_clauses)),
            },
        }
    }

    /// Get a reference to the parsed header frame.
    pub fn header(&self) -> Option<&HeaderFrame> {
        self.header.as_ref().ok()
    }

    /// Get a mutable reference to the parsed header frame.
    pub fn header_mut(&mut self) -> Option<&mut HeaderFrame> {
        self.header.as_mut().ok()
    }
}

// impl<B: BufRead> AsRef<B> for SequentialReader<B> {
//     fn as_ref(&self) -> &B {
//         &self.stream
//     }
// }
//
// impl<B: BufRead> AsMut<B> for SequentialReader<B> {
//     fn as_mut(&mut self) -> &mut B {
//         &mut self.stream
//     }
// }

// impl TryFrom<File> for SequentialReader<BufReader<File>> {
//     type Error = Error;
//     fn try_from(f: File) -> Result<Self, Self::Error> {
//         Self::new(BufReader::new(f))
//     }
// }

impl<B: BufRead> Iterator for SequentialReader<B> {
    type Item = Result<EntityFrame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut l: &str = &self.line;
        let mut frame_lines = String::new();
        let mut local_line_offset = 0;
        let mut local_offset = 0;

        if self.header.is_err() {
            let e = std::mem::replace(&mut self.header, Ok(HeaderFrame::new()));
            return Some(Err(e.unwrap_err()));
        }

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
                        Ok(mut pairs) => EntityFrame::from_pair_unchecked(pairs.next().unwrap())
                            .map_err(Error::from),
                        Err(e) => Err(Error::from(
                            SyntaxError::from(e).with_offsets(self.line_offset, self.offset),
                        )),
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

impl<B: BufRead> TryFrom<SequentialReader<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: SequentialReader<B>) -> Result<Self, Self::Error> {
        let mut doc = OboDoc::new();
        for result in &mut reader {
            doc.entities_mut().push(result?);
        }

        // header is always Ok
        std::mem::swap(reader.header_mut().unwrap(), doc.header_mut());
        Ok(doc)
    }
}
