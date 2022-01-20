use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::Receiver;
use crossbeam_channel::RecvTimeoutError;
use crossbeam_channel::Sender;

use crate::ast::EntityFrame;
use crate::ast::Frame;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::syntax::Lexer;
use crate::syntax::Rule;

use super::Cache;
use super::FromPair;

// ---

pub struct Input {
    pub text: String,
    pub index: usize,
    pub line_offset: usize,
    pub offset: usize,
}

impl Input {
    pub fn new(text: String, index: usize, line_offset: usize, offset: usize) -> Self {
        Self {
            text,
            index,
            line_offset,
            offset,
        }
    }
}

pub struct Output {
    pub res: Result<Frame, Error>,
    pub index: usize,
}

impl Output {
    pub fn new(res: Result<Frame, Error>, index: usize) -> Self {
        Self { res, index }
    }
}

pub struct Consumer {
    r_text: Receiver<Option<Input>>,
    s_item: Sender<Output>,
    handle: Option<JoinHandle<()>>,
    interner: Arc<Cache>,
}

impl Consumer {
    pub fn new(
        r_text: Receiver<Option<Input>>,
        s_item: Sender<Output>,
        interner: Arc<Cache>,
    ) -> Self {
        Self {
            r_text,
            s_item,
            interner,
            handle: None,
        }
    }

    pub fn start(&mut self) {
        let s_item = self.s_item.clone();
        let r_text = self.r_text.clone();
        let interner = self.interner.clone();

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
                match Lexer::tokenize(Rule::EntitySingle, &msg.text) {
                    Ok(mut pairs) => unsafe {
                        let pair = pairs.next().unwrap();
                        let frame = EntityFrame::from_pair_unchecked(pair, &interner);
                        let res = frame.map(Frame::from).map_err(Error::from);
                        s_item.send(Output::new(res, msg.index)).ok();
                    },
                    Err(e) => {
                        let se = SyntaxError::from(e).with_offsets(msg.line_offset, msg.offset);
                        let res = Err(Error::from(se));
                        s_item.send(Output::new(res, msg.index)).ok();
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
