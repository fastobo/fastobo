use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::num::NonZeroUsize;
use std::sync::Arc;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError;
use lazy_static::lazy_static;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::error::ThreadingError;
use crate::syntax::Lexer;
use crate::syntax::Rule;

use super::Cache;
use super::FromPair;
use super::Parser;

use self::consumer::Consumer;
use self::consumer::Input as ConsumerInput;
use self::consumer::Output as ConsumerOutput;

mod consumer;

// ---

/// The state of a `ThreadedParser` instance.
#[derive(PartialEq, Eq)]
enum State {
    Idle,
    Started,
    AtEof,
    Waiting,
    Finished,
}

// ---

/// An iterator reading entity frames contained in an OBO stream in parallel.
#[cfg_attr(feature = "_doc", doc(cfg(feature = "threading")))]
pub struct ThreadedParser<B: BufRead> {
    // the reader
    stream: B,
    // the state of the parser
    state: State,
    // the consumer threads
    consumers: Vec<Consumer>,

    // communication channels
    r_item: Receiver<ConsumerOutput>,
    s_text: Sender<Option<ConsumerInput>>,

    /// Buffer for the last line that was read.
    line: String,

    /// Number of threads requested by the user
    threads: NonZeroUsize,

    /// Offsets to report proper error position
    line_offset: usize,
    offset: usize,

    /// Local progress counters
    ordered: bool,
    read_index: usize,
    sent_index: usize,

    /// Result queue to maintain frame order if in ordered mode.
    queue: HashMap<usize, Result<Frame, Error>>,
}

impl<B: BufRead> AsRef<B> for ThreadedParser<B> {
    fn as_ref(&self) -> &B {
        &self.stream
    }
}

impl<B: BufRead> AsRef<B> for Box<ThreadedParser<B>> {
    fn as_ref(&self) -> &B {
        (**self).as_ref()
    }
}

impl<B: BufRead> AsMut<B> for ThreadedParser<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.stream
    }
}

impl<B: BufRead> AsMut<B> for Box<ThreadedParser<B>> {
    fn as_mut(&mut self) -> &mut B {
        (**self).as_mut()
    }
}

impl<B: BufRead> From<B> for ThreadedParser<B> {
    fn from(reader: B) -> Self {
        <Self as Parser<B>>::new(reader)
    }
}

impl<B: BufRead> From<B> for Box<ThreadedParser<B>> {
    fn from(reader: B) -> Self {
        Box::new(ThreadedParser::new(reader))
    }
}

impl<B: BufRead> Iterator for ThreadedParser<B> {
    type Item = Result<Frame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! send_or_error {
            ($channel:expr, $msg:expr) => {
                if $channel.send($msg).is_err() {
                    self.state = State::Finished;
                    let err = ThreadingError::DisconnectedChannel;
                    return Some(Err(Error::from(err)));
                }
            };
        }

        loop {
            // return and item from the queue if in ordered mode
            if self.ordered {
                if let Some(result) = self.queue.remove(&self.read_index) {
                    self.read_index += 1;
                    return Some(result);
                }
            }

            // poll for parsed frames to return
            match self.r_item.try_recv().map(|i| (i.res, i.index)) {
                // item is found, don't care about order: simply return it
                Ok((Ok(entry), _)) if !self.ordered => return Some(Ok(entry)),
                // error is found: finalize and return it
                Ok((Err(e), _)) if !self.ordered => {
                    self.state = State::Finished;
                    return Some(Err(e));
                }
                // item is found and is the right index: return it
                Ok((result, index)) if index == self.read_index => {
                    self.read_index += 1;
                    return Some(result);
                }
                // item is found but is not the right index: store it
                Ok((result, index)) => {
                    self.queue.insert(index, result);
                }
                // empty queue after all the threads were joined: we are done
                Err(TryRecvError::Empty) if self.state == State::Waiting => {
                    self.state = State::Finished;
                    return None;
                }
                // empty queue in any other state: just do something else
                Err(TryRecvError::Empty) => (),
                // queue was disconnected: stop and return an error
                Err(TryRecvError::Disconnected) => {
                    if self.state != State::Finished {
                        self.state = State::Finished;
                        return Some(Err(Error::from(ThreadingError::DisconnectedChannel)));
                    }
                }
            }

            // depending on the state, do something before polling
            match self.state {
                State::Waiting => (),
                State::AtEof => {
                    self.state = State::Waiting;
                    for consumer in self.consumers.iter_mut() {
                        consumer.join().unwrap();
                    }
                }
                State::Idle => {
                    self.state = State::Started;
                    for consumer in &mut self.consumers {
                        consumer.start();
                    }
                }
                State::Finished => {
                    return None;
                }
                State::Started => {
                    //
                    let mut lines = String::new();
                    let mut l: &str;
                    let mut local_line_offset = 0;
                    let mut local_offset = 0;

                    loop {
                        // store the previous line and process the next line
                        lines.push_str(&self.line);
                        self.line.clear();

                        // read the next line
                        if let Err(e) = self.stream.read_line(&mut self.line) {
                            self.state = State::Finished;
                            return Some(Err(Error::from(e)));
                        }

                        // check if we reached the end of the frame
                        l = self.line.trim_start();
                        if l.starts_with('[') {
                            // send the entire frame with the location offsets
                            let msg = ConsumerInput::new(
                                lines,
                                self.sent_index,
                                self.line_offset,
                                self.offset,
                            );
                            send_or_error!(self.s_text, Some(msg));
                            // update the local offsets and bail out
                            self.sent_index += 1;
                            self.line_offset += local_line_offset + 1;
                            self.offset += local_offset + self.line.len();
                            break;
                        } else if self.line.is_empty() {
                            // change the state to wait for workers to finish
                            self.state = State::AtEof;
                            // if some lines remain, send them as text
                            if !lines.chars().all(|c| c.is_whitespace()) {
                                let msg = ConsumerInput::new(
                                    lines,
                                    self.sent_index,
                                    self.line_offset,
                                    self.offset,
                                );
                                send_or_error!(self.s_text, Some(msg));
                            }
                            // poison-pill the remaining workers and bail out
                            for _ in 0..self.threads.get() {
                                send_or_error!(self.s_text, None);
                            }
                            break;
                        }

                        // Update local offsets
                        local_line_offset += 1;
                        local_offset += self.line.len();
                    }
                }
            }
        }
    }
}

impl<B: BufRead> Parser<B> for ThreadedParser<B> {
    /// Create a new `ThreadedParser` with all available CPUs.
    ///
    /// The number of available CPUs will be polled at runtime and then the
    /// right number of threads will be spawned accordingly.
    fn new(stream: B) -> Self {
        lazy_static! {
            static ref THREADS: usize = num_cpus::get();
        }
        let threads = unsafe { NonZeroUsize::new_unchecked(*THREADS) };
        Self::with_threads(stream, threads)
    }

    /// Create a new `ThreadedParser` with the given number of threads.
    fn with_threads(mut stream: B, threads: NonZeroUsize) -> Self {
        // create the buffers and counters
        let mut frame_clauses = Vec::new();
        let mut line = String::new();
        let mut l: &str;
        let mut offset = 0;
        let mut line_offset = 0;
        let interner = Arc::new(Cache::default());

        // create the communication channels
        let (s_text, r_text) = crossbeam_channel::unbounded();
        let (s_item, r_item) = crossbeam_channel::unbounded();

        // read until we reach the first entity frame
        let header = loop {
            // Read the next line
            line.clear();
            if let Err(e) = stream.read_line(&mut line) {
                break Err(Error::from(e));
            };
            l = line.trim_start();

            // if the line is not empty, parse it
            if !l.starts_with('[') && !l.is_empty() {
                // parse the header clause
                let clause = Lexer::tokenize(Rule::HeaderClause, &line)
                    .map_err(SyntaxError::from)
                    .map(|mut p| p.next().unwrap())
                    .and_then(|p| HeaderClause::from_pair(p, &interner))
                    .map_err(SyntaxError::from);
                // check if the clause was parsed properly or not
                match clause {
                    Ok(c) => frame_clauses.push(c),
                    Err(e) => {
                        let err = e.with_offsets(line_offset, offset);
                        break Err(Error::from(err));
                    }
                };
            }

            // if the line is the beginning of an entity frame, stop
            if l.starts_with('[') || line.is_empty() {
                break Ok(Frame::from(HeaderFrame::from(frame_clauses)));
            } else {
                line_offset += 1;
                offset += line.len();
            }
        };

        // create the consumers
        let mut consumers = Vec::with_capacity(threads.get());
        for _ in 0..threads.get() {
            let c = Consumer::new(r_text.clone(), s_item.clone(), interner.clone());
            consumers.push(c);
        }

        // send the header to the channel (to get it back immediately after)
        s_item.send(ConsumerOutput::new(header, 0)).ok();

        // return the parser
        Self {
            stream,
            r_item,
            s_text,
            threads,
            consumers,
            line,
            line_offset,
            offset,
            ordered: false,
            read_index: 0,
            sent_index: 1,
            queue: HashMap::new(),
            state: State::Idle,
        }
    }

    /// Make the parser yield frames in the order they appear in the document.
    ///
    /// Note that this has a small performance impact, so this is disabled
    /// by default.
    fn ordered(&mut self, ordered: bool) -> &mut Self {
        self.ordered = ordered;
        self
    }

    /// Consume the reader and extract the internal reader.
    fn into_inner(self) -> B {
        self.stream
    }
}

impl<B: BufRead> Parser<B> for Box<ThreadedParser<B>> {
    fn new(stream: B) -> Self {
        Box::new(ThreadedParser::new(stream))
    }

    fn with_threads(stream: B, threads: NonZeroUsize) -> Self {
        Box::new(ThreadedParser::with_threads(stream, threads))
    }

    fn ordered(&mut self, ordered: bool) -> &mut Self {
        (**self).ordered(ordered);
        self
    }

    fn into_inner(self) -> B {
        (*self).into_inner()
    }
}

impl<B: BufRead> TryFrom<ThreadedParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: ThreadedParser<B>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut reader)
    }
}

impl<B: BufRead> TryFrom<&mut ThreadedParser<B>> for OboDoc {
    type Error = Error;
    fn try_from(reader: &mut ThreadedParser<B>) -> Result<Self, Self::Error> {
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

impl<B: BufRead> TryFrom<Box<ThreadedParser<B>>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: Box<ThreadedParser<B>>) -> Result<Self, Self::Error> {
        OboDoc::try_from(&mut (*reader))
    }
}

impl From<File> for ThreadedParser<BufReader<File>> {
    fn from(f: File) -> Self {
        Self::new(BufReader::new(f))
    }
}

impl From<File> for Box<ThreadedParser<BufReader<File>>> {
    fn from(f: File) -> Self {
        Box::new(ThreadedParser::new(BufReader::new(f)))
    }
}
