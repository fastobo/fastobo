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

    /// Add implicit OBO clauses to the `OboDoc` syntax tree.
    ///
    /// The OBO standard defines some implicit clauses that are automatically
    /// added to all documents, which can be added by this method. The
    /// concerned header clauses are:
    /// - `idspace: RO http://purl.obolibrary.org/obo/RO_`
    /// - `idspace: BFO http://purl.obolibrary.org/obo/BFO_`
    /// - `treat-xrefs-as-equivalent: RO`
    /// - `treat-xrefs-as-equivalent: BFO`
    ///
    /// # Note
    /// After having called this method, the syntax tree will behave as if
    /// these clauses were in the original document. In particular, they will
    /// be written down in case the syntax tree is to be serialized.
    fn preprocess(&mut self);

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
    /// # Note
    /// After processing the document, none of the original `xrefs` will
    /// be removed from the AST.
    ///
    /// # See also
    /// - [Header Macro Translation](http://owlcollab.github.io/oboformat/doc/obo-syntax.html#4.4.2)
    ///   section of the syntax and semantics guide.
    fn process_macros(&mut self);
}


impl OboSemantics for OboDoc {

    fn preprocess(&mut self) {

        let bfo_idspace = HeaderClause::Idspace(
            IdentPrefix::new("BFO"),
            Url::parse("http://purl.obolibrary.org/obo/BFO_").unwrap(),
            None
        );
        if !self.header.contains(&bfo_idspace) {
            self.header.push(bfo_idspace);
        }

        let ro_idspace = HeaderClause::Idspace(
            IdentPrefix::new("RO"),
            Url::parse("http://purl.obolibrary.org/obo/RO_").unwrap(),
            None
        );
        if !self.header.contains(&ro_idspace) {
            self.header.push(ro_idspace);
        }

        let bfo_macro = HeaderClause::TreatXrefsAsEquivalent(IdentPrefix::new("BFO"));
        if !self.header.contains(&bfo_macro) {
            self.header.push(bfo_macro);
        }

        let ro_macro =  HeaderClause::TreatXrefsAsEquivalent(IdentPrefix::new("RO"));
        if !self.header.contains(&ro_macro) {
            self.header.push(ro_macro);
        }
    }

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
        let ns = self.header.default_namespace()?;
        'outer: for entity in &mut self.entities {
            match entity {
                Term(x) => expand!(x, TermClause, ns, 'outer),
                Typedef(x) => expand!(x, TypedefClause, ns, 'outer),
                Instance(x) => expand!(x, InstanceClause, ns, 'outer),
            }
        }

        Ok(())
    }

    fn process_macros(&mut self) {
        use self::HeaderClause::*;
        for clause in self.header.iter_mut() {
            match clause {
                TreatXrefsAsEquivalent(prefix) =>
                    macros::process_treat_xrefs_as_equivalent(&mut self.entities, &prefix),
                _ => (),
            }
        }
    }
}


mod macros {

    use super::*;

    /// Apply a single `treat-xrefs-as-equivalent` macro to the whole document.
    pub fn process_treat_xrefs_as_equivalent(
        entities: &mut Vec<EntityFrame>,
        prefix: &IdentPrefix
    ) {
        // Macro to reduce code duplication
        macro_rules! process {
            ($frame:ident, $clause:ident, $ident:ident) => ({
                let mut new = Vec::new();
                for clause in $frame.clauses() {
                    if let $clause::Xref(xref) = clause.as_ref() {
                        if let Ident::Prefixed(p) = &xref.id {
                            if &p.prefix == prefix {
                                new.push(Line::from(
                                    $clause::EquivalentTo(
                                        $ident::from(xref.id.clone())
                                    )
                                ));
                            }
                        }
                    }
                }
                $frame.clauses_mut().extend(new);
            });
        }

        use self::EntityFrame::*;
        for entity in entities.iter_mut() {
            match entity {
                Term(x) => process!(x, TermClause, ClassIdent),
                Typedef(x) => process!(x, TypedefClause, RelationIdent),
                Instance(_) => (),
            }
        }
    }
}
