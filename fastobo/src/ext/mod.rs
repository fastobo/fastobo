//! A selection of useful traits that exceed the scope of OBO syntax.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ptr::NonNull;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::visit::VisitMut;

mod treat_xrefs;

// TODO: implement for
// - EntityFrame
// - TermFrame
// - TypedefFrame
// - InstanceFrame
// - HeaderClause
// - TermClause
// - TypedefClause
// - EntityClause
/// A trait for structs that can be serialized in a particular order.
pub trait SerializationOrd {
    fn serialization_cmp(&self, other: &Self) -> Ordering;
}

/// Additional methods for `OboDoc` that can be used to edit the syntax tree.
///
/// The OBO 1.4 semantics are used to process header macros or to add the
/// default OBO namespace to all the frames of the document.
pub trait OboSemantics {
    /// Assign the ontology default namespace to all frames without one.
    ///
    /// This function will not check the cardinality of `namespace` clauses in
    /// entity frames: it will only add a single `namespace` clause to all
    /// frames that have none.
    ///
    /// # Errors
    /// - `CardinalityError::MissingClause`: if the header frame does not
    ///   contain any default namespace definition.
    /// - `CardinalityError::DuplicateClauses` if the header frame does
    ///   contain more than one default namespace definition.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use pretty_assertions::assert_eq;
    /// # use std::str::FromStr;
    /// # use std::string::ToString;
    /// # use fastobo::ast::*;
    /// # use fastobo::ext::OboSemantics;
    /// let mut doc = OboDoc::from_str(
    /// "default-namespace: TST
    ///
    /// [Term]
    /// id: TST:01
    ///
    /// [Term]
    /// id: PATO:0000001
    /// namespace: quality
    /// ").unwrap();
    ///
    /// doc.assign_namespaces().unwrap();
    ///
    /// assert_eq!(doc.to_string(),
    /// "default-namespace: TST
    ///
    /// [Term]
    /// id: TST:01
    /// namespace: TST
    ///
    /// [Term]
    /// id: PATO:0000001
    /// namespace: quality
    /// ");
    ///
    fn assign_namespaces(&mut self) -> Result<(), CardinalityError>;

    /// Process macros in the header frame, adding clauses to relevant entities.
    ///
    /// Header macros are used to expand an ontology by overloading the
    /// actual semantics of  `xref` clauses contained in several entity frames.
    /// In case the translated clauses are already present in the document,
    /// they *won't* be added a second time.
    ///
    /// The following implicit macros will be processed even if they are not
    /// part of the document:
    /// - `treat-xrefs-as-equivalent: RO`
    /// - `treat-xrefs-as-equivalent: BFO`
    ///
    /// # Note
    /// After processing the document, neither the original frame `xrefs`
    /// nor the `treat-xrefs` header clauses will be removed from the AST.
    ///
    /// # See also
    /// - [Header Macro Translation](http://owlcollab.github.io/oboformat/doc/obo-syntax.html#4.4.2)
    ///   section of the syntax and semantics guide.
    fn treat_xrefs(&mut self);
}

/// Additional methods for `OboDoc` that can be used to edit the syntax tree.
impl OboSemantics for OboDoc {
    /// Assign the ontology default namespace to all frames without one.
    fn assign_namespaces(&mut self) -> Result<(), CardinalityError>{

        macro_rules! expand {
            ($frame:ident, $clause:ident, $ns:ident, $outer:lifetime) => ({
                for clause in $frame.iter() {
                    if let $clause::Namespace(_) = clause.as_ref() {
                        continue $outer
                    }
                }
                $frame.push(Line::from($clause::Namespace($ns.clone())));
            });
        }

        use self::EntityFrame::*;

        // Force borrowck to split borrows: we shoudl be able to borrow
        // the header AND the entities at the same time.
        let ns = self.header().default_namespace()?;
        let entities = unsafe {
            &mut *(
                self.entities()
                as *const Vec<EntityFrame>
                as *mut Vec<EntityFrame>
            )
        };

        'outer: for entity in entities {
            match entity {
                Term(x) => expand!(x, TermClause, ns, 'outer),
                Typedef(x) => expand!(x, TypedefClause, ns, 'outer),
                Instance(x) => expand!(x, InstanceClause, ns, 'outer),
            }
        }

        Ok(())
    }

    /// Process macros in the header frame, adding clauses to relevant entities.
    fn treat_xrefs(&mut self) {
        use self::HeaderClause::*;

        // Force borrowck to split borrows: we should be able to mutably
        // borrow the header AND the entities at the same time.
        let entities = unsafe {
            &mut *(
                self.entities()
                as *const Vec<EntityFrame>
                as *mut Vec<EntityFrame>
            )
        };

        // Apply implicit macros for `BFO` and `RO`
        self::treat_xrefs::as_equivalent(entities, &IdentPrefix::new("BFO"));
        self::treat_xrefs::as_equivalent(entities, &IdentPrefix::new("RO"));

        // Apply all `treat-xrefs` macros to the document.
        for clause in self.header() {
            match clause {
                TreatXrefsAsEquivalent(prefix) =>
                    self::treat_xrefs::as_equivalent(
                        entities,
                        &prefix
                    ),
                TreatXrefsAsIsA(prefix) =>
                    self::treat_xrefs::as_is_a(
                        entities,
                        &prefix
                    ),
                TreatXrefsAsHasSubclass(prefix) =>
                    self::treat_xrefs::as_has_subclass(
                        entities,
                        &prefix
                    ),
                TreatXrefsAsGenusDifferentia(prefix, rel, cls) =>
                    self::treat_xrefs::as_genus_differentia(
                        entities,
                        &prefix,
                        &rel,
                        &cls
                    ),
                TreatXrefsAsReverseGenusDifferentia(prefix, rel, cls) =>
                    self::treat_xrefs::as_reverse_genus_differentia(
                        entities,
                        &prefix,
                        &rel,
                        &cls
                    ),
                TreatXrefsAsRelationship(prefix, rel) =>
                    self::treat_xrefs::as_relationship(
                        entities,
                        &prefix,
                        &rel,
                    ),
                _ => (),
            }
        }
    }
}
