//! A selection of useful traits that exceed the scope of OBO syntax.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

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
                TreatXrefsAsIsA(prefix) =>
                    macros::process_treat_xrefs_as_is_a(&mut self.entities, &prefix),
                TreatXrefsAsHasSubclass(prefix) =>
                    macros::process_treat_xrefs_as_has_subclass(&mut self.entities, &prefix),
                _ => (),
            }
        }
    }
}


mod macros {

    use super::*;
    use self::EntityFrame::*;

    /// Apply a single `treat-xrefs-as-equivalent` macro to the whole document.
    pub fn process_treat_xrefs_as_equivalent(
        entities: &mut Vec<EntityFrame>,
        prefix: &IdentPrefix
    ) {
        // Macro to reduce code duplication
        macro_rules! process {
            ($frame:ident, $clause:ident, $ident:ident) => ({
                let mut new = Vec::with_capacity($frame.clauses().len());
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
                let clauses = $frame.clauses_mut();
                for new_clause in new.into_iter() {
                    if !clauses.contains(&new_clause) {
                        clauses.push(new_clause);
                    }
                }
            });
        }

        for entity in entities.iter_mut() {
            match entity {
                Term(x) => process!(x, TermClause, ClassIdent),
                Typedef(x) => process!(x, TypedefClause, RelationIdent),
                Instance(_) => (),
            }
        }
    }

    /// Apply a single `treat-xrefs-as-is_a` macro to the whole document.
    pub fn process_treat_xrefs_as_is_a(
        entities: &mut Vec<EntityFrame>,
        prefix: &IdentPrefix
    ) {
        // Macro to reduce code duplication
        macro_rules! process {
            ($frame:ident, $clause:ident, $ident:ident) => ({
                let mut new = Vec::with_capacity($frame.clauses().len());
                for clause in $frame.clauses() {
                    if let $clause::Xref(xref) = clause.as_ref() {
                        if let Ident::Prefixed(p) = &xref.id {
                            if &p.prefix == prefix {
                                new.push(Line::from(
                                    $clause::IsA($ident::from(xref.id.clone()))
                                ));
                            }
                        }
                    }
                }

                let clauses = $frame.clauses_mut();
                for new_clause in new.into_iter() {
                    if !clauses.contains(&new_clause) {
                        clauses.push(new_clause);
                    }
                }
            });
        }

        for entity in entities.iter_mut() {
            match entity {
                Term(x) => process!(x, TermClause, ClassIdent),
                Typedef(x) => process!(x, TypedefClause, RelationIdent),
                Instance(_) => (),
            }
        }
    }

    /// Apply a single `treat-xrefs-as-is_a` macro to the whole document.
    pub fn process_treat_xrefs_as_has_subclass(
        entities: &mut Vec<EntityFrame>,
        prefix: &IdentPrefix
    ) {
        // Collect subclass info into a mapping where `key is_a value`
        macro_rules! collect {
            ($frame:ident, $clause:ident, $ident:ident) => ({
                let mut new: HashMap<Ident, Ident> = HashMap::new();
                for clause in $frame.clauses() {
                    if let $clause::Xref(xref) = clause.as_ref() {
                        if let Ident::Prefixed(p) = &xref.id {
                            if &p.prefix == prefix {
                                new.insert(
                                    $frame.id().clone().into_inner().into(),
                                    xref.id.clone().into(),
                                );
                            }
                        }
                    }
                }
                new
            })
        }

        // Collect a complete map of all `is_a` clauses that must be added.
        let mut subclass_map: HashMap<Ident, HashSet<Ident>> = HashMap::new();
        let mut entities_map: HashMap<Ident, &mut EntityFrame> = HashMap::new();
        for entity in entities.iter_mut() {

            let entity_mapping = match entity {
                Term(x) => collect!(x, TermClause, ClassIdent),
                Typedef(x) => collect!(x, TypedefClause, RelationIdent),
                Instance(x) => collect!(x, InstanceClause, InstanceIdent),
            };

            for (key, value) in entity_mapping.into_iter() {
                subclass_map.entry(key).or_default().insert(value);
            }

            entities_map.insert(entity.id().clone(), entity);
        }

        // Patch all entity frames with the xref id `is_a` clause.
        for (subclass, superclasses) in subclass_map.into_iter() {
            for superclass in superclasses.into_iter() {
                match entities_map.get_mut(&superclass) {
                    Some(Term(ref mut x)) => {
                        x.clauses_mut().push(Line::from(
                            TermClause::IsA(subclass.clone().into())
                        ))
                    }
                    Some(Typedef(ref mut x)) => {
                        x.clauses_mut().push(Line::from(
                            TypedefClause::IsA(subclass.clone().into())
                        ))
                    }
                    _ => ()
                }
            }
        }
    }

}


#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::string::ToString;

    use pretty_assertions::assert_eq;
    use textwrap::dedent;

    use super::*;

    #[test]
    fn process_treat_xrefs_as_equivalent() {
        let mut doc = OboDoc::from_str(&dedent("
            treat-xrefs-as-equivalent: TEST

            [Term]
            id: TEST:001
            xref: TEST:002

            [Term]
            id: TEST:002
        ")).unwrap();

        doc.process_macros();

        self::assert_eq!(dedent("
            treat-xrefs-as-equivalent: TEST

            [Term]
            id: TEST:001
            xref: TEST:002
            equivalent_to: TEST:002

            [Term]
            id: TEST:002
        ").trim_start_matches('\n'), doc.to_string());
    }

    #[test]
    fn process_treat_xrefs_as_is_a() {
        let mut doc = OboDoc::from_str(&dedent("
            treat-xrefs-as-is_a: TEST

            [Term]
            id: TEST:001
            xref: TEST:002

            [Term]
            id: TEST:002
        ")).unwrap();

        doc.process_macros();

        self::assert_eq!(dedent("
            treat-xrefs-as-is_a: TEST

            [Term]
            id: TEST:001
            xref: TEST:002
            is_a: TEST:002

            [Term]
            id: TEST:002
        ").trim_start_matches('\n'), doc.to_string());
    }

    #[test]
    fn process_treat_xrefs_as_has_subclass() {
        let mut doc = OboDoc::from_str(&dedent("
            treat-xrefs-as-has-subclass: TEST

            [Term]
            id: TEST:001
            xref: TEST:002

            [Term]
            id: TEST:002
        ")).unwrap();

        doc.process_macros();

        self::assert_eq!(dedent("
            treat-xrefs-as-has-subclass: TEST

            [Term]
            id: TEST:001
            xref: TEST:002

            [Term]
            id: TEST:002
            is_a: TEST:001
        ").trim_start_matches('\n'), doc.to_string());
    }
}
