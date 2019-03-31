# `fastobo-syntax`

*PEG Syntax and pest parser for the OBO flat file format 1.4*

[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/tree/master/fastobo-syntax)
[![Crate](https://img.shields.io/crates/v/fastobo-syntax.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-syntax)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo/latest/fastobo/parser/)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/blob/master/fastobo-syntax/CHANGELOG.md)


## About

This library is a strict implementation of the [OBO flat file format 1.4](http://owlcollab.github.io/oboformat/doc/obo-syntax.html)
syntax using the [`pest`](https://pest.rs/) parser generator. It was outsourced from [`fastobo`](https://github.com/althonos/fastobo/)
to reduce compilation time, since deriving the `OboParser` from
[`grammar.pest`](https://github.com/althonos/fastobo/blob/master/fastobo-syntax/src/grammar.pest) takes some time.

The parser itself is reexported in [`fastobo::parser`](https://docs.rs/fastobo/latest/fastobo/parser/), so there
is probably no need to depend on this library directly.

## Strictness

The syntax is a strict implementation of the 1.4 format, with the following exceptions:

* `property_value` clauses can have a value with is not quote-enclosed. This is a workaround
  to support some ontology files using `obo2owl` or the `owlapi` to generate their OBO
  counterpart, which does not quote-enclose property values
  ([owlcs/owlapi#833](https://github.com/owlcs/owlapi/pull/833)).
* ISO-8601 datetimes can only be parsed from the canonical format (`ỲYYY-MM-DDTHH:MM:SS`)
  with an optional timezone. Week dates and calendar dates are not supported.