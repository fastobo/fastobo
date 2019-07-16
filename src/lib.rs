#![cfg_attr(feature = "_doc", feature(doc_cfg, external_doc))]
#![cfg_attr(feature = "_doc", doc(include = "../README.md"))]
#![warn(clippy::all)]
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate opaque_typedef_macros;

#[macro_use]
extern crate fastobo_derive_internal;
extern crate fastobo_syntax;

#[cfg(feature = "memchr")]
extern crate memchr;
extern crate opaque_typedef;
extern crate pest;
#[cfg(test)]
extern crate textwrap;
extern crate url;

#[macro_use]
pub mod parser;

pub mod ast;
pub mod error;
pub mod semantics;
pub mod share;
pub mod visit;
