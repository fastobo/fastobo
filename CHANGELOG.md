# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]
[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.14.1...HEAD

## [v0.14.1] - 2022-01-11
[v0.14.1]: https://github.com/fastobo/fastobo/compare/v0.14.0...v0.14.1
### Fixed
- Rendering of documentation on `docs.rs`.

## [v0.14.0] - 2022-01-11
[v0.14.0]: https://github.com/fastobo/fastobo/compare/v0.13.2...v0.14.0
### Added
- `IsoDate` and `IsoTime` to manage individuals components of an `IsoDateTime`.
- `CreationDate` to store the date declared in a `creation_date` clause.
- `Display` and `Orderable` trait implementations for `ast::Frame`.
### Changed
- Bump `fastobo-syntax` dependency to `v0.7.1`.
- Use string interning for all identifier types allowing reference counting for common strings.
- `fastobo::parser::FromPair` trait now takes a cache argument for interning strings.
- `XrefList::from_pair` and `QualifierList::from_pair` will now use `Xref::from_pair` and `Qualifier::from_pair` instead of `FromStr::from_str` like before.
### Fixed
- `OboDoc.assign_namespaces` will not raise a `CardinalityError` on documents where all entities already have a namespace clause.
- `OboDoc.sort` will not clone identifiers anymore before comparing entities.

## [v0.13.2] - 2022-01-11
[v0.13.2]: https://github.com/fastobo/fastobo/compare/v0.13.1...v0.13.2
### Fixed
- `fastobo::to_writer` not writing newlines between entity frames.

## [v0.13.1] - 2021-03-30
[v0.13.1]: https://github.com/fastobo/fastobo/compare/v0.13.0...v0.13.1
### Added
- Missing implementation of `fastobo::ast::PrefixedIdent::is_canonical`.
### Fixed
- Curly braces not being escaped from `fastobo::ast::UnquotedString`.

## [v0.13.0] - 2021-02-18
[v0.13.0]: https://github.com/fastobo/fastobo/compare/v0.12.0...v0.13.0
### Changed
- Make `PrefixedIdent` store both ID components contiguously and make it immutable.
- Bumped outdated dependencies.
- Replaced `err-derive` with `thiserror` to derive `std::error::Error` on error types.
### Fixed
- Compilation issues caused by latest version of `syn`.
### Removed
- `fastobo::ast::id::local` module.

## [v0.12.0] - 2020-09-29
[v0.12.0]: https://github.com/fastobo/fastobo/compare/v0.11.2...v0.12.0
### Added
- `fastobo::ast::Url` struct replacing `url::Url` to store URL identifiers.
### Changed
- Removed `url` crate dependency.
- Bumped `fastobo-syntax` dependency to `v0.6.2` to fix parsing of some
  URLs with empty paths lacking a trailing slash (e.g. `http://example.com`).

## [v0.11.2] - 2020-09-04
[v0.11.2]: https://github.com/fastobo/fastobo/compare/v0.11.1...v0.11.2
### Fixed
- Serialization of `SynonymTypedef` header clauses not using the correct tag.
  ([pronto#97](https://github.com/althonos/pronto/issues/97)).

## [v0.11.1] - 2020-08-31
[v0.11.1]: https://github.com/fastobo/fastobo/compare/v0.11.0...v0.11.1
### Changed
- Comparison order of entity frames for consistency with the OBO 1.4
  [serializer conventions](https://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html#S.3.5).

## [v0.11.0] - 2020-08-29
[v0.11.0]: https://github.com/fastobo/fastobo/compare/v0.10.0...v0.11.0
### Added
- Derived `Default` implementation for `QuotedString` and `UnquotedString` types.
- `PropertyValue` variants now have actual corresponding structs
  `fastobo::ast::ResourcePropertyValue` and `fastobo::ast::LiteralPropertyValue`.
- `Definition` struct to store text and xrefs of a definition clause.
- `EntityFrame.definition` (and corresponding method for all entity frames)
  to get the definition of an entity frame if it is unique.
### Changed
- Enum types use boxed fields to reduce the memory footprint of their variant.
- Bumped `fastobo-syntax` dependency to `v0.6.1` to allow any datatype identifier
  for literal property values instead of XML Schema ones previously.
- Made some optional fields of structs heap-allocated to further reduce memory
  consumption.
### Removed
- Dependency on `opaque-typedef`, which was pulling an outdated `syn` version
  in the dependencies and greatly increasing build time.

## [v0.10.0] - 2020-07-24
[v0.10.0]: https://github.com/fastobo/fastobo/compare/v0.9.0...v0.10.0
### Added
- `smartstring` feature to enable using the `smartstring` crate to store
  strings instead of `std::string::String`.
- Additional blanket implementations for `visit::Visit` and `visit::VisitMut`
  traits using the `blanket` crate.
- `fastobo::parser::Parser` to factor common operations on available OBO parsers.
### Changed
- Bumped `fastobo-syntax` dependency to `v0.4.0` to support line comments.
- Moved `fastobo-syntax` re-exports to `fastobo::syntax`.
- Renamed `fastobo::parser` structs.

## [v0.9.0] - 2020-06-14
[v0.9.0]: https://github.com/fastobo/fastobo/compare/v0.8.4...v0.9.0
### Removed
- `fastobo::share` module.
### Changed
- `IdentPrefix` and `IdentLocal` do not store whether they are canonical or
  not to reduce the global memory footprint.

## [v0.8.4] - 2020-06-12
[v0.8.4]: https://github.com/fastobo/fastobo/compare/v0.8.3...v0.8.4
### Changed
- Relax exact version requirement for `pest` dependency.
### Added
- `SequentialParser.into_inner` method allowing to retrieve inner `BufRead`
  from a `SequentialParser` instance.

## [v0.8.3] - 2020-02-12
[v0.8.3]: https://github.com/fastobo/fastobo/compare/v0.8.2...v0.8.3
### Fixed
- `IsoDateTime` formatting in `DD-MM-YYYY` format instead of `YYYY-MM-DD`
  format when serialized to `xsd:datetime`.

## [v0.8.2] - 2020-02-11
[v0.8.2]: https://github.com/fastobo/fastobo/compare/v0.8.1...v0.8.2
### Fixed
- `Display` implementation for the `HeaderClause::Unreserved` variant.

## [v0.8.1] - 2020-01-24
[v0.8.1]: https://github.com/fastobo/fastobo/compare/v0.8.0...v0.8.1
### Added
- [`FrameReader.ordered`](https://docs.rs/fastobo/0.8.1/fastobo/parser/struct.ThreadedReader.html#method.ordered)
  method to make the reader preserve the order of the frames as they appear
  in the source document.
- `TryFrom<&mut FrameReader>` implementation for
  [`OboDoc`](https://docs.rs/fastobo/latest/fastobo/ast/struct.OboDoc.html)
  (allows giving a mutable reference and not only taking ownership of the
  source reader).

## [v0.8.0] - 2020-01-23
[v0.8.0]: https://github.com/fastobo/fastobo/compare/v0.7.5...v0.8.0
### Added
- Parallel `FrameReader` implementation compiled under the `threading`
  feature gate.
### Fixed
- Location of syntax errors not being reported properly in most cases.
### Changed
- Interface of `fastobo::parser::FrameReader` to be more intuitive to use.
- Bumped `fastobo-syntax` to `0.3.8` to support indentation within files.

## [v0.7.5] - 2020-01-18
[v0.7.5]: https://github.com/fastobo/fastobo/compare/v0.7.4...v0.7.5
### Changed
- Bumped `fastobo-syntax` to `0.3.7` to support Windows-style line breaks.

## [v0.7.4] - 2019-10-05
[v0.7.4]: https://github.com/fastobo/fastobo/compare/v0.7.3...v0.7.4
### Changed
- Bumped `fastobo-syntax` to `0.3.6` to fix an edge-case bug
  when parsing a `Synonym`.
### Added
- `IsoDateTime.timezone` accessor to get the underlying `IsoTimeZone`.

## [v0.7.3] - 2019-09-17
[v0.7.3]: https://github.com/fastobo/fastobo/compare/v0.7.2...v0.7.3
### Added
- `FrameReader.as_ref` and `FrameReader.as_mut` to get a reference to
  the underlying `BufRead`.

## [v0.7.2] - 2019-08-27
[v0.7.2]: https://github.com/fastobo/fastobo/compare/v0.7.1...v0.7.2
### Changed
- Bumped `syn` and `quote` to version `1.0` in derive macros.
### Added
- Added BOSC 2019 poster reference to `README.md`.

## [v0.7.1] - 2019-08-08
[v0.7.1]: https://github.com/fastobo/fastobo/compare/v0.7.0...v0.7.1
### Changed
- Updated documentation.

## [v0.7.0] - 2019-08-06
[v0.7.0]: https://github.com/fastobo/fastobo/compare/v0.6.1...v0.7.0
### Added
- `Line.as_inner_mut` method (alias for `Line.as_mut`).
- `OboDoc.is_empty` method to check if an ontology is empty.
- `fastobo::from_file`, `fastobo::from_reader`, `fastobo::from_str`,
  `fastobo::to_writer` and `fastobo::to_file`.
- `AsRef<Ident>` implementation for `Ident`.
### Changed
- Require exact `pest` version `2.1.1` because of unsafe hack to access
  `PestError` fields.
### Removed
- `OboDoc::from_file` and `OboDoc::from_stream` methods (replaced with
  `fastobo::from_file` and `fastobo::from_reader`).
### Fixed
- Serialization of `UnquotedString` not escaping `!` characters.

## [v0.6.1] - 2019-07-24
[v0.6.1]: https://github.com/fastobo/fastobo/compare/v0.6.0...v0.6.1
### Changed
- Bumped internal `url` dependency to `v2.0.0`.

## [v0.6.0] - 2019-07-23
[v0.6.0]: https://github.com/fastobo/fastobo/compare/v0.5.0...v0.6.0
### Added
- `fraction` method to `Iso8601DateTime`.
### Fixed
- `Iso8601DateTime` now support parsing ISO datetimes with fractional seconds.

## [v0.5.0] - 2019-07-15
[v0.5.0]: https://github.com/fastobo/fastobo/compare/v0.4.4...v0.5.0
### Changed
- Transitioned to [`err-derive`](https://docs.rs/err-derive) for error
  management instead of `failure`.

## [v0.4.4] - 2019-07-08
[v0.4.4]: https://github.com/fastobo/fastobo/compare/v0.4.3...v0.4.4
### Added
- `fastobo::visit::IdCompactor` and `fastobo::visit::IdDecompactor` to handle
  url to prefixed ident conversion in OBO documents.

## [v0.4.3] - 2019-06-17
[v0.4.3]: https://github.com/fastobo/fastobo/compare/v0.4.2...v0.4.3
### Fixed
- `HeaderFrame.sort` to avoid shuffling `OwlAxiom` header clauses.
### Added
- `HeaderFrame.merge_owl_axioms` method to merge OWL axioms in a
  header frame.

## [v0.4.2] - 2019-06-13
[v0.4.2]: https://github.com/fastobo/fastobo/compare/v0.4.1...v0.4.2
### Fixed
- `Cardinality::to_error` not returning an error for `ZeroOrOne` clauses
  present twice in a frame.

## [v0.4.1] - 2019-06-13
[v0.4.1]: https://github.com/fastobo/fastobo/compare/v0.4.0...v0.4.1
### Fixed
- `TermClause::PropertyValue` having invalid `ZeroOrOne` cardinality

## [v0.4.0] - 2019-06-12
[v0.4.0]: https://github.com/fastobo/fastobo/compare/v0.3.0...v0.4.0
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

## [v0.3.0] - 2019-05-27
[v0.3.0]: https://github.com/fastobo/fastobo/compare/v0.2.1...v0.3.0
### Changed
- Renamed variants of `PropertyValue` and `PropVal` enums.

## [v0.2.1] - 2019-05-24
[v0.2.1]: https://github.com/fastobo/fastobo/compare/v0.2.0...v0.2.1
### Fixed
- `InstanceFrame::from_pair_unchecked` being implemented but not used in
  `EntityFrame::from_pair_unchecked`, causing a panic when parsing an OBO
  document with instance frames.

## [v0.2.0] - 2019-05-14
[v0.2.0]: https://github.com/fastobo/fastobo/compare/v0.1.1...v0.2.0
### Added
- `Orderable` trait for syntax structs that must be serialized in a
  particular order (e.g. `EntityFrame`, `XrefList`, `OboDoc`, ...).
- `Identified` trait for syntax structs that have an identifier
  (e.g. `EntityFrame`, `Qualifier`, ...).
- Support for `is_asymmetric` typedef clause.
### Fixed
- `Error::IOError` and `Error::ParserError` will now return their inner
  error when calling the [`Fail.cause`](https://docs.rs/failure/0.1.5/failure/trait.Fail.html#method.cause)
  method.

## [v0.1.1] - 2019-05-10
[v0.1.1]: https://github.com/fastobo/fastobo/compare/v0.1.0...v0.1.1
### Added
- [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
  implementation for header clauses, identifiers, `Synonym` and `PropertyValue`.
### Fixed
- Removed missing `docs` feature from `docs.rs` build metadata.
- Changed links to refer to the new outsourced repository
  [`fastobo/fastobo`](https://github.com/fastobo/fastobo).

## [v0.1.0] - 2019-05-08
[v0.1.0]: https://github.com/fastobo/fastobo/compare/40aa9b0...v0.1.0
Initial release.
