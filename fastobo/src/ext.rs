//! A selection of useful traits that exceed the scope of OBO syntax.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::visit::VisitMut;

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
}


impl OboSemantics for OboDoc {
    fn assign_namespaces(&mut self) -> Result<(), CardinalityError>{
        let ns = self.header.default_namespace()?;
        'outer: for entity in &mut self.entities {
            match entity {
                EntityFrame::Term(term) => {
                    for clause in term.iter() {
                        if let TermClause::Namespace(_) = clause.as_ref() {
                            continue 'outer
                        }
                    }
                    term.push(Line::from(TermClause::Namespace(ns.clone())));
                }
                EntityFrame::Typedef(ty) => {
                    for clause in ty.iter() {
                        if let TypedefClause::Namespace(_) = clause.as_ref() {
                            continue 'outer
                        }
                    }
                    ty.push(Line::from(TypedefClause::Namespace(ns.clone())));
                }
                EntityFrame::Instance(inst) => {
                    for clause in inst.iter() {
                        if let InstanceClause::Namespace(_) = clause.as_ref() {
                            continue 'outer
                        }
                    }
                    inst.push(Line::from(InstanceClause::Namespace(ns.clone())));
                }
            }
        }
        Ok(())
    }
}
