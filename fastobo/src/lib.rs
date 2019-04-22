//! *Faultless AST for Open Biomedical Ontologies.*
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/fastobo/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/fastobo/branches)
//! [![Codecov](https://img.shields.io/codecov/c/gh/althonos/fastobo/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/fastobo)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo)
//! [![Crate](https://img.shields.io/crates/v/fastobo.svg?maxAge=600&style=flat-square)](https://crates.io/crates/obofoundry)
//! [![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/blob/master/README.md)
//! [![GitHub issues](https://img.shields.io/github/issues/althonos/fastobo.svg?style=flat-square)](https://github.com/althonos/fastobo/issues)
//!
//! ## Overview
//!
//! This library provides a mostly-complete implementation of the
//! [OBO flat file format 1.4](http://owlcollab.github.io/oboformat/doc/obo-syntax.html).
//!
//! * **Data structures** - `fastobo` provides a complete owned AST for the
//!   OBO language, with constructors and covenience traits where applicable.
//!   There is a plan to provide borrowed data structures as well, to be able
//!   to build a view of an OBO document from borrowed data.
//! * **Parsing** - The parser is implemented using [pest](http://pest.rs/),
//!   and is reexported from the [`fastobo-syntax`](https://crates.io/crates/fastobo-syntax)
//!   crate. Most structures implement the [`FromPair`](./parser/trait.FromPair.html)
//!   trait which allows to build a data structure from a stream of pest tokens.
//! * **Errors** - All functions in that crate that return a `Result` will
//!   always use the `Error` struct defined in the `error` module. Errors
//!   reported by pest are very meaningful, and can give the exact location
//!   of a syntax error encountered by the parser.
//!
//! *Warning: this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
//!  but the API is likely to change a lot before the release of a stable 1.0.*
//!
//!
//! ## Usage
//!
//! Add `fastobo` to the `[dependencies]` sections of your `Cargo.toml` manifest:
//! ```toml
//! [dependencies]
//! fastobo = "0.1.0"
//! ```
//!
//! The [`OboDoc`](./ast/struct.OboDoc.html) struct acts as the root of the AST.
//! Use [`OboDoc::from_stream`](ast/struct.OboDoc.html#method.from_stream) to load
//! an OBO document from a [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)
//! implementor (use [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
//! if needed to adapt from a raw stream):
//!
//! ```rust
//! extern crate fastobo;
//! extern crate reqwest;
//!
//! fn main() {
//!     let response = reqwest::get("http://purl.obolibrary.org/obo/ms.obo").unwrap();
//!     let mut reader = std::io::BufReader::new(response);
//!
//!     match fastobo::ast::OboDoc::from_stream(&mut reader) {
//!         Ok(doc) => println!("Number of MS entities: {}", doc.entities.len()),
//!         Err(e) => panic!("Could not parse the Mass-Spec Ontology: {}", e),
//!     }
//! }
//! ```
//!
//!
//! ## See also
//!
//! * [`fastobo-py`](https://pypi.org/project/fastobo/): Idiomatic Python bindings to this crate.
//!
//!
//! ## Feedback
//!
//! Found a bug ? Have an enhancement request ? Head over to the
//! [GitHub issue tracker](https://github.com/althonos/fastobo/issues) of the project if
//! you need to report or ask something. If you are filling in on a bug, please include as much
//! information as you can about the issue, and try to recreate the same bug in a simple, easily
//! reproducible situation.
//!
//!
//! ## About
//!
//! This project is currently being developed by [Martin Larralde](https://github.com/althonos)
//! as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
//! [Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
//! [Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/).


#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate opaque_typedef_macros;

extern crate fastobo_syntax;
#[cfg(feature = "memchr")]
extern crate memchr;
extern crate opaque_typedef;
extern crate pest;
extern crate url;

#[macro_use]
pub mod parser;

pub mod ast;
pub mod error;
pub mod share;
pub mod visit;
