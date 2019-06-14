//! A selection of useful traits that exceed the scope of OBO syntax.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ptr::NonNull;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::visit::VisitMut;

mod treat_xrefs;
pub(crate) use self::treat_xrefs::*;

/// The cardinality constraint for a given clause type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cardinality {
    ZeroOrOne,
    One,
    NotOne,
    Any,
}

impl Cardinality {
    pub fn is_match(&self, n: usize) -> bool {
        match self {
            Cardinality::ZeroOrOne => n < 2,
            Cardinality::One => n == 1,
            Cardinality::NotOne => n != 1,
            Cardinality::Any => true,
        }
    }

    pub fn to_error<S: Into<String>>(&self, n: usize, tag: S) -> Option<CardinalityError> {
        use self::CardinalityError::*;
        let name = tag.into();
        match self {
            Cardinality::ZeroOrOne if n > 1 => Some(DuplicateClauses { name }),
            Cardinality::One if n == 0 => Some(MissingClause { name }),
            Cardinality::One if n > 1 => Some(DuplicateClauses { name }),
            Cardinality::NotOne if n == 1 => Some(SingleClause { name }),
            _ => None,
        }
    }
}

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
    fn cardinality_check(&self) -> Result<(), CardinalityError> {
        use std::collections::HashMap;
        use std::mem::discriminant;

        // Group clauses by variant kind
        let mut clause_index: HashMap<_, Vec<&Self::Clause>> = HashMap::new();
        for clause in self.clauses_ref() {
            clause_index
                .entry(clause.tag())
                .or_default()
                .push(clause);
        }

        // Check each variant kind
        for (tag, clauses) in clause_index {
            let cardinality = clauses[0].cardinality();
            if let Some(err) = cardinality.to_error(clauses.len(), tag) {
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
    /// # use fastobo::semantics::OboClause;
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
    /// # use fastobo::semantics::OboClause;
    /// # use fastobo::semantics::Cardinality;
    /// let clause = HeaderClause::SavedBy("Martin Larralde".into());
    /// assert_eq!(clause.cardinality(), Cardinality::ZeroOrOne);
    /// ```
    fn cardinality(&self) -> Cardinality;
}

/// A trait for structs that have an identifier.
pub trait Identified {
    /// Get a reference to the identifier of the entity.
    fn as_id(&self) -> &Ident;

    /// Get a mutable reference to the identifier of the entity.
    fn as_id_mut(&mut self) -> &mut Ident;
}
