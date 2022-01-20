#![cfg_attr(feature = "_doc", feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(clippy::all)]
#![allow(clippy::module_inception)]

extern crate blanket;
extern crate fastobo_derive_internal;
extern crate fastobo_syntax;
extern crate ordered_float;
extern crate pest;
extern crate thiserror;

#[cfg(feature = "memchr")]
extern crate memchr;

#[cfg(feature = "threading")]
extern crate crossbeam_channel;
#[cfg(feature = "threading")]
extern crate lazy_static;
#[cfg(feature = "threading")]
extern crate num_cpus;

#[cfg(feature = "smartstring")]
extern crate smartstring;

#[cfg(test)]
extern crate textwrap_macros;

pub mod ast;
pub mod error;
pub mod parser;
pub mod semantics;
pub mod syntax;
pub mod visit;

use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use self::ast::OboDoc;
use self::error::Error;
use self::error::Result;
use self::parser::DefaultParser;
use self::parser::Parser;

// ---------------------------------------------------------------------------

/// Parse an OBO document from a string.
#[inline]
pub fn from_str<S: AsRef<str>>(src: S) -> Result<OboDoc> {
    OboDoc::from_str(src.as_ref()).map_err(Error::from)
}

/// Parse an OBO document from a `BufRead` implementor.
#[inline]
pub fn from_reader<B: BufRead>(r: B) -> Result<OboDoc> {
    OboDoc::try_from(DefaultParser::new(r))
}

/// Parse an OBO document from a file on the local filesystem.
#[inline]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<OboDoc> {
    let pathref = path.as_ref();
    File::open(pathref)
        .map(BufReader::new)
        .map_err(From::from)
        .and_then(from_reader)
        .map_err(|e| {
            if let Error::SyntaxError { error } = e {
                error.with_path(&pathref.to_string_lossy()).into()
            } else {
                e
            }
        })
}

// ---------------------------------------------------------------------------

/// Write an OBO document to a `Write` implementor.
#[inline]
pub fn to_writer<W: Write>(mut writer: W, doc: &OboDoc) -> Result<()> {
    write!(writer, "{}", doc.header())?;
    if !doc.header().is_empty() && !doc.entities().is_empty() {
        writeln!(writer)?;
    }

    let mut entities = doc.entities().iter().peekable();
    while let Some(entity) = entities.next() {
        write!(writer, "{}", entity)?;
        if entities.peek().is_some() {
            writeln!(writer)?;
        }
    }

    Ok(())
}

/// Write an OBO document to a file on the local filesystem.
#[inline]
pub fn to_file<P: AsRef<Path>>(path: P, doc: &OboDoc) -> Result<()> {
    File::create(path)
        .map_err(From::from)
        .and_then(|r| to_writer(r, doc).map_err(From::from))
}
