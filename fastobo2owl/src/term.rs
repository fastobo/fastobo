use std::collections::BTreeSet;

use fastobo::ast as obo;
use horned_owl::model as owl;

use super::Context;
use super::IntoOwlCtx;
use super::OwlEntity;

impl IntoOwlCtx for obo::TermFrame {
    type Owl = BTreeSet<owl::AnnotatedAxiom>;
    fn into_owl(mut self, ctx: &mut Context) -> Self::Owl {

        // The produced axiom.
        let mut frame: Self::Owl = BTreeSet::new();

        // Build the annotated class declaration.
        let mut cls = owl::AnnotatedAxiom {
            annotation: BTreeSet::new(),
            axiom: owl::Axiom::from(owl::DeclareClass(
                owl::Class(obo::Ident::from(self.id().clone().into_inner()).into_owl(ctx))
            )),
        };

        // Add the original OBO ID as an annotation.
        cls.annotation.insert(owl::Annotation {
            annotation_property: owl::AnnotationProperty(
                ctx.build.iri("oboInOwl:id")
            ),
            annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                datatype_iri: Some(ctx.build.iri("xsd:string")),
                literal: Some(self.id().as_ref().to_string()),
                lang: None,
            })
        });

        // Convert remaining clauses to annotations or axioms.
        for line in self.into_iter() {
            match line.into_inner().into_owl(ctx) {
                OwlEntity::Annotation(annot) => cls.annotation.insert(annot),
                OwlEntity::Axiom(axiom) => frame.insert(owl::AnnotatedAxiom::from(axiom)),
                OwlEntity::None => true,
            };
        }

        // Add the main clause declaration to the produced axioms and return the frame.
        frame.insert(cls.into());
        frame
    }
}

impl IntoOwlCtx for obo::TermClause {
    type Owl = OwlEntity;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        match self {

            // IsAnonymous(bool),
            obo::TermClause::Name(name) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property("rdfs:label"),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(name.into_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Namespace(ns) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        "oboInOwl:hasOBONamespace"
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(ns.to_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::AltId(id) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        "oboInOwl:hasAlternativeId"
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(id.to_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Def(desc, xrefs) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property("obo:IAO_0000115"),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(desc.into_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Comment(comment) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property("rdfs:comment"),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(comment.into_string()),
                        lang: None,
                    })
                }
            ),


            obo::TermClause::Subset(subset) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property("oboInOwl:inSubset"),
                    annotation_value: owl::AnnotationValue::IRI(
                        obo::Ident::from(subset).into_owl(ctx),
                    )
                }
            ),
            // Xref(Xref),
            // Builtin(bool),
            // PropertyValue(PropertyValue),
            // IsA(ClassIdent),
            // IntersectionOf(Option<RelationIdent>, ClassIdent),
            // UnionOf(ClassIdent),
            // EquivalentTo(ClassIdent),
            // DisjointFrom(ClassIdent),
            // Relationship(RelationIdent, ClassIdent),
            // IsObsolete(bool),
            // ReplacedBy(ClassIdent),
            // Consider(ClassIdent),
            // CreatedBy(UnquotedString),
            // CreationDate(IsoDateTime),

            _ => OwlEntity::None,
        }


    }
}
