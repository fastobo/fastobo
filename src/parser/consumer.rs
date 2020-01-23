use std::io::BufRead;
use std::num::NonZeroUsize;
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::TryRecvError;
use crossbeam_channel::RecvTimeoutError;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::ast::HeaderFrame;
use crate::ast::HeaderClause;
use crate::error::Error;
use crate::error::SyntaxError;

use super::OboParser;
use super::Rule;
use super::FromPair;

// ---

pub struct ConsumerInput {
    text: String,
    line_offset: usize,
    offset: usize,
}

impl ConsumerInput {
    pub fn new(text: String, line_offset: usize, offset: usize) -> Self {
        Self {
            text,
            line_offset,
            offset
        }
    }
}

pub struct Consumer {
    r_text: Receiver< Option< ConsumerInput > >,
    s_item: Sender< Result<Frame, Error> >,
    handle: Option< JoinHandle<()> >
}

impl Consumer {
    pub fn new(
        r_text: Receiver< Option<ConsumerInput> >,
        s_item: Sender< Result<Frame, Error> >,
    ) -> Self {
        Self {
            r_text,
            s_item,
            handle: None,
        }
    }

    pub fn start(&mut self) {
        let s_item = self.s_item.clone();
        let r_text = self.r_text.clone();

        self.handle = Some(std::thread::spawn(move || {
            loop {
                // get the string containing the entire frame
                let msg = loop {
                    match r_text.recv_timeout(Duration::from_micros(1)) {
                        Ok(Some(text)) => break text,
                        Ok(None) => return,
                        Err(RecvTimeoutError::Timeout) => (),
                        Err(RecvTimeoutError::Disconnected) => return,
                    }
                };

                // parse the string
                match OboParser::parse(Rule::EntitySingle, &msg.text) {
                    Ok(mut pairs) => unsafe {
                        let pair = pairs.next().unwrap();
                        let res = EntityFrame::from_pair_unchecked(pair);
                        s_item.send(res.map(Frame::from).map_err(Error::from)).ok();
                    }
                    Err(e) => {
                        let se = SyntaxError::from(e)
                            .with_offsets(msg.line_offset, msg.offset);
                        s_item.send(Err(Error::from(se))).ok();
                        return;
                    }
                }
            }
        }));
    }

    pub fn join(&mut self) -> std::thread::Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }
}
