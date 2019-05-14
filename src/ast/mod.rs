//! Owned syntax tree for the [OBO format version 1.4].
//!
//! [`OboDoc`] is the struct acting as the root of the syntax tree. It can be
//! created from a borrowed string slice with either [`FromStr::from_str`] or
//! [`FromSlice::from_slice`], from a file with [`OboDoc::from_file`], or from
//! a buffered reader with [`OboDoc::from_stream`].
//!
//! [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
//! [`FromSlice::from_slice`]: ../parser/trait.FromSlice.html#method.from_slice
//! [`OboDoc`]: ./struct.OboDoc.html
//! [`OboDoc::from_file`]: ./struct.OboDoc.html#method.from_file
//! [`OboDoc::from_stream`]: ./struct.OboDoc.html#method.from_stream
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.


mod date;
mod doc;
mod header;
mod entity;
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

pub use self::date::*;
pub use self::doc::*;
pub use self::header::*;
pub use self::entity::*;
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

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::OboParser;
use crate::parser::Rule;


/// A trait for structs that can be sorted in an order specified in the OBO spec.
pub trait Orderable {
    /// Sort the elements of the collection in the right serialization order.
    ///
    /// # See Also
    /// - The [Serializer conventions](https://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html#S.3.5)
    ///   section of the OBO Flat File format guide.
    fn sort(&mut self);

    /// Check if the collection is sorted in the right serialization order.
    fn is_sorted(&self) -> bool;
}

/// A trait for structs that have an identifier.
pub trait Identified {
    /// Get a reference to the identifier of the entity.
    fn as_id(&self) -> &Ident;

    /// Get a mutable reference to the identifier of the entity.
    fn as_id_mut(&mut self) -> &mut Ident;
}
