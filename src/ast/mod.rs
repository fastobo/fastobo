//! Owned syntax tree for the [OBO format version 1.4].
//!
//! [`OboDoc`] is the struct acting as the root of the syntax tree. It can be
//! created from a borrowed string slice with either [`FromStr::from_str`] or
//! [`FromSlice::from_slice`], from a file with [`fastobo::from_file`], or from
//! a buffered reader with [`fastobo::from_stream`].
//!
//! # About `FromStr`
//! All types in this module should implement `FromStr` to allow them to be
//! read from their string *serialization*. However, some types are simple
//! wrappers for string types (e.g. [`UnquotedString`] and [`QuotedString`])
//! and can be constructed from their string *value* using the `From<&str>`
//! implementation. Make sure not to confuse how you instantiate these types
//! depending on the content of the string you use.
//!
//! [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
//! [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
//! [`FromSlice::from_slice`]: ../parser/trait.FromSlice.html#method.from_slice
//! [`OboDoc`]: ./struct.OboDoc.html
//! [`QuotedString`]: ./struct.QuotedString.html
//! [`UnquotedString`]: ./struct.UnquotedString.html
//! [`fastobo::from_file`]: ../fn.from_file.html
//! [`fastobo::from_stream`]: ../fn.from_file.html
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.

#[macro_use]
pub(self) mod macros;

mod datetime;
mod doc;
mod entity;
mod frame;
mod header;
mod id;
mod instance;
mod line;
mod pv;
mod qualifier;
mod strings;
mod synonym;
mod term;
mod typedef;
mod xref;

pub use self::datetime::*;
pub use self::doc::*;
pub use self::entity::*;
pub use self::frame::*;
pub use self::header::*;
pub use self::id::*;
pub use self::instance::*;
pub use self::line::*;
pub use self::pv::*;
pub use self::qualifier::*;
pub use self::strings::*;
pub use self::synonym::*;
pub use self::term::*;
pub use self::typedef::*;
pub use self::xref::*;

use super::error::CardinalityError;

/// The inner string type used to store text.
///
/// If `fastobo` is compiled with the `smartstring` feature enabled, then this
/// type will be [`smartstring::SmartString`]. Otherwise, plain [`String`] is
/// used.
///
/// [`smartstring::SmartString`]: https://docs.rs/smartstring/latest/smartstring/struct.SmartString.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
pub type StringType = StringTypeImpl;

#[cfg(feature = "smartstring")]
type StringTypeImpl = smartstring::SmartString<smartstring::Compact>;

#[cfg(not(feature = "smartstring"))]
type StringTypeImpl = String;

/// The inner string type used to store identifiers.
///
/// If `fastobo` is compiled with the `threading` feature enabled, then this
/// type will be [`Arc<str>`]. Otherwise, plain [`Rc<str>`] is used.
///
/// [`Rc<str>`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
/// [`Arc<str>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
pub type IdentType = IdentTypeImpl;

#[cfg(feature = "threading")]
type IdentTypeImpl = std::sync::Arc<str>;

#[cfg(not(feature = "threading"))]
type IdentTypeImpl = std::rc::Rc<str>;
