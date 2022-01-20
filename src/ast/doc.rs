use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::iter::FromIterator;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::semantics::Identified;
use crate::semantics::Orderable;
use crate::syntax::Rule;

/// A complete OBO document in format version 1.4.
#[derive(Clone, Default, Debug, Hash, Eq, FromStr, PartialEq)]
pub struct OboDoc {
    header: HeaderFrame,
    entities: Vec<EntityFrame>,
}

/// Constructors and builder methods.
///
/// # Parser
/// Use `from_file` to parse a file on the local filesystem, or `from_stream`
/// to parse a `BufRead` implementor (`BufRead` is needed instead of `Read` as
/// the parser is line-based):
/// ```rust
/// # extern crate fastobo;
/// # use std::io::BufReader;
/// # use std::fs::File;
/// # use fastobo::ast::*;
/// let doc1 = fastobo::from_file("tests/data/ms.obo").unwrap();
///
/// // This is equivalent to (but with the file path set in eventual errors):
/// let mut r = BufReader::new(File::open("tests/data/ms.obo").unwrap());
/// let doc2 = fastobo::from_reader(&mut r).unwrap();
///
/// // FIXME: threaded parser may not maintain ordering YET
/// // assert_eq!(doc1, doc2);
/// ```
///
/// # Builder Pattern
/// The builder pattern makes it easy to create an `OboDoc` from an interator
/// of `EntityFrame`, in order to add an `HeaderFrame` after all the entities
/// where collected:
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::*;
/// use std::iter::FromIterator;
///
/// let entities = vec![TermFrame::new(ClassIdent::from(PrefixedIdent::new("TEST", "001")))];
/// let doc = OboDoc::from_iter(entities.into_iter())
///     .and_header(HeaderFrame::from(HeaderClause::FormatVersion(Box::new("1.4".into()))));
/// ```
impl OboDoc {
    /// Create a new empty OBO document.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new OBO document with the provided frame.
    pub fn with_header(header: HeaderFrame) -> Self {
        Self {
            header,
            entities: Default::default(),
        }
    }

    /// Use the provided frame as the header of the OBO document.
    #[must_use]
    pub fn and_header(mut self, header: HeaderFrame) -> Self {
        self.header = header;
        self
    }

    /// Create a new OBO document with the provided entity frames.
    pub fn with_entities(entities: Vec<EntityFrame>) -> Self {
        Self {
            header: Default::default(),
            entities,
        }
    }

    /// Use the provided entity frames as the content of the OBO document.
    #[must_use]
    pub fn and_entities(mut self, entities: Vec<EntityFrame>) -> Self {
        self.entities = entities;
        self
    }
}

/// Shared and mutable getters.
impl OboDoc {
    /// Get a reference to the header of the OBO document.
    pub fn header(&self) -> &HeaderFrame {
        &self.header
    }

    /// Get a mutable reference to the header of the OBO document.
    pub fn header_mut(&mut self) -> &mut HeaderFrame {
        &mut self.header
    }

    /// Get a reference to the entities of the OBO document.
    pub fn entities(&self) -> &Vec<EntityFrame> {
        &self.entities
    }

    /// Get a reference to the entities of the OBO document.
    pub fn entities_mut(&mut self) -> &mut Vec<EntityFrame> {
        &mut self.entities
    }

    /// Check whether or not the document is empty.
    ///
    /// An empty document has no header clauses and no entity frames.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.header().is_empty() && self.entities().is_empty()
    }
}

/// Additional methods for `OboDoc` that can be used to edit the syntax tree.
///
/// The OBO 1.4 semantics are used to process header macros or to add the
/// default OBO namespace to all the frames of the document.
impl OboDoc {
    /// Assign the ontology default namespace to all frames without one.
    ///
    /// This function will not check the cardinality of `namespace` clauses in
    /// entity frames: it will only add a single `namespace` clause to all
    /// frames that have none.
    ///
    /// # Errors
    ///
    /// If all frames already have a `namespace` clause, this function will
    /// not check the contents of the header, return `Ok(())`. However, if
    /// a frame requires the assignment of the default namespace, then a
    /// [`CardinalityError`](../error/enum.CardinalityError.html) may be raised depending on the header contents.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use pretty_assertions::assert_eq;
    /// # use std::str::FromStr;
    /// # use std::string::ToString;
    /// # use fastobo::ast::*;
    /// let mut doc = OboDoc::from_str(
    /// "default-namespace: test
    ///
    /// [Term]
    /// id: TST:01
    ///
    /// [Term]
    /// id: PATO:0000001
    /// namespace: quality
    /// ").unwrap();
    ///
    /// assert_eq!(doc.assign_namespaces().unwrap().to_string(),
    /// "default-namespace: test
    ///
    /// [Term]
    /// id: TST:01
    /// namespace: test
    ///
    /// [Term]
    /// id: PATO:0000001
    /// namespace: quality
    /// ");
    ///
    pub fn assign_namespaces(&mut self) -> Result<(), CardinalityError> {
        macro_rules! expand {
            ($frame:ident, $clause:ident, $ns:ident, $outer:lifetime) => {{
                if !$frame
                    .iter()
                    .any(|clause| matches!(clause.as_ref(), $clause::Namespace(_)))
                {
                    match $ns {
                        Err(e) => return Err(e.clone()),
                        Ok(&ns) => {
                            $frame.push(Line::from($clause::Namespace(Box::new(ns.clone()))))
                        }
                    }
                }
            }};
        }

        use self::EntityFrame::*;

        // Force borrowck to split borrows: we shoudl be able to borrow
        // the header AND the entities at the same time.
        let ns = self.header.default_namespace();
        let ns_ref = ns.as_ref();
        for entity in &mut self.entities {
            match entity {
                Term(x) => expand!(x, TermClause, ns_ref, 'outer),
                Typedef(x) => expand!(x, TypedefClause, ns_ref, 'outer),
                Instance(x) => expand!(x, InstanceClause, ns_ref, 'outer),
            }
        }

        Ok(())
    }

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
    pub fn treat_xrefs(&mut self) {
        use self::HeaderClause::*;

        // Force borrowck to split borrows: we should be able to mutably
        // borrow the header AND the entities at the same time.
        let entities = &mut self.entities;

        // Apply implicit macros for `BFO` and `RO`
        crate::semantics::as_equivalent(entities, &IdentPrefix::new("BFO"));
        crate::semantics::as_equivalent(entities, &IdentPrefix::new("RO"));

        // Apply all `treat-xrefs` macros to the document.
        for clause in &self.header {
            match clause {
                TreatXrefsAsEquivalent(prefix) => crate::semantics::as_equivalent(entities, prefix),
                TreatXrefsAsIsA(prefix) => crate::semantics::as_is_a(entities, prefix),
                TreatXrefsAsHasSubclass(prefix) => {
                    crate::semantics::as_has_subclass(entities, prefix)
                }
                TreatXrefsAsGenusDifferentia(prefix, rel, cls) => {
                    crate::semantics::as_genus_differentia(entities, prefix, rel, cls)
                }
                TreatXrefsAsReverseGenusDifferentia(prefix, rel, cls) => {
                    crate::semantics::as_reverse_genus_differentia(entities, prefix, rel, cls)
                }
                TreatXrefsAsRelationship(prefix, rel) => {
                    crate::semantics::as_relationship(entities, prefix, rel)
                }
                _ => (),
            }
        }
    }

    /// Check if the OBO document is fully labeled.
    ///
    /// An OBO ontology is fully labeled if every frame has exactly one `name`
    /// clause. This is equivalent to the definition in the [OBO specification]
    /// if we suppose an invalid OBO document is never *fully labeled*.
    ///
    /// [OBO specification]: http://owlcollab.github.io/oboformat/doc/obo-syntax.html#6.1.5
    pub fn is_fully_labeled(&self) -> bool {
        self.entities.iter().all(|frame| match frame {
            EntityFrame::Term(f) => f.name().is_ok(),
            EntityFrame::Typedef(f) => f.name().is_ok(),
            EntityFrame::Instance(f) => f.name().is_ok(),
        })
    }
}

impl AsRef<[EntityFrame]> for OboDoc {
    fn as_ref(&self) -> &[EntityFrame] {
        self.entities.as_slice()
    }
}

impl AsRef<Vec<EntityFrame>> for OboDoc {
    fn as_ref(&self) -> &Vec<EntityFrame> {
        &self.entities
    }
}

impl AsMut<Vec<EntityFrame>> for OboDoc {
    fn as_mut(&mut self) -> &mut Vec<EntityFrame> {
        &mut self.entities
    }
}

impl Display for OboDoc {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.header.fmt(f)?;
        if !self.header.is_empty() && !self.entities.is_empty() {
            f.write_char('\n')?;
        }

        let mut entities = self.entities.iter().peekable();
        while let Some(entity) = entities.next() {
            entity.fmt(f)?;
            if entities.peek().is_some() {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl<E> FromIterator<E> for OboDoc
where
    E: Into<EntityFrame>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = E>,
    {
        Self::with_entities(iter.into_iter().map(Into::into).collect())
    }
}

impl Orderable for OboDoc {
    /// Sort the document in the right serialization order.
    fn sort(&mut self) {
        self.header.sort_unstable();
        // FIXME(@althonos): should probably not require cloning here.
        self.entities
            .sort_unstable_by(|e1, e2| e1.as_id().cmp(e2.as_id()));
        for entity in &mut self.entities {
            entity.sort()
        }
    }

    /// Check if the document is sorted in the right serialization order.
    fn is_sorted(&self) -> bool {
        // Check entities are sorted on their identifier.
        for i in 1..self.entities.len() {
            if self.entities[i - 1].as_id() > self.entities[i].as_id() {
                return false;
            }
        }

        // Check every entity is sorted.
        for entity in &self.entities {
            if !entity.is_sorted() {
                return false;
            }
        }

        // Check the header is sorted.
        self.header.is_sorted()
    }
}

impl<'i> FromPair<'i> for OboDoc {
    const RULE: Rule = Rule::OboDoc;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();

        let mut entities = Vec::new();
        let header = HeaderFrame::from_pair_unchecked(inner.next().unwrap(), cache)?;

        let mut pair = inner.next().unwrap();
        while pair.as_rule() != Rule::EOI {
            entities.push(EntityFrame::from_pair_unchecked(pair, cache)?);
            pair = inner.next().unwrap();
        }
        Ok(OboDoc { header, entities })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::iter::FromIterator;
    use std::str::FromStr;

    use pretty_assertions::assert_eq;
    use textwrap_macros::dedent;

    #[test]
    fn from_str() {
        // Empty file should give empty `OboDoc`.
        let doc = OboDoc::from_str("").unwrap();
        self::assert_eq!(doc, Default::default());

        // Empty lines should be ignored.
        let doc = OboDoc::from_str("\n\n").unwrap();
        self::assert_eq!(doc, Default::default());

        // A simple file should parse.
        let doc = OboDoc::from_str(dedent!(
            r#"
            format-version: 1.2

            [Term]
            id: TEST:001
            "#
        ))
        .unwrap();

        let header = HeaderFrame::from_iter(vec![HeaderClause::FormatVersion(Box::new(
            UnquotedString::new("1.2"),
        ))]);
        let term = TermFrame::new(ClassIdent::from(PrefixedIdent::new("TEST", "001")));
        self::assert_eq!(doc, OboDoc::from_iter(Some(term)).and_header(header));
    }

    #[test]
    fn to_string() {
        // Empty `OboDoc` should give empty string.
        let doc = OboDoc::default();
        self::assert_eq!(doc.to_string(), "");

        // `OboDoc` with only header frame should not add newline separator.
        let doc = OboDoc::with_header(HeaderFrame::from(vec![
            HeaderClause::FormatVersion(Box::new(UnquotedString::new("1.2"))),
            HeaderClause::Remark(Box::new(UnquotedString::new("this is a test"))),
        ]));
        self::assert_eq!(
            doc.to_string(),
            dedent!(
                r#"
                format-version: 1.2
                remark: this is a test
                "#
            )
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn is_fully_labeled() {
        let doc = OboDoc::from_str("[Term]\nid: TEST:001\n").unwrap();
        assert!(!doc.is_fully_labeled());

        let doc = OboDoc::from_str("[Term]\nid: TEST:001\nname: test item\n").unwrap();
        assert!(doc.is_fully_labeled());

        let doc = OboDoc::from_str(dedent!(
            r#"
            [Term]
            id: TEST:001
            name: test item

            [Term]
            id: TEST:002
            name: test item two
            "#
        ))
        .unwrap();
        assert!(doc.is_fully_labeled());

        let doc = OboDoc::from_str(dedent!(
            r#"
            [Term]
            id: TEST:001
            name: test item

            [Term]
            id: TEST:002
            "#
        ))
        .unwrap();
        assert!(!doc.is_fully_labeled());

        let doc = OboDoc::from_str(dedent!(
            r#"
            [Term]
            id: TEST:001

            [Term]
            id: TEST:002
            name: test item two
            "#
        ))
        .unwrap();
        assert!(!doc.is_fully_labeled());
    }
}
