use std::io::BufRead;
use std::io::BufReader;
use std::num::NonZeroUsize;
use std::thread::JoinHandle;
use std::time::Duration;
use std::convert::TryFrom;
use std::fs::File;
use std::iter::Iterator;
use std::str::FromStr;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::RecvTimeoutError;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderFrame;
use crate::ast::HeaderClause;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;

use super::OboParser;
use super::Rule;
use super::FromPair;

use super::consumer::Consumer;

// ---

#[derive(PartialEq, Eq)]
enum State {
    Idle,
    Started,
    AtEof,
    Waiting,
    Finished,
}

// ---

pub struct ThreadedReader<B: BufRead> {
    // the reader
    stream: B,
    // the state of the parser
    state: State,
    //
    consumers: Vec<Consumer>,

    // communication channels
    r_item: Receiver< Result<Frame, Error> >,
    s_text: Sender< Option<String> >,

    line: String,
    threads: NonZeroUsize,
}

impl<B: BufRead> ThreadedReader<B> {
    pub fn new(stream: B) -> Self {
        lazy_static !{ static ref THREADS: usize = num_cpus::get(); }
        let threads = unsafe { NonZeroUsize::new_unchecked(*THREADS) };
        Self::with_threads(stream, threads)
    }

    pub fn with_threads(mut stream: B, threads: NonZeroUsize) -> Self {
        //
        let mut frame_clauses = Vec::new();
        let mut line = String::new();
        let mut l: &str;

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
            l = line.trim();

            // if the line is not empty, parse it
            if !l.starts_with('[') && !l.is_empty() {
                // parse the header clause
                let clause = OboParser::parse(Rule::HeaderClause, &l)
                    .map_err(SyntaxError::from)
                    .map(|mut p| p.next().unwrap())
                    .and_then(HeaderClause::from_pair)
                    .map_err(Error::from);
                // check if the clause was parsed properly or not
                match clause {
                    Ok(c) => frame_clauses.push(c),
                    Err(e) => break Err(e),
                };
            }

            // if the line is the beginning of an entity frame, stop
            if l.starts_with('[') || line.is_empty() {
                break Ok(Frame::Header(HeaderFrame::from(frame_clauses)));
            }
        };

        // create the consumers
        let mut consumers = Vec::with_capacity(threads.get());
        for _ in 0..threads.get() {
            let c = Consumer::new(r_text.clone(), s_item.clone());
            consumers.push(c);
        }

        // send the header to the channel (to get it back immediately after)
        s_item.send(header).ok();

        // return the parser
        Self {
            stream,
            r_item,
            s_text,
            threads,
            consumers,
            line,
            state: State::Idle,
        }
    }

    pub fn into_underlying_reader(self) -> B {
        self.stream
    }
}

impl<B: BufRead> Iterator for ThreadedReader<B> {
    type Item = Result<Frame, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // poll for parsed frames to return
            match self.r_item.try_recv() {
                // item is found: simply return it
                Ok(Ok(entry)) => return Some(Ok(entry)),
                // error is found: finalize and return it
                Ok(Err(e)) => {
                    self.state = State::Finished;
                    return Some(Err(e));
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
                        panic!("ERR: DISCONNECTED CHANNEL");
                    }
                }
            }

            // depending on the state, do something before polling
            match self.state {
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
                State::Waiting => {
                    std::thread::sleep(Duration::from_micros(1));
                }
                State::Started => {
                    //
                    let mut lines = String::new();
                    let mut l: &str = self.line.trim_start();

                    loop {
                        // store the previous line and process the next line
                        lines.push_str(l);
                        self.line.clear();

                        // read the next line
                        if let Err(e) = self.stream.read_line(&mut self.line) {
                            panic!("ERR: {:?}", e);
                        }

                        // check if we reached the end of the frame
                        l = self.line.trim_start();
                        if l.starts_with('[') {
                            self.s_text.send(Some(lines));
                            break;
                        } else if self.line.is_empty() {
                            self.state = State::AtEof;
                            if !lines.chars().all(|c| c.is_whitespace()) {
                                self.s_text.send(Some(lines));
                            }
                            for _ in 0..self.threads.get() {
                                self.s_text.send(None);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}

impl<B: BufRead> TryFrom<ThreadedReader<B>> for OboDoc {
    type Error = Error;
    fn try_from(mut reader: ThreadedReader<B>) -> Result<Self, Self::Error> {
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

impl<B: BufRead> AsRef<B> for ThreadedReader<B> {
    fn as_ref(&self) -> &B {
        &self.stream
    }
}

impl<B: BufRead> AsMut<B> for ThreadedReader<B> {
    fn as_mut(&mut self) -> &mut B {
        &mut self.stream
    }
}

// ---
