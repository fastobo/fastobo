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
