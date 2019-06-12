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
mod entity;
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

pub use self::date::*;
pub use self::doc::*;
pub use self::entity::*;
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

#[cfg(feature = "semantics")]
use crate::error::CardinalityError;
use crate::error::Error;
use crate::parser::FromPair;
use crate::parser::OboParser;
use crate::parser::Rule;

/// Common attributes and operations for all frames.
pub trait OboFrame {
    type Clause: OboClause;

    /// Get a vector of references to the clauses of a frame.
    ///
    /// # Note
    /// While currently returning a `Box<Iterator>`, this method will be changed
    /// to return an associated type when [RFC1598] is implemented and available
    /// in *stable* Rust.
    ///
    /// [RFC1598]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
    fn clauses_ref(&self) -> Vec<&Self::Clause>;

    /// Check the frame only contains clauses with the right cardinality.
    ///
    /// # Note
    /// The current implementation does not check for missing clauses: good
    /// ergonomics are to be found to provide a collection of required clauses
    /// in a generic manner.
    #[cfg(feature = "semantics")]
    #[cfg_attr(feature = "_doc", doc(cfg(feature = "semantics")))]
    fn cardinality_check(&self) -> Result<(), CardinalityError> {
        use std::collections::HashMap;
        use std::mem::discriminant;

        // Group clauses by variant kind
        let mut clause_index: HashMap<_, Vec<&Self::Clause>> = HashMap::new();
        for clause in self.clauses_ref() {
            clause_index
                .entry(discriminant(clause))
                .or_default()
                .push(clause);
        }

        // Check each variant kind
        for clauses in clause_index.values() {
            let cardinality = clauses[0].cardinality();
            if let Some(err) = cardinality.to_error(clauses.len(), clauses[0].tag()) {
                return Err(err);
            }
        }

        Ok(())
    }
}

/// Common attributes and operations for all clauses.
pub trait OboClause {
    /// Get the raw string corresponding to the tag of a clause.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let clause = HeaderClause::SavedBy("Martin Larralde".into());
    /// assert_eq!(clause.tag(), "saved-by");
    /// ```
    fn tag(&self) -> &str;

    /// Get the cardinality expected for a clause variant.
    ///
    /// While most clauses can appear any number of time in a frame, some
    /// have a constraint on how many time they can appear: for instance,
    /// a `namespace` clause must appear exactly once in every entity frame,
    /// and an `intersection_of` clause cannot appear only once.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// use fastobo::semantics::Cardinality;
    /// let clause = HeaderClause::SavedBy("Martin Larralde".into());
    /// assert_eq!(clause.cardinality(), Cardinality::ZeroOrOne);
    /// ```
    #[cfg(feature = "semantics")]
    #[cfg_attr(feature = "_doc", doc(cfg(feature = "semantics")))]
    fn cardinality(&self) -> crate::semantics::Cardinality;
}

/// A trait for structs that have an identifier.
pub trait Identified {
    /// Get a reference to the identifier of the entity.
    fn as_id(&self) -> &Ident;

    /// Get a mutable reference to the identifier of the entity.
    fn as_id_mut(&mut self) -> &mut Ident;
}
