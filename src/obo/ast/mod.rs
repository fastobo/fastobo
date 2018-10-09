//! OBO syntax tree declaration.
//!
//! # Syntax
//!
//! This syntax tree attempts to be an exact representation of the OBO 1.4
//! syntax. It should roundtrip from and to an OBO file without modifying it,
//! including comments, excepted non-canonical whitespaces and newlines.
//!
//!
//! # Semantics
//!
//! An `OboTree` is obtained after parsing a physical OBO document file.
//! Structural constraints defined on an abstract OBO document do not hold.
//! The AST will not attempt to perform any kind of validation, allowing
//! entity frames with duplicate identifiers, and ignoring clauses cardinality.
//!
//!
//! # Usage
//!
//! ## Deserialization
//! Use the [`FromStr`] trait to deserialize the AST from an OBO file loaded
//! in memory, or [`From<Read>`] to build the AST from a text stream.
//!
//! ## Serialization
//! Use the [`ToString`] trait to serialize the AST to a `String`, or
//! the [`Display`] trait to write it to any [`Write`] implementor.
//!
//!
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`From<Read>`]: https://doc.rust-lang.org/std/convert/trait.From.html
//! [`ToString`]: https://doc.rust-lang.org/std/string/trait.ToString.html
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [`Write`]: https://doc.rust-lang.org/std/fmt/trait.Write.html

#[macro_use]
mod _macros;

mod dbxref;
mod header;
mod id;
mod instance;
mod property;
mod qualifier;
mod scope;
mod term;
mod typedef;

pub use self::dbxref::*;
pub use self::header::*;
pub use self::id::*;
pub use self::instance::*;
pub use self::property::*;
pub use self::qualifier::*;
pub use self::scope::*;
pub use self::term::*;
pub use self::typedef::*;

/// An OBO entity frame.
#[derive(Debug, PartialEq)]
pub enum EntityFrame {
    Term(self::term::TermFrame),
    Typedef(self::typedef::TypedefFrame),
    Instance(self::instance::InstanceFrame),
}

/// The OBO abstract syntax tree.
#[derive(Debug, PartialEq)]
pub struct OboTree {
    pub header: self::header::HeaderFrame,
    pub entities: Vec<EntityFrame>,
}
