# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.4.4...HEAD

## [0.4.4] - 2019-07-08

[0.4.4]: https://github.com/fastobo/fastobo/compare/v0.4.3...v0.4.4

## Added
- `fastobo::visit::IdCompactor` and `fastobo::visit::IdDecompactor` to handle
  url to prefixed ident conversion in OBO documents.


## [0.4.3] - 2019-06-17

[0.4.3]: https://github.com/fastobo/fastobo/compare/v0.4.2...v0.4.3

### Fixed
- `HeaderFrame.sort` to avoid shuffling `OwlAxiom` header clauses.

### Added
- `HeaderFrame.merge_owl_axioms` method to merge OWL axioms in a
  header frame.


## [0.4.2] - 2019-06-13

[0.4.2]: https://github.com/fastobo/fastobo/compare/v0.4.1...v0.4.2

### Fixed
- `Cardinality::to_error` not returning an error for `ZeroOrOne` clauses
  present twice in a frame.


## [0.4.1] - 2019-06-13

[0.4.1]: https://github.com/fastobo/fastobo/compare/v0.4.0...v0.4.1

### Fixed
- `TermClause::PropertyValue` having invalid `ZeroOrOne` cardinality


## [0.4.0] - 2019-06-12

[0.4.0]: https://github.com/fastobo/fastobo/compare/v0.3.0...v0.4.0

### Added
- Parsing iterator implementation in `fastobo::parser::FrameReader`
- `fastobo-derive-internal` proc-macros to reduce code boilerplate.
- `OboClause` and `OboFrame` traits with common operations for all
  clauses/frames in an OBO document.
- `Cardinality` enum which can be retrieved from any `OboClause` variant
  with the `cardinality` method.
- `OboDoc.is_fully_labeled` semantic check.

### Changed
- Decomposed errors into smaller errors: the main `Error` now wraps `CardinalityError`,
  `IOError` and `SyntaxError` which can be accessed independently.

### Removed
- Removed `OboSemantics` trait and added functions to `OboDoc` directly.

### Fixed
- Bug causing `XrefList` to be parsed incorrectly when having a comma in their
  quote-enclosed description.


## [0.3.0] - 2019-05-27

[0.3.0]: https://github.com/fastobo/fastobo/compare/v0.2.1...v0.3.0

### Changed
- Renamed variants of `PropertyValue` and `PropVal` enums.


## [0.2.1] - 2019-05-24

[0.2.1]: https://github.com/fastobo/fastobo/compare/v0.2.0...v0.2.1

### Fixed
- `InstanceFrame::from_pair_unchecked` being implemented but not used in
  `EntityFrame::from_pair_unchecked`, causing a panic when parsing an OBO
  document with instance frames.


## [0.2.0] - 2019-05-14

[0.2.0]: https://github.com/fastobo/fastobo/compare/v0.1.1...v0.2.0

### Added
- `Orderable` trait for syntax structs that must be serialized in a
  particular order (e.g. `EntityFrame`, `XrefList`, `OboDoc`, ...).
- `Identified` trait for syntax structs that have an identifier
  (e.g. `EntityFrame`, `Qualifier`, ...).
- Support for `is_asymmetric` typedef clause.

### Fixed
- `Error::IOError` and `Error::ParserError` will now return their inner
  error when calling the [`Fail.cause`] method.

[`Fail.cause`]: https://docs.rs/failure/0.1.5/failure/trait.Fail.html#method.cause


## [0.1.1] - 2019-05-10

[0.1.1]: https://github.com/fastobo/fastobo/compare/v0.1.0...v0.1.1

### Added
- [`PartialOrd`] implementation for header clauses, identifiers, `Synonym`
  and `PropertyValue`.

[`PartialOrd`]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html

### Fixed
- Removed missing `docs` feature from `docs.rs` build metadata.
- Changed links to refer to the new outsourced repository
  [`fastobo/fastobo`](https://github.com/fastobo/fastobo).


## [0.1.0] - 2019-05-08

[0.1.0]: https://github.com/fastobo/fastobo/compare/40aa9b0...v0.1.0

Initial release.
