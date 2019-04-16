# `fastobo` [![Star me](https://img.shields.io/github/stars/althonos/fastobo.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/fastobo/stargazers)

*Faultless AST for Open Biomedical Ontologies.*

[![TravisCI](https://img.shields.io/travis/althonos/fastobo/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/fastobo/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/fastobo/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/fastobo)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo)
[![Crate](https://img.shields.io/crates/v/fastobo.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/blob/master/fastobo/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/fastobo.svg?style=flat-square)](https://github.com/althonos/fastobo/issues)


## Overview

This library provides an abstract syntax tree for the [OBO flat file format 1.4](http://owlcollab.github.io/oboformat/doc/obo-syntax.html).


## Usage

Add `fastobo` to the `[dependencies]` sections of your `Cargo.toml` manifest:
```toml
[dependencies]
fastobo = "0.1.0"
```

The `OboDoc` struct acts as the root of the AST. Use `OboDoc::from_stream` to load an OBO document
from a [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) implementor
(use [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html) if needed):

```rust
extern crate fastobo;
extern crate reqwest;

fn main() {
    let response = reqwest::get("http://purl.obolibrary.org/obo/go.obo").unwrap();
    let mut reader = std::io::BufReader::new(response);

    match fastobo::ast::OboDoc::from_stream(&mut reader) {
        Ok(doc) => println!("Number of GO entities: {}", doc.entities.len()),
        Err(e) => panic!("Could not parse the Gene Ontology: {}", e),
    }
}
```


## Missing features

* [ ] Support for comment blocks between frames
* [ ] Support for comments in `property_value` clauses in headers.
* [ ] More `std` traits implementation.


## See also

* [`fastobo-syntax`](https://crates.io/crates/fastobo-syntax): Standalone `pest` parser for the OBO
  format version 1.4.
* [`fastobo-py`](https://pypi.org/project/fastobo/): Idiomatic Python bindings to this crate.


## Feedback

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/althonos/fastobo/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.


## About

This project is currently being developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/).
