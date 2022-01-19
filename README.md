# `fastobo` [![Star me](https://img.shields.io/github/stars/fastobo/fastobo.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/fastobo/stargazers)

*Faultless AST for Open Biomedical Ontologies.*

[![Actions](https://img.shields.io/github/workflow/status/fastobo/fastobo/Test?style=flat-square&maxAge=600)](https://github.com/fastobo/fastobo/actions)
[![Codecov](https://img.shields.io/codecov/c/gh/fastobo/fastobo/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/fastobo/fastobo)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo)
[![Crate](https://img.shields.io/crates/v/fastobo.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/fastobo/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/fastobo.svg?style=flat-square)](https://github.com/fastobo/fastobo/issues)
[![DOI](https://img.shields.io/badge/doi-10.7490%2Ff1000research.1117405.1-brightgreen?style=flat-square&maxAge=31536000)](https://f1000research.com/posters/8-1500)


## Overview

This library provides a mostly-complete implementation of the
[OBO flat file format 1.4](http://owlcollab.github.io/oboformat/doc/obo-syntax.html).

* **Data structures** - `fastobo` provides a complete owned AST for the
  OBO language, with constructors and covenience traits where applicable.
  There is a plan to provide borrowed data structures as well, to be able
  to build a view of an OBO document from borrowed data.
* **Parsing** - The parser is implemented using [pest](http://pest.rs/),
  and is reexported from the [`fastobo-syntax`](https://crates.io/crates/fastobo-syntax)
  crate. Most structures implement the [`FromPair`](./parser/trait.FromPair.html)
  trait which allows to build a data structure from a stream of pest tokens.
* **Errors** - All functions in that crate that return a `Result` will
  always use the `Error` struct defined in the `error` module. Errors
  reported by pest are very meaningful, and can give the exact location
  of a syntax error encountered by the parser.
* **Semantics** - This library exports specific methods that can be used
  to edit an OBO syntax tree with the semantics expected in the format
  guide: mapping identifiers to URLs, adding default namespaces, or
  expanding entity frames using `treat-xrefs` macros.

*Warning: this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
but the API is likely to change a lot before the release of a stable 1.0.*


## Features

All the following features are enabled by default, but can be disabled and
cherry-picked using the `default-features = false` option in the `Cargo.toml`
manifest of your project:

* **`memchr`** - Use the [`memchr`](https://docs.rs/memchr/) library to improve
  parser speed when searching for a particular character in a buffer.
* **`threading`** - Use a multi-threaded parser (additionally depending on
  [`crossbeam-channel`](https://docs.rs/crossbeam-channel)), which can greatly
  improve the parser speed if parsing is CPU-bound.
* **`smartstring`** - Use the [`smartstring`](https://docs.rs/smartstring)
  library to reduce heap allocation for identifiers and string data.

## Usage

Add `fastobo` to the `[dependencies]` sections of your `Cargo.toml` manifest:
```toml
[dependencies]
fastobo = "0.13.0"
```

The top-level `fastobo` module provides several functions to parse an `OboDoc`.
Use `fastobo::from_reader` to load an OBO document from a
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html) implementor
(use [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
if needed):

```rust
extern crate fastobo;
extern crate ureq;

fn main() {
    let response = ureq::get("http://purl.obolibrary.org/obo/ms.obo").call();
    let mut reader = std::io::BufReader::new(response.unwrap().into_reader());

    match fastobo::from_reader(&mut reader) {
        Ok(doc) => println!("Number of MS entities: {}", doc.entities().len()),
        Err(e) => panic!("Could not parse the Mass-Spec Ontology: {}", e),
    }
}
```


## See also

* [`fastobo-syntax`](https://crates.io/crates/fastobo-syntax): Standalone `pest` parser
  for the OBO format version 1.4.
* [`fastobo-graphs`](https://crates.io/crates/fastobo-graphs): Data model and `serde`
  implementation of the OBO graphs specification, with conversion traits from and to OBO.
* [`fastobo-py`](https://pypi.org/project/fastobo/): Idiomatic Python bindings to
  this crate.
* [`fastobo-validator`](https://crates.io/crates/fastobo-validator): Standalone CLI
  to validate OBO files against the specification.
* [`horned-functional`](https://crates.io/crates/horned-functional): Parser for
  OWL2 Functional Syntax (can be used to parse `owl-axioms` clauses).


## Feedback

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/fastobo/fastobo/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.


## About

This project was developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/). Cite this project as:

*Larralde M.* **Developing Python and Rust libraries to improve the ontology ecosystem**
*\[version 1; not peer reviewed\].* F1000Research 2019, 8(ISCB Comm J):1500 (poster)
([https://doi.org/10.7490/f1000research.1117405.1](https://doi.org/10.7490/f1000research.1117405.1))
