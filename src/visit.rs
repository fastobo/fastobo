//! Visitor traits for the OBO ast.
//!
//! `Visit` can be used to implement an algorithm that can traverse an OBO
//! AST without editing it, e.g. for validation purposes. This reduces the
//! boilerplate needed for functions that only require to work on specific
//! parts of an OBO document.
//!
//! # Example
//! The following visitor will collect all synonym types of an OBO document
//! and can be used to return a set of references to the undeclared ones:
//! ```rust
//! extern crate fastobo;
//!
//! use std::collections::HashSet;
//! use fastobo::ast::*;
//! use fastobo::visit::Visit;
//!
//! #[derive(Default)]
//! struct SynTypeChecker<'ast> {
//!     declared: HashSet<&'ast SynonymTypeIdent>,
//!     used: HashSet<&'ast SynonymTypeIdent>,
//! }
//!
//! impl<'ast> Visit<'ast> for SynTypeChecker<'ast> {
//!     fn visit_header_clause(&mut self, clause: &'ast HeaderClause) {
//!         if let HeaderClause::SynonymTypedef(ty, _, _) = clause {
//!             self.declared.insert(ty);
//!         }
//!     }
//!
//!     fn visit_synonymtype_ident(&mut self, id: &'ast SynonymTypeIdent) {
//!         self.used.insert(id);
//!     }
//! }
//!
//! pub fn undeclared_synonym_types(doc: &OboDoc) -> HashSet<&SynonymTypeIdent> {
//!     let mut checker = SynTypeChecker::default();
//!     checker.visit_doc(doc);
//!     checker.used.difference(&checker.declared).cloned().collect()
//! }
//!
//! let doc = OboDoc::from_file("tests/data/ms.obo").unwrap();
//! assert!(undeclared_synonym_types(&doc).is_empty());
//! ```
//!
//! # See also
//! * The [Visitor design pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md)
//!   in [rust-unofficial/patterns](https://github.com/rust-unofficial/patterns).
//!

use url::Url;

use crate::ast::*;

/// Syntax tree traversal to walk a shared borrow of an OBO syntax tree.
pub trait Visit<'ast> {

    fn visit_class_ident(&mut self, id: &'ast ClassIdent) {
        self.visit_ident(id.as_ref())
    }

    fn visit_doc(&mut self, doc: &'ast OboDoc) {
        self.visit_header_frame(doc.header());
        for frame in doc.entities().iter() {
            self.visit_entity_frame(frame)
        }
    }

    fn visit_entity_frame(&mut self, frame: &'ast EntityFrame) {
        use self::EntityFrame::*;
        match frame {
            Term(ref t) => self.visit_term_frame(t),
            Typedef(ref t) => self.visit_typedef_frame(t),
            Instance(ref i) => self.visit_instance_frame(i),
        }
    }

    fn visit_header_clause(&mut self, clause: &'ast HeaderClause) {
        use self::HeaderClause::*;
        match clause {
            FormatVersion(s) => self.visit_unquoted_string(s),
            DataVersion(s) => self.visit_unquoted_string(s),
            Date(date) => self.visit_naive_date(date),
            SavedBy(s) => self.visit_unquoted_string(s),
            AutoGeneratedBy(s) => self.visit_unquoted_string(s),
            Import(i) => self.visit_import(i),
            Subsetdef(id, s) => {
                self.visit_subset_ident(id);
                self.visit_quoted_string(s);
            }
            SynonymTypedef(ty, s, sc) => {
                self.visit_synonymtype_ident(ty);
                self.visit_quoted_string(s);
                if let Some(scope) = sc {
                    self.visit_synonym_scope(scope);
                }
            }
            DefaultNamespace(ns) => self.visit_namespace_ident(ns),
            NamespaceIdRule(r) => self.visit_unquoted_string(r),
            Idspace(id, url, d) => {
                self.visit_ident_prefix(id);
                self.visit_url(url);
                if let Some(desc) = d {
                    self.visit_quoted_string(desc);
                }
            }
            TreatXrefsAsEquivalent(pref) => self.visit_ident_prefix(pref),
            TreatXrefsAsGenusDifferentia(pref, rid, cid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            TreatXrefsAsReverseGenusDifferentia(pref, rid, cid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            TreatXrefsAsRelationship(pref, rid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
            }
            TreatXrefsAsIsA(pref) => self.visit_ident_prefix(pref),
            TreatXrefsAsHasSubclass(pref) => self.visit_ident_prefix(pref),
            PropertyValue(pv) => self.visit_property_value(pv),
            Remark(s) => self.visit_unquoted_string(s),
            Ontology(s) => self.visit_unquoted_string(s),
            OwlAxioms(s) => self.visit_unquoted_string(s),
            Unreserved(tag, value) => {
                self.visit_unquoted_string(tag);
                self.visit_unquoted_string(value);
            }
        }
    }

    fn visit_header_frame(&mut self, header: &'ast HeaderFrame) {
        for clause in header.iter() {
            self.visit_header_clause(clause)
        }
    }

    fn visit_ident(&mut self, id: &'ast Ident) {
        use self::Ident::*;
        match id {
            Prefixed(p) => self.visit_prefixed_ident(p),
            Unprefixed(u) => self.visit_unprefixed_ident(u),
            Url(u) => self.visit_url(u),
        }
    }

    #[allow(unused_variables)]
    fn visit_ident_local(&mut self, prefix: &'ast IdentLocal) {}

    #[allow(unused_variables)]
    fn visit_ident_prefix(&mut self, prefix: &'ast IdentPrefix) {}

    fn visit_import(&mut self, import: &'ast Import) {
        use self::Import::*;
        match &import {
            Url(url) => self.visit_url(url),
            Abbreviated(id) => self.visit_ident(id),
        }
    }

    fn visit_instance_clause(&mut self, clause: &'ast InstanceClause) {
        use self::InstanceClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(id) => self.visit_namespace_ident(id),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            PropertyValue(pv) => self.visit_property_value(pv),
            InstanceOf(id) => self.visit_class_ident(id),
            Relationship(r, id) => {
                self.visit_relation_ident(r);
                self.visit_ident(id);
            }
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_instance_ident(id),
            Consider(id) => self.visit_ident(id),
        }
    }

    fn visit_instance_frame(&mut self, frame: &'ast InstanceFrame) {
        self.visit_instance_ident(frame.id());
        for clause in frame.iter() {
            self.visit_instance_clause(clause);
        }
    }

    fn visit_instance_ident(&mut self, id: &'ast InstanceIdent) {
        self.visit_ident(id.as_ref())
    }

    #[allow(unused_variables)]
    fn visit_iso_date(&mut self, date: &'ast IsoDateTime) {}

    #[allow(unused_variables)]
    fn visit_naive_date(&mut self, date: &'ast NaiveDateTime) {}

    fn visit_namespace_ident(&mut self, id: &'ast NamespaceIdent) {
        self.visit_ident(id.as_ref())
    }

    fn visit_property_value(&mut self, pv: &'ast PropertyValue) {
        use self::PropertyValue::*;
        match &pv {
            Identified(relation, value) => {
                self.visit_relation_ident(relation);
                self.visit_ident(value);
            }
            Typed(relation, value, ty) => {
                self.visit_relation_ident(relation);
                self.visit_quoted_string(value);
                self.visit_ident(ty);
            }
        }
    }

    fn visit_prefixed_ident(&mut self, id: &'ast PrefixedIdent) {
        self.visit_ident_prefix(id.prefix());
        self.visit_ident_local(id.local());
    }

    #[allow(unused_variables)]
    fn visit_quoted_string(&mut self, string: &'ast QuotedString) {}

    fn visit_relation_ident(&mut self, id: &'ast RelationIdent) {
        self.visit_ident(id.as_ref())
    }

    fn visit_subset_ident(&mut self, id: &'ast SubsetIdent) {
        self.visit_ident(id.as_ref())
    }

    fn visit_synonym(&mut self, syn: &'ast Synonym) {
        self.visit_quoted_string(syn.description());
        self.visit_synonym_scope(syn.scope());
        if let Some(ref id) = syn.ty() {
            self.visit_synonymtype_ident(id);
        }
        self.visit_xref_list(syn.xrefs())
    }

    #[allow(unused_variables)]
    fn visit_synonym_scope(&mut self, scope: &'ast SynonymScope) {}

    fn visit_synonymtype_ident(&mut self, id: &'ast SynonymTypeIdent) {
        self.visit_ident(id.as_ref())
    }

    fn visit_term_clause(&mut self, clause: &'ast TermClause) {
        use self::TermClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(ns) => self.visit_namespace_ident(ns),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            Builtin(_) => (),
            PropertyValue(pv) => self.visit_property_value(pv),
            IsA(id) => self.visit_class_ident(id),
            IntersectionOf(rid, cid) => {
                if let Some(ref rel) = rid {
                    self.visit_relation_ident(rel);
                }
                self.visit_class_ident(cid);
            }
            UnionOf(id) => self.visit_class_ident(id),
            EquivalentTo(id) => self.visit_class_ident(id),
            DisjointFrom(id) => self.visit_class_ident(id),
            Relationship(rid, cid) => {
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_class_ident(id),
            Consider(id) => self.visit_class_ident(id),
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
        }
    }

    fn visit_term_frame(&mut self, frame: &'ast TermFrame) {
        self.visit_class_ident(frame.id());
        for clause in frame.iter() {
            self.visit_term_clause(clause);
        }
    }

    fn visit_typedef_clause(&mut self, clause: &'ast TypedefClause) {
        use self::TypedefClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(ns) => self.visit_namespace_ident(ns),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            PropertyValue(pv) => self.visit_property_value(pv),
            Domain(id) => self.visit_class_ident(id),
            Range(id) => self.visit_class_ident(id),
            Builtin(_) => (),

            IsAntiSymmetric(_) => (),
            IsCyclic(_) => (),
            IsReflexive(_) => (),
            IsSymmetric(_) => (),
            IsAsymmetric(_) => (),
            IsTransitive(_) => (),
            IsFunctional(_) => (),
            IsInverseFunctional(_) => (),

            IsA(id) => self.visit_relation_ident(id),
            IntersectionOf(id) => self.visit_relation_ident(id),
            UnionOf(id) => self.visit_relation_ident(id),
            EquivalentTo(id) => self.visit_relation_ident(id),
            DisjointFrom(id) => self.visit_relation_ident(id),
            InverseOf(id) => self.visit_relation_ident(id),
            TransitiveOver(id) => self.visit_relation_ident(id),
            EquivalentToChain(r1, r2) | HoldsOverChain(r1, r2) | Relationship(r1, r2) => {
                self.visit_relation_ident(r1);
                self.visit_relation_ident(r2);
            }
            DisjointOver(id) => self.visit_relation_ident(id),
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_relation_ident(id),
            Consider(id) => self.visit_ident(id),
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
            ExpandAssertionTo(s, xrefs) | ExpandExpressionTo(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }

            IsMetadataTag(_) | IsClassLevel(_) => ()
        }
    }

    fn visit_typedef_frame(&mut self, frame: &'ast TypedefFrame) {
        self.visit_relation_ident(frame.id());
        for clause in frame.iter() {
            self.visit_typedef_clause(clause);
        }
    }

    #[allow(unused_variables)]
    fn visit_unprefixed_ident(&mut self, string: &'ast UnprefixedIdent) {}

    #[allow(unused_variables)]
    fn visit_unquoted_string(&mut self, string: &'ast UnquotedString) {}

    #[allow(unused_variables)]
    fn visit_url(&mut self, url: &'ast Url) {}

    fn visit_xref(&mut self, xref: &'ast Xref) {
        self.visit_ident(xref.id());
        if let Some(ref d) = xref.description() {
            self.visit_quoted_string(d);
        }
    }

    fn visit_xref_list(&mut self, xrefs: &'ast XrefList) {
        for xref in xrefs.iter() {
            self.visit_xref(xref)
        }
    }

}


/// Syntax tree traversal to walk a mutable borrow of an OBO syntax tree.
pub trait VisitMut {

    fn visit_class_ident(&mut self, id: &mut ClassIdent) {
        self.visit_ident(id.as_mut())
    }

    fn visit_doc(&mut self, doc: &mut OboDoc) {
        self.visit_header_frame(doc.header_mut());
        for frame in doc.entities_mut().iter_mut() {
            self.visit_entity_frame(frame)
        }
    }

    fn visit_entity_frame(&mut self, frame: &mut EntityFrame) {
        use self::EntityFrame::*;
        match frame {
            Term(ref mut t) => self.visit_term_frame(t),
            Typedef(ref mut t) => self.visit_typedef_frame(t),
            Instance(ref mut i) => self.visit_instance_frame(i),
        }
    }

    fn visit_header_clause(&mut self, clause: &mut HeaderClause) {
        use self::HeaderClause::*;
        match clause {
            FormatVersion(s) => self.visit_unquoted_string(s),
            DataVersion(s) => self.visit_unquoted_string(s),
            Date(date) => self.visit_naive_date(date),
            SavedBy(s) => self.visit_unquoted_string(s),
            AutoGeneratedBy(s) => self.visit_unquoted_string(s),
            Import(i) => self.visit_import(i),
            Subsetdef(id, s) => {
                self.visit_subset_ident(id);
                self.visit_quoted_string(s);
            }
            SynonymTypedef(ty, s, sc) => {
                self.visit_synonymtype_ident(ty);
                self.visit_quoted_string(s);
                if let Some(ref mut scope) = sc {
                    self.visit_synonym_scope(scope);
                }
            }
            DefaultNamespace(ns) => self.visit_namespace_ident(ns),
            NamespaceIdRule(r) => self.visit_unquoted_string(r),
            Idspace(id, url, d) => {
                self.visit_ident_prefix(id);
                self.visit_url(url);
                if let Some(ref mut desc) = d {
                    self.visit_quoted_string(desc);
                }
            }
            TreatXrefsAsEquivalent(pref) => self.visit_ident_prefix(pref),
            TreatXrefsAsGenusDifferentia(pref, rid, cid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            TreatXrefsAsReverseGenusDifferentia(pref, rid, cid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            TreatXrefsAsRelationship(pref, rid) => {
                self.visit_ident_prefix(pref);
                self.visit_relation_ident(rid);
            }
            TreatXrefsAsIsA(pref) => self.visit_ident_prefix(pref),
            TreatXrefsAsHasSubclass(pref) => self.visit_ident_prefix(pref),
            PropertyValue(pv) => self.visit_property_value(pv),
            Remark(s) => self.visit_unquoted_string(s),
            Ontology(s) => self.visit_unquoted_string(s),
            OwlAxioms(s) => self.visit_unquoted_string(s),
            Unreserved(tag, value) => {
                self.visit_unquoted_string(tag);
                self.visit_unquoted_string(value);
            }
        }
    }

    fn visit_header_frame(&mut self, header: &mut HeaderFrame) {
        for clause in header.iter_mut() {
            self.visit_header_clause(clause)
        }
    }

    fn visit_ident(&mut self, id: &mut Ident) {
        use self::Ident::*;
        match id {
            Prefixed(ref mut p) => self.visit_prefixed_ident(p),
            Unprefixed(ref mut u) => self.visit_unprefixed_ident(u),
            Url(ref mut u) => self.visit_url(u),
        }
    }

    #[allow(unused_variables)]
    fn visit_ident_local(&mut self, prefix: &mut IdentLocal) {}

    #[allow(unused_variables)]
    fn visit_ident_prefix(&mut self, prefix: &mut IdentPrefix) {}

    fn visit_import(&mut self, import: &mut Import) {
        use self::Import::*;
        match import {
            Url(ref mut url) => self.visit_url(url),
            Abbreviated(ref mut id) => self.visit_ident(id),
        }
    }

    fn visit_instance_clause(&mut self, clause: &mut InstanceClause) {
        use self::InstanceClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(id) => self.visit_namespace_ident(id),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            PropertyValue(pv) => self.visit_property_value(pv),
            InstanceOf(id) => self.visit_class_ident(id),
            Relationship(r, id) => {
                self.visit_relation_ident(r);
                self.visit_ident(id);
            }
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_instance_ident(id),
            Consider(id) => self.visit_ident(id),
        }
    }

    fn visit_instance_frame(&mut self, frame: &mut InstanceFrame) {
        self.visit_instance_ident(frame.id_mut());
        for clause in frame.clauses_mut().iter_mut() {
            self.visit_instance_clause(clause);
        }
    }

    fn visit_instance_ident(&mut self, id: &mut InstanceIdent) {
        self.visit_ident(id.as_mut())
    }

    #[allow(unused_variables)]
    fn visit_iso_date(&mut self, date: &mut IsoDateTime) {}

    #[allow(unused_variables)]
    fn visit_naive_date(&mut self, date: &mut NaiveDateTime) {}

    fn visit_namespace_ident(&mut self, id: &mut NamespaceIdent) {
        self.visit_ident(id.as_mut())
    }

    fn visit_property_value(&mut self, pv: &mut PropertyValue) {
        use self::PropertyValue::*;
        match pv {
            Identified(ref mut relation, ref mut value) => {
                self.visit_relation_ident(relation);
                self.visit_ident(value);
            }
            Typed(ref mut relation, ref mut value, ref mut ty) => {
                self.visit_relation_ident(relation);
                self.visit_quoted_string(value);
                self.visit_ident(ty);
            }
        }
    }

    fn visit_prefixed_ident(&mut self, id: &mut PrefixedIdent) {
        self.visit_ident_prefix(id.prefix_mut());
        self.visit_ident_local(id.local_mut());
    }

    #[allow(unused_variables)]
    fn visit_quoted_string(&mut self, string: &mut QuotedString) {}

    fn visit_relation_ident(&mut self, id: &mut RelationIdent) {
        self.visit_ident(id.as_mut())
    }

    fn visit_subset_ident(&mut self, id: &mut SubsetIdent) {
        self.visit_ident(id.as_mut())
    }

    fn visit_synonym(&mut self, syn: &mut Synonym) {
        self.visit_quoted_string(syn.description_mut());
        self.visit_synonym_scope(syn.scope_mut());
        if let Some(id) = syn.ty_mut() {
            self.visit_synonymtype_ident(id);
        }
        self.visit_xref_list(syn.xrefs_mut())
    }

    #[allow(unused_variables)]
    fn visit_synonym_scope(&mut self, scope: &mut SynonymScope) {}

    fn visit_synonymtype_ident(&mut self, id: &mut SynonymTypeIdent) {
        self.visit_ident(id.as_mut())
    }

    fn visit_term_clause(&mut self, clause: &mut TermClause) {
        use self::TermClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(ns) => self.visit_namespace_ident(ns),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            Builtin(_) => (),
            PropertyValue(pv) => self.visit_property_value(pv),
            IsA(id) => self.visit_class_ident(id),
            IntersectionOf(rid, cid) => {
                if let Some(ref mut rel) = rid {
                    self.visit_relation_ident(rel);
                }
                self.visit_class_ident(cid);
            }
            UnionOf(id) => self.visit_class_ident(id),
            EquivalentTo(id) => self.visit_class_ident(id),
            DisjointFrom(id) => self.visit_class_ident(id),
            Relationship(rid, cid) => {
                self.visit_relation_ident(rid);
                self.visit_class_ident(cid);
            }
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_class_ident(id),
            Consider(id) => self.visit_class_ident(id),
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
        }
    }

    fn visit_term_frame(&mut self, frame: &mut TermFrame) {
        {
            self.visit_class_ident(frame.id_mut());
        }
        for clause in frame.iter_mut() {
            self.visit_term_clause(clause);
        }
    }

    fn visit_typedef_clause(&mut self, clause: &mut TypedefClause) {
        use self::TypedefClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => self.visit_unquoted_string(s),
            Namespace(ns) => self.visit_namespace_ident(ns),
            AltId(id) => self.visit_ident(id),
            Def(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }
            Comment(s) => self.visit_unquoted_string(s),
            Subset(id) => self.visit_subset_ident(id),
            Synonym(s) => self.visit_synonym(s),
            Xref(x) => self.visit_xref(x),
            PropertyValue(pv) => self.visit_property_value(pv),
            Domain(id) => self.visit_class_ident(id),
            Range(id) => self.visit_class_ident(id),
            Builtin(_) => (),

            IsAntiSymmetric(_) => (),
            IsCyclic(_) => (),
            IsReflexive(_) => (),
            IsSymmetric(_) => (),
            IsAsymmetric(_) => (),
            IsTransitive(_) => (),
            IsFunctional(_) => (),
            IsInverseFunctional(_) => (),

            IsA(id) => self.visit_relation_ident(id),
            IntersectionOf(id) => self.visit_relation_ident(id),
            UnionOf(id) => self.visit_relation_ident(id),
            EquivalentTo(id) => self.visit_relation_ident(id),
            DisjointFrom(id) => self.visit_relation_ident(id),
            InverseOf(id) => self.visit_relation_ident(id),
            TransitiveOver(id) => self.visit_relation_ident(id),
            EquivalentToChain(r1, r2) | HoldsOverChain(r1, r2) | Relationship(r1, r2) => {
                self.visit_relation_ident(r1);
                self.visit_relation_ident(r2);
            }
            DisjointOver(id) => self.visit_relation_ident(id),
            IsObsolete(_) => (),
            ReplacedBy(id) => self.visit_relation_ident(id),
            Consider(id) => self.visit_ident(id),
            CreatedBy(s) => self.visit_unquoted_string(s),
            CreationDate(dt) => self.visit_iso_date(dt),
            ExpandAssertionTo(s, xrefs) | ExpandExpressionTo(s, xrefs) => {
                self.visit_quoted_string(s);
                self.visit_xref_list(xrefs);
            }

            IsMetadataTag(_) | IsClassLevel(_) => ()
        }
    }

    fn visit_typedef_frame(&mut self, frame: &mut TypedefFrame) {
        {
            self.visit_relation_ident(frame.id_mut());
        }
        for clause in frame.iter_mut() {
            self.visit_typedef_clause(clause);
        }
    }

    #[allow(unused_variables)]
    fn visit_unprefixed_ident(&mut self, string: &mut UnprefixedIdent) {}

    #[allow(unused_variables)]
    fn visit_unquoted_string(&mut self, string: &mut UnquotedString) {}

    #[allow(unused_variables)]
    fn visit_url(&mut self, url: &mut Url) {}

    fn visit_xref(&mut self, xref: &mut Xref) {
        self.visit_ident(xref.id_mut());
        if let Some(d) = xref.description_mut() {
            self.visit_quoted_string(d);
        }
    }

    fn visit_xref_list(&mut self, xrefs: &mut XrefList) {
        for xref in xrefs.iter_mut() {
            self.visit_xref(xref)
        }
    }

}
