# `fastobo-syntax` [![Star me](https://img.shields.io/github/stars/althonos/fastobo.svg?style=social&label=Star&maxAge=3600)](https://github.com/althonos/fastobo/stargazers)

*PEG Syntax and pest parser for the OBO flat file format 1.4*

[![TravisCI](https://img.shields.io/travis/althonos/fastobo/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/fastobo/branches)
[![Codecov](https://img.shields.io/codecov/c/gh/althonos/fastobo/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/althonos/fastobo)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/tree/master/fastobo-syntax)
[![Crate](https://img.shields.io/crates/v/fastobo-syntax.svg?maxAge=600&style=flat-square)](https://crates.io/crates/fastobo-syntax)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/fastobo/latest/fastobo/parser/)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/fastobo/blob/master/fastobo-syntax/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/althonos/fastobo.svg?style=flat-square)](https://github.com/althonos/fastobo/issues)

## Overview

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

## See also

* [`fastobo`](https://crates.io/crates/fastobo): Abstract Syntax Tree and data structures for the OBO
  format version 1.4
* [`fastobo-py`](https://pypi.org/project/fastobo/): Idiomatic Python bindings to the `fastobo` crate.


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
