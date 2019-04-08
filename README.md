# `fastobo`

*Faultless AST for Open Biomedical Ontologies.*

[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo)

## Workspace

### `fastobo-syntax`

This directory contains the Rust crate implementing a `pest` parser for the OBO flat file format 1.4.
It aims at being an accurate translation of the BNF grammar from the OBO Syntax & Semantics draft to
the PEG grammar language used by `pest-derive`. This crate is reexported in `fastobo::parser` so there
is probably no need to use it directly.

### `fastobo`

The Rust implementation of an Abstract Syntax Tree for the OBO language.

### `fastobo-py`

Idiomatic Python bindings to the `fastobo` Rust crate.


## About

This project is currently being developed by [Martin Larralde](https://github.com/althonos)
as part of a Master's Degree internship in the [BBOP team](http://berkeleybop.org/) of the
[Lawrence Berkeley National Laboratory](https://www.lbl.gov/), under the supervision of
[Chris Mungall](http://biosciences.lbl.gov/profiles/chris-mungall/).