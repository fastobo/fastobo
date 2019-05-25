# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [Unreleased]

[Unreleased]: https://github.com/fastobo/fastobo/compare/v0.2.1...HEAD

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
