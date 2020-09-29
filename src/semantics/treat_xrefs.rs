use std::collections::HashMap;
use std::collections::HashSet;

use crate::ast::EntityFrame::*;
use crate::ast::*;

use super::Identified;

/// Apply a single `treat-xrefs-as-equivalent` macro to the whole document.
pub fn as_equivalent(entities: &mut Vec<EntityFrame>, prefix: &IdentPrefix) {
    // Macro to reduce code duplication
    macro_rules! process {
        ($frame:ident, $clause:ident) => {{
            let mut new = Vec::with_capacity($frame.clauses().len());
            for clause in $frame.clauses() {
                if let $clause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            new.push(Line::from($clause::EquivalentTo(Box::new(
                                xref.id().clone().into(),
                            ))));
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
        }};
    }

    for entity in entities.iter_mut() {
        match entity {
            Term(x) => process!(x, TermClause),
            Typedef(x) => process!(x, TypedefClause),
            Instance(_) => (),
        }
    }
}

/// Apply a single `treat-xrefs-as-is_a` macro to the whole document.
pub fn as_is_a(entities: &mut Vec<EntityFrame>, prefix: &IdentPrefix) {
    // Macro to reduce code duplication
    macro_rules! process {
        ($frame:ident, $clause:ident) => {{
            let mut new = Vec::with_capacity($frame.clauses().len());
            for clause in $frame.clauses() {
                if let $clause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            new.push(Line::from($clause::IsA(Box::new(xref.id().clone().into()))));
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
        }};
    }

    for entity in entities.iter_mut() {
        match entity {
            Term(x) => process!(x, TermClause),
            Typedef(x) => process!(x, TypedefClause),
            Instance(_) => (),
        }
    }
}

/// Apply a single `treat-xrefs-as-is_a` macro to the whole document.
pub fn as_has_subclass(entities: &mut Vec<EntityFrame>, prefix: &IdentPrefix) {
    // Collect subclass info into a mapping where `key is_a value`
    macro_rules! collect {
        () => {
            HashMap::new()
        };
        ($frame:ident, $clause:ident) => {{
            let mut new: HashMap<Ident, Ident> = HashMap::new();
            for clause in $frame.clauses() {
                if let $clause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            new.insert(
                                $frame.id().clone().into_inner().into(),
                                xref.id().clone().into(),
                            );
                        }
                    }
                }
            }
            new
        }};
    }

    /// Add `is_a: $supercls` clause to the `$subcls` frame.
    macro_rules! process {
        ($subcls:ident, $supercls:ident, $clause:ident) => {{
            $subcls
                .clauses_mut()
                .push(Line::from($clause::IsA(Box::new($supercls.clone().into()))));
        }};
    }

    // Collect a complete map of all `is_a` clauses that must be added.
    let mut subclass_map: HashMap<Ident, HashSet<Ident>> = HashMap::new();
    let mut entities_map: HashMap<Ident, &mut EntityFrame> = HashMap::new();
    for entity in entities.iter_mut() {
        let entity_mapping = match entity {
            Term(x) => collect!(x, TermClause),
            Typedef(x) => collect!(x, TypedefClause),
            Instance(_) => collect!(),
        };

        for (key, value) in entity_mapping.into_iter() {
            subclass_map.entry(key).or_default().insert(value);
        }

        entities_map.insert(entity.as_id().clone(), entity);
    }

    // Patch all entity frames with the xref id `is_a` clause.
    for (superclass, subclasses) in subclass_map.into_iter() {
        for subclass in subclasses.into_iter() {
            match entities_map.get_mut(&subclass) {
                Some(Term(ref mut x)) => process!(x, superclass, TermClause),
                Some(Typedef(ref mut x)) => process!(x, superclass, TypedefClause),
                _ => (),
            }
        }
    }
}

/// Apply a single `treat-xrefs-as-genus-differentia` macro to the whole document.
pub fn as_genus_differentia(
    entities: &mut Vec<EntityFrame>,
    prefix: &IdentPrefix,
    relid: &RelationIdent,
    classid: &ClassIdent,
) {
    for entity in entities.iter_mut() {
        if let Term(x) = entity {
            // Collect xrefs with the appropriate prefix
            let mut has_intersection_of = false;
            let mut new = Vec::with_capacity(x.clauses().len());
            for clause in x.clauses() {
                if let TermClause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            // add genus from Xref
                            new.push(Line::from(TermClause::IntersectionOf(
                                None,
                                Box::new(xref.id().clone().into()),
                            )));
                            // add differentia from header
                            new.push(Line::from(TermClause::IntersectionOf(
                                Some(Box::new(relid.clone())),
                                Box::new(classid.clone()),
                            )));
                        }
                    }
                } else if let TermClause::IntersectionOf(_, _) = clause.as_ref() {
                    has_intersection_of = true;
                }
            }
            // Apply the genus-differentia clause
            // if the frame has no `intersection_of`
            if !has_intersection_of {
                let clauses = x.clauses_mut();
                for new_clause in new.into_iter() {
                    if !clauses.contains(&new_clause) {
                        clauses.push(new_clause);
                    }
                }
            }
        }
    }
}

/// Apply a single `treat-xrefs-as-reverse-genus-differentia` macro to the whole document.
pub fn as_reverse_genus_differentia(
    entities: &mut Vec<EntityFrame>,
    prefix: &IdentPrefix,
    relid: &RelationIdent,
    classid: &ClassIdent,
) {
    /// Collect genus info into a mapping where `value := intersection_of key`
    macro_rules! collect {
        () => {
            HashMap::new()
        };
        ($frame:ident, $clause:ident) => {{
            let mut new: HashMap<Ident, Ident> = HashMap::new();
            for clause in $frame.clauses() {
                if let $clause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            new.insert(
                                $frame.id().clone().into_inner().into(),
                                xref.id().clone().into(),
                            );
                        }
                    }
                }
            }
            new
        }};
    }

    /// Add genus-differentia to the other frame.
    macro_rules! process {
        ($frame:ident, $genus:ident, $clause:ident) => {{
            let clauses = $frame.clauses_mut();
            clauses.push(Line::from($clause::IntersectionOf(
                None,
                Box::new($genus.clone().into()),
            )));
            clauses.push(Line::from($clause::IntersectionOf(
                Some(Box::new(relid.clone())),
                Box::new(classid.clone()),
            )));
        }};
    }

    // Collect a complete map of all `is_a` clauses that must be added.
    let mut subclass_map: HashMap<Ident, HashSet<Ident>> = HashMap::new();
    let mut entities_map: HashMap<Ident, &mut EntityFrame> = HashMap::new();
    for entity in entities.iter_mut() {
        let entity_mapping = match entity {
            Term(x) => collect!(x, TermClause),
            Typedef(_) => collect!(),
            Instance(_) => collect!(),
        };

        for (key, value) in entity_mapping.into_iter() {
            subclass_map.entry(key).or_default().insert(value);
        }

        entities_map.insert(entity.as_id().clone(), entity);
    }

    // Patch all entity frames with the right `intersection_of` clauses.
    for (genus, classes) in subclass_map.into_iter() {
        for cls in classes.into_iter() {
            if let Some(Term(ref mut x)) = entities_map.get_mut(&cls) {
                process!(x, genus, TermClause)
            }
        }
    }
}

/// Apply a single `treat-xrefs-as-relationship` macro to the whole document.
pub fn as_relationship(
    entities: &mut Vec<EntityFrame>,
    prefix: &IdentPrefix,
    relid: &RelationIdent,
) {
    // Macro to reduce code duplication
    macro_rules! process {
        ($frame:ident, $clause:ident) => {{
            let mut new = Vec::with_capacity($frame.clauses().len());
            for clause in $frame.clauses() {
                if let $clause::Xref(xref) = clause.as_ref() {
                    if let Ident::Prefixed(p) = xref.id() {
                        if p.prefix() == prefix.as_str() {
                            new.push(Line::from($clause::Relationship(
                                Box::new(relid.clone()),
                                Box::new(xref.id().clone().into()),
                            )));
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
        }};
    }

    for entity in entities.iter_mut() {
        match entity {
            Term(x) => process!(x, TermClause),
            Typedef(x) => process!(x, TypedefClause),
            Instance(x) => process!(x, InstanceClause),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::string::ToString;

    use pretty_assertions::assert_eq;
    use textwrap_macros::dedent;

    use super::*;

    #[test]
    fn as_equivalent() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
            treat-xrefs-as-equivalent: TEST

            [Term]
            id: TEST:001
            xref: TEST:002

            [Term]
            id: TEST:002
            "#
        ))
        .unwrap();

        doc.treat_xrefs();

        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-equivalent: TEST

                [Term]
                id: TEST:001
                xref: TEST:002
                equivalent_to: TEST:002

                [Term]
                id: TEST:002
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }

    #[test]
    fn as_is_a() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-is_a: TEST

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                "#
        ))
        .unwrap();
        doc.treat_xrefs();
        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-is_a: TEST

                [Term]
                id: TEST:001
                xref: TEST:002
                is_a: TEST:002

                [Term]
                id: TEST:002
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );

        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-is_a: TEST

                [Typedef]
                id: TEST:001
                xref: TEST:002

                [Typedef]
                id: TEST:002
                "#
        ))
        .unwrap();
        doc.treat_xrefs();
        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-is_a: TEST

                [Typedef]
                id: TEST:001
                xref: TEST:002
                is_a: TEST:002

                [Typedef]
                id: TEST:002
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }

    #[test]
    fn as_has_subclass() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-has-subclass: TEST

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                "#
        ))
        .unwrap();
        doc.treat_xrefs();
        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-has-subclass: TEST

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                is_a: TEST:001
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );

        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-has-subclass: TEST

                [Typedef]
                id: TEST:001
                xref: TEST:002

                [Typedef]
                id: TEST:002
                "#
        ))
        .unwrap();
        doc.treat_xrefs();
        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-has-subclass: TEST

                [Typedef]
                id: TEST:001
                xref: TEST:002

                [Typedef]
                id: TEST:002
                is_a: TEST:001
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }

    #[test]
    fn as_genus_differentia() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-genus-differentia: TEST part_of something

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                "#
        ))
        .unwrap();

        doc.treat_xrefs();

        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-genus-differentia: TEST part_of something

                [Term]
                id: TEST:001
                xref: TEST:002
                intersection_of: TEST:002
                intersection_of: part_of something

                [Term]
                id: TEST:002
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }

    #[test]
    fn as_reverse_genus_differentia() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-reverse-genus-differentia: TEST part_of something

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                "#
        ))
        .unwrap();

        doc.treat_xrefs();

        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-reverse-genus-differentia: TEST part_of something

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                intersection_of: TEST:001
                intersection_of: part_of something
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }

    #[test]
    fn as_relationship() {
        let mut doc = OboDoc::from_str(dedent!(
            r#"
                treat-xrefs-as-relationship: TEST connected_to

                [Term]
                id: TEST:001
                xref: TEST:002

                [Term]
                id: TEST:002
                "#
        ))
        .unwrap();

        doc.treat_xrefs();

        self::assert_eq!(
            dedent!(
                r#"
                treat-xrefs-as-relationship: TEST connected_to

                [Term]
                id: TEST:001
                xref: TEST:002
                relationship: connected_to TEST:002

                [Term]
                id: TEST:002
                "#
            )
            .trim_start_matches('\n'),
            doc.to_string()
        );
    }
}
