//! Visitor traits for the OBO syntax tree.
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
//! let doc = fastobo::from_file("tests/data/ms.obo").unwrap();
//! assert!(undeclared_synonym_types(&doc).is_empty());
//! ```
//!
//! # See also
//! * The [Visitor design pattern](https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md)
//!   in [rust-unofficial/patterns](https://github.com/rust-unofficial/patterns).
//!

use std::collections::HashMap;

use blanket::blanket;

use crate::ast::*;
use crate::parser::Cache;
use crate::parser::QuickFind;

// ---------------------------------------------------------------------------

/// Syntax tree traversal to walk a shared borrow of an OBO syntax tree.
///
/// Default implementations of this trait methods can be found in the
/// [`visit`](./visit/index.html) submodule for easy composition.
#[blanket(default = "visit", derive(Mut, Box))]
pub trait Visit<'ast> {
    fn visit_class_ident(&mut self, id: &'ast ClassIdent);
    fn visit_creation_date(&mut self, creation_date: &'ast CreationDate);
    fn visit_definition(&mut self, id: &'ast Definition);
    fn visit_doc(&mut self, doc: &'ast OboDoc);
    fn visit_entity_frame(&mut self, frame: &'ast EntityFrame);
    fn visit_header_clause(&mut self, clause: &'ast HeaderClause);
    fn visit_header_frame(&mut self, header: &'ast HeaderFrame);
    fn visit_ident(&mut self, id: &'ast Ident);
    fn visit_ident_prefix(&mut self, prefix: &'ast IdentPrefix);
    fn visit_import(&mut self, import: &'ast Import);
    fn visit_instance_clause(&mut self, clause: &'ast InstanceClause);
    fn visit_instance_frame(&mut self, frame: &'ast InstanceFrame);
    fn visit_instance_ident(&mut self, id: &'ast InstanceIdent);
    fn visit_iso_date(&mut self, date: &'ast IsoDate);
    fn visit_iso_datetime(&mut self, datetime: &'ast IsoDateTime);
    fn visit_iso_time(&mut self, time: &'ast IsoTime);
    fn visit_literal_property_value(&mut self, id: &'ast LiteralPropertyValue);
    fn visit_naive_date(&mut self, date: &'ast NaiveDateTime);
    fn visit_namespace_ident(&mut self, id: &'ast NamespaceIdent);
    fn visit_property_value(&mut self, pv: &'ast PropertyValue);
    fn visit_prefixed_ident(&mut self, id: &'ast PrefixedIdent);
    fn visit_quoted_string(&mut self, string: &'ast QuotedString);
    fn visit_relation_ident(&mut self, id: &'ast RelationIdent);
    fn visit_resource_property_value(&mut self, id: &'ast ResourcePropertyValue);
    fn visit_subset_ident(&mut self, id: &'ast SubsetIdent);
    fn visit_synonym(&mut self, syn: &'ast Synonym);
    fn visit_synonym_scope(&mut self, scope: &'ast SynonymScope);
    fn visit_synonymtype_ident(&mut self, id: &'ast SynonymTypeIdent);
    fn visit_term_clause(&mut self, clause: &'ast TermClause);
    fn visit_term_frame(&mut self, frame: &'ast TermFrame);
    fn visit_typedef_clause(&mut self, clause: &'ast TypedefClause);
    fn visit_typedef_frame(&mut self, frame: &'ast TypedefFrame);
    fn visit_unprefixed_ident(&mut self, string: &'ast UnprefixedIdent);
    fn visit_unquoted_string(&mut self, string: &'ast UnquotedString);
    fn visit_url(&mut self, url: &'ast Url);
    fn visit_xref(&mut self, xref: &'ast Xref);
    fn visit_xref_list(&mut self, xrefs: &'ast XrefList);
}

/// Default implementation of `Visit` trait methods.
pub mod visit {

    use super::*;

    pub fn visit_class_ident<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, id: &'ast ClassIdent) {
        visitor.visit_ident(id.as_ref())
    }

    pub fn visit_creation_date<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        creation_date: &'ast CreationDate,
    ) {
        use self::CreationDate::*;
        match creation_date {
            Date(ref d) => visitor.visit_iso_date(d.as_ref()),
            DateTime(ref d) => visitor.visit_iso_datetime(d.as_ref()),
        }
    }

    pub fn visit_definition<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, def: &'ast Definition) {
        visitor.visit_quoted_string(def.text());
        visitor.visit_xref_list(def.xrefs());
    }

    pub fn visit_doc<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, doc: &'ast OboDoc) {
        visitor.visit_header_frame(doc.header());
        for frame in doc.entities().iter() {
            visitor.visit_entity_frame(frame)
        }
    }

    pub fn visit_entity_frame<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        frame: &'ast EntityFrame,
    ) {
        use self::EntityFrame::*;
        match frame {
            Term(ref t) => visitor.visit_term_frame(t),
            Typedef(ref t) => visitor.visit_typedef_frame(t),
            Instance(ref i) => visitor.visit_instance_frame(i),
        }
    }

    pub fn visit_header_clause<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        clause: &'ast HeaderClause,
    ) {
        use self::HeaderClause::*;
        match clause {
            FormatVersion(s) => visitor.visit_unquoted_string(s),
            DataVersion(s) => visitor.visit_unquoted_string(s),
            Date(date) => visitor.visit_naive_date(date),
            SavedBy(s) => visitor.visit_unquoted_string(s),
            AutoGeneratedBy(s) => visitor.visit_unquoted_string(s),
            Import(i) => visitor.visit_import(i),
            Subsetdef(id, s) => {
                visitor.visit_subset_ident(id);
                visitor.visit_quoted_string(s);
            }
            SynonymTypedef(ty, s, sc) => {
                visitor.visit_synonymtype_ident(ty);
                visitor.visit_quoted_string(s);
                if let Some(scope) = sc {
                    visitor.visit_synonym_scope(scope);
                }
            }
            DefaultNamespace(ns) => visitor.visit_namespace_ident(ns),
            NamespaceIdRule(r) => visitor.visit_unquoted_string(r),
            Idspace(id, url, d) => {
                visitor.visit_ident_prefix(id);
                visitor.visit_url(url);
                if let Some(desc) = d {
                    visitor.visit_quoted_string(desc);
                }
            }
            TreatXrefsAsEquivalent(pref) => visitor.visit_ident_prefix(pref),
            TreatXrefsAsGenusDifferentia(pref, rid, cid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            TreatXrefsAsReverseGenusDifferentia(pref, rid, cid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            TreatXrefsAsRelationship(pref, rid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
            }
            TreatXrefsAsIsA(pref) => visitor.visit_ident_prefix(pref),
            TreatXrefsAsHasSubclass(pref) => visitor.visit_ident_prefix(pref),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            Remark(s) => visitor.visit_unquoted_string(s),
            Ontology(s) => visitor.visit_unquoted_string(s),
            OwlAxioms(s) => visitor.visit_unquoted_string(s),
            Unreserved(tag, value) => {
                visitor.visit_unquoted_string(tag);
                visitor.visit_unquoted_string(value);
            }
        }
    }

    pub fn visit_header_frame<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        header: &'ast HeaderFrame,
    ) {
        for clause in header.iter() {
            visitor.visit_header_clause(clause)
        }
    }

    pub fn visit_ident<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, id: &'ast Ident) {
        use self::Ident::*;
        match id {
            Prefixed(p) => visitor.visit_prefixed_ident(p),
            Unprefixed(u) => visitor.visit_unprefixed_ident(u),
            Url(u) => visitor.visit_url(u),
        }
    }

    #[allow(unused_variables)]
    pub fn visit_ident_prefix<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        prefix: &'ast IdentPrefix,
    ) {
    }

    pub fn visit_import<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, import: &'ast Import) {
        use self::Import::*;
        match &import {
            Url(url) => visitor.visit_url(url),
            Abbreviated(id) => visitor.visit_ident(id),
        }
    }

    pub fn visit_instance_clause<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        clause: &'ast InstanceClause,
    ) {
        use self::InstanceClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(id) => visitor.visit_namespace_ident(id),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            InstanceOf(id) => visitor.visit_class_ident(id),
            Relationship(r, id) => {
                visitor.visit_relation_ident(r);
                visitor.visit_ident(id);
            }
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_instance_ident(id),
            Consider(id) => visitor.visit_ident(id),
        }
    }

    pub fn visit_instance_frame<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        frame: &'ast InstanceFrame,
    ) {
        visitor.visit_instance_ident(frame.id());
        for clause in frame.iter() {
            visitor.visit_instance_clause(clause);
        }
    }

    pub fn visit_instance_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast InstanceIdent,
    ) {
        visitor.visit_ident(id.as_ref())
    }

    #[allow(unused_variables)]
    pub fn visit_iso_date<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, datetime: &'ast IsoDate) {
    }

    pub fn visit_iso_datetime<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        datetime: &'ast IsoDateTime,
    ) {
        visitor.visit_iso_date(datetime.as_ref());
        visitor.visit_iso_time(datetime.as_ref());
    }

    #[allow(unused_variables)]
    pub fn visit_iso_time<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, time: &'ast IsoTime) {}

    pub fn visit_literal_property_value<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        pv: &'ast LiteralPropertyValue,
    ) {
        visitor.visit_relation_ident(pv.property());
        visitor.visit_quoted_string(pv.literal());
        visitor.visit_ident(pv.datatype());
    }

    #[allow(unused_variables)]
    pub fn visit_naive_date<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        date: &'ast NaiveDateTime,
    ) {
    }

    pub fn visit_namespace_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast NamespaceIdent,
    ) {
        visitor.visit_ident(id.as_ref())
    }

    pub fn visit_property_value<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        pv: &'ast PropertyValue,
    ) {
        use self::PropertyValue::*;
        match &pv {
            Resource(pv) => visitor.visit_resource_property_value(pv),
            Literal(pv) => visitor.visit_literal_property_value(pv),
        }
    }

    #[allow(unused_variables)]
    pub fn visit_prefixed_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast PrefixedIdent,
    ) {
    }

    #[allow(unused_variables)]
    pub fn visit_quoted_string<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        string: &'ast QuotedString,
    ) {
    }

    pub fn visit_relation_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast RelationIdent,
    ) {
        visitor.visit_ident(id.as_ref())
    }

    pub fn visit_resource_property_value<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        pv: &'ast ResourcePropertyValue,
    ) {
        visitor.visit_relation_ident(pv.property());
        visitor.visit_ident(pv.target());
    }

    pub fn visit_subset_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast SubsetIdent,
    ) {
        visitor.visit_ident(id.as_ref())
    }

    pub fn visit_synonym<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, syn: &'ast Synonym) {
        visitor.visit_quoted_string(syn.description());
        visitor.visit_synonym_scope(syn.scope());
        if let Some(id) = syn.ty() {
            visitor.visit_synonymtype_ident(id);
        }
        visitor.visit_xref_list(syn.xrefs())
    }

    #[allow(unused_variables)]
    pub fn visit_synonym_scope<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        scope: &'ast SynonymScope,
    ) {
    }

    pub fn visit_synonymtype_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        id: &'ast SynonymTypeIdent,
    ) {
        visitor.visit_ident(id.as_ref())
    }

    pub fn visit_term_clause<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        clause: &'ast TermClause,
    ) {
        use self::TermClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(ns) => visitor.visit_namespace_ident(ns),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            Builtin(_) => (),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            IsA(id) => visitor.visit_class_ident(id),
            IntersectionOf(rid, cid) => {
                if let Some(ref rel) = rid {
                    visitor.visit_relation_ident(rel);
                }
                visitor.visit_class_ident(cid);
            }
            UnionOf(id) => visitor.visit_class_ident(id),
            EquivalentTo(id) => visitor.visit_class_ident(id),
            DisjointFrom(id) => visitor.visit_class_ident(id),
            Relationship(rid, cid) => {
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_class_ident(id),
            Consider(id) => visitor.visit_class_ident(id),
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
        }
    }

    pub fn visit_term_frame<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        frame: &'ast TermFrame,
    ) {
        visitor.visit_class_ident(frame.id());
        for clause in frame.iter() {
            visitor.visit_term_clause(clause);
        }
    }

    pub fn visit_typedef_clause<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        clause: &'ast TypedefClause,
    ) {
        use self::TypedefClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(ns) => visitor.visit_namespace_ident(ns),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            Domain(id) => visitor.visit_class_ident(id),
            Range(id) => visitor.visit_class_ident(id),
            Builtin(_) => (),

            IsAntiSymmetric(_) => (),
            IsCyclic(_) => (),
            IsReflexive(_) => (),
            IsSymmetric(_) => (),
            IsAsymmetric(_) => (),
            IsTransitive(_) => (),
            IsFunctional(_) => (),
            IsInverseFunctional(_) => (),

            IsA(id) => visitor.visit_relation_ident(id),
            IntersectionOf(id) => visitor.visit_relation_ident(id),
            UnionOf(id) => visitor.visit_relation_ident(id),
            EquivalentTo(id) => visitor.visit_relation_ident(id),
            DisjointFrom(id) => visitor.visit_relation_ident(id),
            InverseOf(id) => visitor.visit_relation_ident(id),
            TransitiveOver(id) => visitor.visit_relation_ident(id),
            EquivalentToChain(r1, r2) | HoldsOverChain(r1, r2) | Relationship(r1, r2) => {
                visitor.visit_relation_ident(r1);
                visitor.visit_relation_ident(r2);
            }
            DisjointOver(id) => visitor.visit_relation_ident(id),
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_relation_ident(id),
            Consider(id) => visitor.visit_ident(id),
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
            ExpandAssertionTo(s, xrefs) | ExpandExpressionTo(s, xrefs) => {
                visitor.visit_quoted_string(s);
                visitor.visit_xref_list(xrefs);
            }

            IsMetadataTag(_) | IsClassLevel(_) => (),
        }
    }

    pub fn visit_typedef_frame<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        frame: &'ast TypedefFrame,
    ) {
        visitor.visit_relation_ident(frame.id());
        for clause in frame.iter() {
            visitor.visit_typedef_clause(clause);
        }
    }

    #[allow(unused_variables)]
    pub fn visit_unprefixed_ident<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        string: &'ast UnprefixedIdent,
    ) {
    }

    #[allow(unused_variables)]
    pub fn visit_unquoted_string<'ast, V: Visit<'ast> + ?Sized>(
        visitor: &mut V,
        string: &'ast UnquotedString,
    ) {
    }

    #[allow(unused_variables)]
    pub fn visit_url<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, url: &'ast Url) {}

    pub fn visit_xref<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, xref: &'ast Xref) {
        visitor.visit_ident(xref.id());
        if let Some(d) = xref.description() {
            visitor.visit_quoted_string(d);
        }
    }

    pub fn visit_xref_list<'ast, V: Visit<'ast> + ?Sized>(visitor: &mut V, xrefs: &'ast XrefList) {
        for xref in xrefs.iter() {
            visitor.visit_xref(xref)
        }
    }
}

// ---------------------------------------------------------------------------

/// Syntax tree traversal to walk a mutable borrow of an OBO syntax tree.
///
/// Default implementations of this trait methods can be found in the
/// [`visit_mut`](./visit_mut/index.html) submodule for easy composition.
#[blanket(default = "visit_mut", derive(Mut, Box))]
pub trait VisitMut {
    fn visit_class_ident(&mut self, id: &mut ClassIdent);
    fn visit_creation_date(&mut self, creation_date: &mut CreationDate);
    fn visit_definition(&mut self, id: &mut Definition);
    fn visit_doc(&mut self, doc: &mut OboDoc);
    fn visit_entity_frame(&mut self, frame: &mut EntityFrame);
    fn visit_header_clause(&mut self, clause: &mut HeaderClause);
    fn visit_header_frame(&mut self, header: &mut HeaderFrame);
    fn visit_ident(&mut self, id: &mut Ident);
    fn visit_ident_prefix(&mut self, prefix: &mut IdentPrefix);
    fn visit_import(&mut self, import: &mut Import);
    fn visit_instance_clause(&mut self, clause: &mut InstanceClause);
    fn visit_instance_frame(&mut self, frame: &mut InstanceFrame);
    fn visit_instance_ident(&mut self, id: &mut InstanceIdent);
    fn visit_iso_date(&mut self, date: &mut IsoDate);
    fn visit_iso_datetime(&mut self, datetime: &mut IsoDateTime);
    fn visit_iso_time(&mut self, time: &mut IsoTime);
    fn visit_literal_property_value(&mut self, id: &mut LiteralPropertyValue);
    fn visit_naive_date(&mut self, date: &mut NaiveDateTime);
    fn visit_namespace_ident(&mut self, id: &mut NamespaceIdent);
    fn visit_property_value(&mut self, pv: &mut PropertyValue);
    fn visit_prefixed_ident(&mut self, id: &mut PrefixedIdent);
    fn visit_quoted_string(&mut self, string: &mut QuotedString);
    fn visit_relation_ident(&mut self, id: &mut RelationIdent);
    fn visit_resource_property_value(&mut self, id: &mut ResourcePropertyValue);
    fn visit_subset_ident(&mut self, id: &mut SubsetIdent);
    fn visit_synonym(&mut self, syn: &mut Synonym);
    fn visit_synonym_scope(&mut self, scope: &mut SynonymScope);
    fn visit_synonymtype_ident(&mut self, id: &mut SynonymTypeIdent);
    fn visit_term_clause(&mut self, clause: &mut TermClause);
    fn visit_term_frame(&mut self, frame: &mut TermFrame);
    fn visit_typedef_clause(&mut self, clause: &mut TypedefClause);
    fn visit_typedef_frame(&mut self, frame: &mut TypedefFrame);
    fn visit_unprefixed_ident(&mut self, string: &mut UnprefixedIdent);
    fn visit_unquoted_string(&mut self, string: &mut UnquotedString);
    fn visit_url(&mut self, url: &mut Url);
    fn visit_xref(&mut self, xref: &mut Xref);
    fn visit_xref_list(&mut self, xrefs: &mut XrefList);
}

/// Default implementation of `VisitMut` trait methods.
pub mod visit_mut {

    use super::*;

    pub fn visit_class_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut ClassIdent) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_creation_date<V: VisitMut + ?Sized>(
        visitor: &mut V,
        creation_date: &mut CreationDate,
    ) {
        use self::CreationDate::*;
        match creation_date {
            Date(d) => visitor.visit_iso_date(d.as_mut()),
            DateTime(dt) => visitor.visit_iso_datetime(dt.as_mut()),
        }
    }

    pub fn visit_definition<V: VisitMut + ?Sized>(visitor: &mut V, def: &mut Definition) {
        visitor.visit_quoted_string(def.text_mut());
        visitor.visit_xref_list(def.xrefs_mut());
    }

    pub fn visit_doc<V: VisitMut + ?Sized>(visitor: &mut V, doc: &mut OboDoc) {
        visitor.visit_header_frame(doc.header_mut());
        for frame in doc.entities_mut().iter_mut() {
            visitor.visit_entity_frame(frame)
        }
    }

    pub fn visit_entity_frame<V: VisitMut + ?Sized>(visitor: &mut V, frame: &mut EntityFrame) {
        use self::EntityFrame::*;
        match frame {
            Term(ref mut t) => visitor.visit_term_frame(t),
            Typedef(ref mut t) => visitor.visit_typedef_frame(t),
            Instance(ref mut i) => visitor.visit_instance_frame(i),
        }
    }

    pub fn visit_header_clause<V: VisitMut + ?Sized>(visitor: &mut V, clause: &mut HeaderClause) {
        use self::HeaderClause::*;
        match clause {
            FormatVersion(s) => visitor.visit_unquoted_string(s),
            DataVersion(s) => visitor.visit_unquoted_string(s),
            Date(date) => visitor.visit_naive_date(date),
            SavedBy(s) => visitor.visit_unquoted_string(s),
            AutoGeneratedBy(s) => visitor.visit_unquoted_string(s),
            Import(i) => visitor.visit_import(i),
            Subsetdef(id, s) => {
                visitor.visit_subset_ident(id);
                visitor.visit_quoted_string(s);
            }
            SynonymTypedef(ty, s, sc) => {
                visitor.visit_synonymtype_ident(ty);
                visitor.visit_quoted_string(s);
                if let Some(ref mut scope) = sc {
                    visitor.visit_synonym_scope(scope);
                }
            }
            DefaultNamespace(ns) => visitor.visit_namespace_ident(ns),
            NamespaceIdRule(r) => visitor.visit_unquoted_string(r),
            Idspace(id, url, d) => {
                visitor.visit_ident_prefix(id);
                visitor.visit_url(url);
                if let Some(ref mut desc) = d {
                    visitor.visit_quoted_string(desc);
                }
            }
            TreatXrefsAsEquivalent(pref) => visitor.visit_ident_prefix(pref),
            TreatXrefsAsGenusDifferentia(pref, rid, cid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            TreatXrefsAsReverseGenusDifferentia(pref, rid, cid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            TreatXrefsAsRelationship(pref, rid) => {
                visitor.visit_ident_prefix(pref);
                visitor.visit_relation_ident(rid);
            }
            TreatXrefsAsIsA(pref) => visitor.visit_ident_prefix(pref),
            TreatXrefsAsHasSubclass(pref) => visitor.visit_ident_prefix(pref),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            Remark(s) => visitor.visit_unquoted_string(s),
            Ontology(s) => visitor.visit_unquoted_string(s),
            OwlAxioms(s) => visitor.visit_unquoted_string(s),
            Unreserved(tag, value) => {
                visitor.visit_unquoted_string(tag);
                visitor.visit_unquoted_string(value);
            }
        }
    }

    pub fn visit_header_frame<V: VisitMut + ?Sized>(visitor: &mut V, header: &mut HeaderFrame) {
        for clause in header.iter_mut() {
            visitor.visit_header_clause(clause)
        }
    }

    pub fn visit_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut Ident) {
        use self::Ident::*;
        match id {
            Prefixed(ref mut p) => visitor.visit_prefixed_ident(p),
            Unprefixed(ref mut u) => visitor.visit_unprefixed_ident(u),
            Url(ref mut u) => visitor.visit_url(u),
        }
    }

    #[allow(unused_variables)]
    pub fn visit_ident_prefix<V: VisitMut + ?Sized>(visitor: &mut V, prefix: &mut IdentPrefix) {}

    pub fn visit_import<V: VisitMut + ?Sized>(visitor: &mut V, import: &mut Import) {
        use self::Import::*;
        match import {
            Url(ref mut url) => visitor.visit_url(url),
            Abbreviated(ref mut id) => visitor.visit_ident(id),
        }
    }

    pub fn visit_instance_clause<V: VisitMut + ?Sized>(
        visitor: &mut V,
        clause: &mut InstanceClause,
    ) {
        use self::InstanceClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(id) => visitor.visit_namespace_ident(id),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            InstanceOf(id) => visitor.visit_class_ident(id),
            Relationship(r, id) => {
                visitor.visit_relation_ident(r);
                visitor.visit_ident(id);
            }
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_instance_ident(id),
            Consider(id) => visitor.visit_ident(id),
        }
    }

    pub fn visit_instance_frame<V: VisitMut + ?Sized>(visitor: &mut V, frame: &mut InstanceFrame) {
        visitor.visit_instance_ident(frame.id_mut());
        for clause in frame.clauses_mut().iter_mut() {
            visitor.visit_instance_clause(clause);
        }
    }

    pub fn visit_instance_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut InstanceIdent) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_iso_datetime<V: VisitMut + ?Sized>(visitor: &mut V, datetime: &mut IsoDateTime) {
        visitor.visit_iso_date(datetime.as_mut());
        visitor.visit_iso_time(datetime.as_mut());
    }

    #[allow(unused_variables)]
    pub fn visit_iso_date<V: VisitMut + ?Sized>(visitor: &mut V, date: &mut IsoDate) {}

    #[allow(unused_variables)]
    pub fn visit_iso_time<V: VisitMut + ?Sized>(visitor: &mut V, date: &mut IsoTime) {}

    pub fn visit_literal_property_value<V: VisitMut + ?Sized>(
        visitor: &mut V,
        pv: &mut LiteralPropertyValue,
    ) {
        visitor.visit_relation_ident(pv.property_mut());
        visitor.visit_quoted_string(pv.literal_mut());
        visitor.visit_ident(pv.datatype_mut());
    }

    #[allow(unused_variables)]
    pub fn visit_naive_date<V: VisitMut + ?Sized>(visitor: &mut V, date: &mut NaiveDateTime) {}

    pub fn visit_namespace_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut NamespaceIdent) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_property_value<V: VisitMut + ?Sized>(visitor: &mut V, pv: &mut PropertyValue) {
        use self::PropertyValue::*;
        match pv {
            Resource(pv) => visitor.visit_resource_property_value(pv),
            Literal(pv) => visitor.visit_literal_property_value(pv),
        }
    }

    #[allow(unused_variables)]
    pub fn visit_prefixed_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut PrefixedIdent) {}

    #[allow(unused_variables)]
    pub fn visit_quoted_string<V: VisitMut + ?Sized>(visitor: &mut V, string: &mut QuotedString) {}

    pub fn visit_relation_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut RelationIdent) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_resource_property_value<V: VisitMut + ?Sized>(
        visitor: &mut V,
        pv: &mut ResourcePropertyValue,
    ) {
        visitor.visit_relation_ident(pv.property_mut());
        visitor.visit_ident(pv.target_mut());
    }

    pub fn visit_subset_ident<V: VisitMut + ?Sized>(visitor: &mut V, id: &mut SubsetIdent) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_synonym<V: VisitMut + ?Sized>(visitor: &mut V, syn: &mut Synonym) {
        visitor.visit_quoted_string(syn.description_mut());
        visitor.visit_synonym_scope(syn.scope_mut());
        if let Some(id) = syn.ty_mut() {
            visitor.visit_synonymtype_ident(id);
        }
        visitor.visit_xref_list(syn.xrefs_mut())
    }

    #[allow(unused_variables)]
    pub fn visit_synonym_scope<V: VisitMut + ?Sized>(visitor: &mut V, scope: &mut SynonymScope) {}

    pub fn visit_synonymtype_ident<V: VisitMut + ?Sized>(
        visitor: &mut V,
        id: &mut SynonymTypeIdent,
    ) {
        visitor.visit_ident(id.as_mut())
    }

    pub fn visit_term_clause<V: VisitMut + ?Sized>(visitor: &mut V, clause: &mut TermClause) {
        use self::TermClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(ns) => visitor.visit_namespace_ident(ns),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            Builtin(_) => (),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            IsA(id) => visitor.visit_class_ident(id),
            IntersectionOf(rid, cid) => {
                if let Some(ref mut rel) = rid {
                    visitor.visit_relation_ident(rel);
                }
                visitor.visit_class_ident(cid);
            }
            UnionOf(id) => visitor.visit_class_ident(id),
            EquivalentTo(id) => visitor.visit_class_ident(id),
            DisjointFrom(id) => visitor.visit_class_ident(id),
            Relationship(rid, cid) => {
                visitor.visit_relation_ident(rid);
                visitor.visit_class_ident(cid);
            }
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_class_ident(id),
            Consider(id) => visitor.visit_class_ident(id),
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
        }
    }

    pub fn visit_term_frame<V: VisitMut + ?Sized>(visitor: &mut V, frame: &mut TermFrame) {
        {
            visitor.visit_class_ident(frame.id_mut());
        }
        for clause in frame.iter_mut() {
            visitor.visit_term_clause(clause);
        }
    }

    pub fn visit_typedef_clause<V: VisitMut + ?Sized>(visitor: &mut V, clause: &mut TypedefClause) {
        use self::TypedefClause::*;
        match clause {
            IsAnonymous(_) => (),
            Name(s) => visitor.visit_unquoted_string(s),
            Namespace(ns) => visitor.visit_namespace_ident(ns),
            AltId(id) => visitor.visit_ident(id),
            Def(def) => visitor.visit_definition(def),
            Comment(s) => visitor.visit_unquoted_string(s),
            Subset(id) => visitor.visit_subset_ident(id),
            Synonym(s) => visitor.visit_synonym(s),
            Xref(x) => visitor.visit_xref(x),
            PropertyValue(pv) => visitor.visit_property_value(pv),
            Domain(id) => visitor.visit_class_ident(id),
            Range(id) => visitor.visit_class_ident(id),
            Builtin(_) => (),

            IsAntiSymmetric(_) => (),
            IsCyclic(_) => (),
            IsReflexive(_) => (),
            IsSymmetric(_) => (),
            IsAsymmetric(_) => (),
            IsTransitive(_) => (),
            IsFunctional(_) => (),
            IsInverseFunctional(_) => (),

            IsA(id) => visitor.visit_relation_ident(id),
            IntersectionOf(id) => visitor.visit_relation_ident(id),
            UnionOf(id) => visitor.visit_relation_ident(id),
            EquivalentTo(id) => visitor.visit_relation_ident(id),
            DisjointFrom(id) => visitor.visit_relation_ident(id),
            InverseOf(id) => visitor.visit_relation_ident(id),
            TransitiveOver(id) => visitor.visit_relation_ident(id),
            EquivalentToChain(r1, r2) | HoldsOverChain(r1, r2) | Relationship(r1, r2) => {
                visitor.visit_relation_ident(r1);
                visitor.visit_relation_ident(r2);
            }
            DisjointOver(id) => visitor.visit_relation_ident(id),
            IsObsolete(_) => (),
            ReplacedBy(id) => visitor.visit_relation_ident(id),
            Consider(id) => visitor.visit_ident(id),
            CreatedBy(s) => visitor.visit_unquoted_string(s),
            CreationDate(dt) => visitor.visit_creation_date(dt),
            ExpandAssertionTo(s, xrefs) | ExpandExpressionTo(s, xrefs) => {
                visitor.visit_quoted_string(s);
                visitor.visit_xref_list(xrefs);
            }

            IsMetadataTag(_) | IsClassLevel(_) => (),
        }
    }

    pub fn visit_typedef_frame<V: VisitMut + ?Sized>(visitor: &mut V, frame: &mut TypedefFrame) {
        {
            visitor.visit_relation_ident(frame.id_mut());
        }
        for clause in frame.iter_mut() {
            visitor.visit_typedef_clause(clause);
        }
    }

    #[allow(unused_variables)]
    pub fn visit_unprefixed_ident<V: VisitMut + ?Sized>(
        visitor: &mut V,
        string: &mut UnprefixedIdent,
    ) {
    }

    #[allow(unused_variables)]
    pub fn visit_unquoted_string<V: VisitMut + ?Sized>(
        visitor: &mut V,
        string: &mut UnquotedString,
    ) {
    }

    #[allow(unused_variables)]
    pub fn visit_url<V: VisitMut + ?Sized>(visitor: &mut V, url: &mut Url) {}

    pub fn visit_xref<V: VisitMut + ?Sized>(visitor: &mut V, xref: &mut Xref) {
        visitor.visit_ident(xref.id_mut());
        if let Some(d) = xref.description_mut() {
            visitor.visit_quoted_string(d);
        }
    }

    pub fn visit_xref_list<V: VisitMut + ?Sized>(visitor: &mut V, xrefs: &mut XrefList) {
        for xref in xrefs.iter_mut() {
            visitor.visit_xref(xref)
        }
    }
}

// ---------------------------------------------------------------------------

/// A visitor that will compact identifiers in an OBO document.
///
/// # Usage
/// The compactor will follow the rules from the OBO specification:
/// * if the document declares an IDSpace prefix `p` that maps to an Url `u`,
///   URL identifiers that can be factorized as `{u}{v}` will be replaced
///   by Prefixed identifiers `{p}:{v}`
/// * if the document does not declare an IDSpace `p'`, URL identifiers that
///   can be factorized as `http://purl.obolibrary.org/obo/{p'}_{id}`
///   will be replaced by Prefixed identifiers `{p'}:{id}`.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use std::str::FromStr;
/// # use std::string::ToString;
/// # use fastobo::visit::*;
/// let mut doc = fastobo::from_str(
/// "[Term]
/// id: http://purl.obolibrary.org/obo/BFO_0000055
/// ").unwrap();
///
/// IdCompactor::new().visit_doc(&mut doc);
/// assert_eq!(doc.to_string(),
/// "[Term]
/// id: BFO:0000055
/// ");
/// ```
///
/// # See also
/// * [IdDecompactor](./struct.IdDecompactor.html) that does the opposite.
#[derive(Clone, Debug, Default)]
pub struct IdCompactor {
    idspaces: HashMap<IdentPrefix, Url>,
    interner: Cache,
}

impl IdCompactor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VisitMut for IdCompactor {
    fn visit_header_frame(&mut self, header: &mut HeaderFrame) {
        // collect all IDSpaces before processing the header
        for clause in header.iter() {
            if let HeaderClause::Idspace(prefix, url, _) = clause {
                self.idspaces
                    .insert(prefix.as_ref().clone(), (**url).clone());
            }
        }

        // process the header as normal
        visit_mut::visit_header_frame(self, header);
    }

    fn visit_ident(&mut self, id: &mut Ident) {
        // the compacted id.
        let mut new: Option<PrefixedIdent> = None;

        if let Ident::Url(ref u) = id {
            // find a prefix from the idspaces declared in the document
            for (prefix, url) in self.idspaces.iter() {
                if u.as_str().starts_with(url.as_str()) {
                    let local = self.interner.intern(&u.as_str()[url.as_str().len()..]);
                    new = Some(PrefixedIdent::new(prefix.clone(), local));
                }
            }
            // if none found, attempt to use the OBO factorisation
            const OBO_URL: &str = "http://purl.obolibrary.org/obo/";
            if new.is_none() {
                if let Some(raw_id) = u.as_str().strip_prefix(OBO_URL) {
                    if let Some(i) = raw_id.quickfind(b'_') {
                        // check we are not using a declared prefix (otherwise
                        // the compaction/expansion would not roundtrip!)
                        // let prefix = IdentPrefix::new(self.intern(&raw_id[..i]));
                        if self.idspaces.get(&raw_id[..i]).is_none() {
                            let prefix = IdentPrefix::new(self.interner.intern(&raw_id[..i]));
                            let local = self.interner.intern(&raw_id[i + 1..]);
                            new = Some(PrefixedIdent::new(prefix, local));
                        }
                    }
                }
            }
        }

        if let Some(new_id) = new {
            *id = Ident::Prefixed(Box::new(new_id));
        }
    }
}

/// A visitor that will decompact identifiers in an OBO document.
///
/// # Usage
/// The decompactor will follow the rules from the OBO specification:
/// * if the document declares an IDSpace prefix `p` that maps to an Url `u`,
///   Prefixed identifiers `{p}:{id}` will be replaced by URLs identifiers
///   `{u}{id}`.
/// * if the document does not declare an IDSpace `p'`, Prefixed identifiers
///   `{p}:{id}` with be replaced by URLs identifiers
///   `http://purl.obolibrary.org/obo/{p'}_{id}`.
///
/// # See also
/// * [IdCompactor](./struct.IdCompactor.html) that does the opposite.
#[derive(Clone, Debug, Default)]
pub struct IdDecompactor {
    idspaces: HashMap<IdentPrefix, Url>,
    interner: Cache,
}

impl IdDecompactor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VisitMut for IdDecompactor {
    fn visit_header_frame(&mut self, header: &mut HeaderFrame) {
        // collect all IDSpaces before processing the header
        for clause in header.iter() {
            if let HeaderClause::Idspace(prefix, url, _) = clause {
                self.idspaces
                    .insert(prefix.as_ref().clone(), (**url).clone());
            }
        }

        // process the header as normal
        visit_mut::visit_header_frame(self, header)
    }

    fn visit_ident(&mut self, id: &mut Ident) {
        // the compacted id.
        let mut new: Option<Url> = None;
        if let Ident::Prefixed(p) = id {
            let new_url = match self.idspaces.get(p.prefix()) {
                Some(base_url) => format!("{}{}", base_url, p.local()),
                None => format!(
                    "http://purl.obolibrary.org/obo/{}_{}",
                    p.prefix(),
                    p.local()
                ),
            };
            new = Url::new(self.interner.intern(&new_url)).ok();
        }

        if let Some(new_id) = new {
            *id = Ident::Url(Box::new(new_id));
        }
    }
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    mod id_decompactor {
        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        use crate::ast::EntityFrame;
        use crate::ast::Ident;
        use crate::ast::OboDoc;

        use crate::ast::TermClause;
        use crate::ast::Url;
        use crate::ast::Xref;

        use super::IdDecompactor;
        use super::VisitMut;

        #[test]
        fn decompact_idspace() {
            let mut doc = OboDoc::from_str(
                "idspace: Wikipedia http://en.wikipedia.org/wiki/

                [Term]
                id: TST:001
                name: Pupil
                xref: Wikipedia:Pupil
                ",
            )
            .unwrap();
            IdDecompactor::new().visit_doc(&mut doc);

            if let Some(EntityFrame::Term(term)) = doc.entities().get(0) {
                let url = Url::from_str("http://en.wikipedia.org/wiki/Pupil").unwrap();
                assert_eq!(
                    term.clauses()[1].as_inner(),
                    &TermClause::Xref(Box::new(Xref::new(url))),
                );
            } else {
                unreachable!()
            }
        }

        #[test]
        fn decompact_obolibrary() {
            let mut doc = OboDoc::from_str(
                "[Term]
                id: MS:1000031
                ",
            )
            .unwrap();
            IdDecompactor::new().visit_doc(&mut doc);

            if let Some(EntityFrame::Term(term)) = doc.entities().get(0) {
                let url = Url::from_str("http://purl.obolibrary.org/obo/MS_1000031").unwrap();
                assert_eq!(term.id().as_inner().as_ref(), &Ident::from(url),);
            } else {
                unreachable!()
            }
        }
    }

    mod id_compactor {

        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        use crate::ast::EntityFrame;
        use crate::ast::Ident;
        use crate::ast::OboDoc;
        use crate::ast::PrefixedIdent;
        use crate::ast::TermClause;
        use crate::ast::Url;
        use crate::ast::Xref;

        use super::IdCompactor;
        use super::VisitMut;

        #[test]
        fn compact_idspace() {
            let mut doc = OboDoc::from_str(
                "idspace: Wikipedia http://en.wikipedia.org/wiki/

                [Term]
                id: TST:001
                name: Pupil
                xref: http://en.wikipedia.org/wiki/Pupil
                ",
            )
            .unwrap();
            IdCompactor::new().visit_doc(&mut doc);

            if let Some(EntityFrame::Term(term)) = doc.entities().get(0) {
                assert_eq!(
                    term.clauses()[1].as_inner(),
                    &TermClause::Xref(Box::new(Xref::new(PrefixedIdent::new(
                        "Wikipedia",
                        "Pupil"
                    )))),
                );
            } else {
                unreachable!()
            }
        }

        #[test]
        fn compact_obolibrary() {
            let mut doc = OboDoc::from_str(
                "[Term]
                id: http://purl.obolibrary.org/obo/MS_1000031
                ",
            )
            .unwrap();
            IdCompactor::new().visit_doc(&mut doc);

            if let Some(EntityFrame::Term(term)) = doc.entities().get(0) {
                assert_eq!(
                    term.id().as_inner().as_ref(),
                    &Ident::from(PrefixedIdent::new("MS", "1000031")),
                );
            } else {
                unreachable!()
            }
        }

        #[test]
        fn compact_no_idspace_override() {
            let mut doc = OboDoc::from_str(
                "idspace: PMC https://www.ncbi.nlm.nih.gov/pmc/articles/PMC

                [Term]
                id: TST:001
                xref: http://purl.obolibrary.org/obo/PMC_2823822
                ",
            )
            .unwrap();
            IdCompactor::new().visit_doc(&mut doc);

            if let Some(EntityFrame::Term(term)) = doc.entities().get(0) {
                let url = Url::from_str("http://purl.obolibrary.org/obo/PMC_2823822").unwrap();
                assert_eq!(
                    term.clauses()[0].as_inner(),
                    &TermClause::Xref(Box::new(Xref::new(url))),
                );
            } else {
                unreachable!()
            }
        }
    }
}
