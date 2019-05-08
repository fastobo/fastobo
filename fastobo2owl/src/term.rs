use std::collections::BTreeSet;

use fastobo::ast as obo;
use horned_owl::model as owl;

use crate::constants::datatype::xsd;
use crate::constants::property::dc;
use crate::constants::property::obo_in_owl;
use super::Context;
use super::IntoOwlCtx;
use super::OwlEntity;

impl IntoOwlCtx for obo::TermFrame {
    type Owl = BTreeSet<owl::AnnotatedAxiom>;
    fn into_owl(mut self, ctx: &mut Context) -> Self::Owl {

        // The ID of this frame translated to an IRI.
        let id = obo::Ident::from(self.id().clone().into_inner()).into_owl(ctx);

        // The translated axioms.
        let mut axioms: Self::Owl = BTreeSet::new();

        // Build the annotated class declaration.
        axioms.insert(owl::AnnotatedAxiom {
            annotation: BTreeSet::new(),
            axiom: owl::Axiom::from(owl::DeclareClass(owl::Class(id.clone()))),
        });

        // Add the original OBO ID as an annotation.
        axioms.insert(
            owl::AnnotatedAxiom::from(
                owl::AnnotationAssertion {
                    annotation_subject: id.clone(),
                    annotation: owl::Annotation {
                        annotation_property: ctx.build.annotation_property(
                            obo_in_owl::ID
                        ),
                        annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                            datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                            literal: Some(self.id_mut().as_ref().to_string()),
                            lang: None,
                        })
                    }
                }
            )
        );

        // Convert remaining clauses to axioms.
        for line in self.into_iter() {
            match line.into_inner().into_owl(ctx) {
                OwlEntity::Annotation(annot) => axioms.insert(
                    owl::AnnotatedAxiom::from(owl::AnnotationAssertion::new(id.clone(), annot))
                ),
                OwlEntity::Axiom(axiom) => axioms.insert(owl::AnnotatedAxiom::from(axiom)),
                OwlEntity::None => true,
            };
        }

        // Return the axioms
        axioms
    }
}

impl IntoOwlCtx for obo::TermClause {
    type Owl = OwlEntity;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        match self {

            // IsAnonymous(bool),
            obo::TermClause::Name(name) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("rdfs:label").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                        literal: Some(name.into_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Namespace(ns) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("oboInOwl:hasOBONamespace").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(
                            ctx.prefixes.expand_curie_string("xsd:string").unwrap()
                        )),
                        literal: Some(ns.to_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::AltId(id) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("oboInOwl:hasAlternativeId").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(
                            ctx.prefixes.expand_curie_string("xsd:string").unwrap()
                        )),
                        literal: Some(id.to_string()),
                        lang: None,
                    })
                }
            ),

            // FIXME: add xrefs to translated axiom.
            obo::TermClause::Def(desc, xrefs) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("obo:IAO_0000115").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                        literal: Some(desc.into_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Comment(comment) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("rdfs:comment").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                        literal: Some(comment.into_string()),
                        lang: None,
                    })
                }
            ),
            obo::TermClause::Subset(subset) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("oboInOwl:inSubset").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::IRI(
                        obo::Ident::from(subset).into_owl(ctx),
                    )
                }
            ),

            // FIXME: add xref description to translated axiom.
            obo::TermClause::Xref(xref) => OwlEntity::Axiom(
                owl::Axiom::from(
                    owl::AnnotationAssertion{
                        annotation_subject: ctx.current_frame.clone(),
                        annotation: owl::Annotation {
                            annotation_property: ctx.build.annotation_property(
                                obo_in_owl::HAS_DBXREF
                            ),
                            annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                                datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                                literal: Some(xref.id().to_string()),
                                lang: None,
                            }),
                        }
                    }
                )
            ),

            // Builtin(bool),
            // PropertyValue(PropertyValue),

            obo::TermClause::IsA(supercls) => OwlEntity::Axiom(
                owl::Axiom::from(
                    owl::SubClassOf {
                        super_class: owl::ClassExpression::Class(
                            owl::Class(supercls.into_owl(ctx))
                        ),
                        sub_class: owl::ClassExpression::Class(
                            owl::Class(ctx.current_frame.clone())
                        )
                    }
                )
            ),


            // IntersectionOf(Option<RelationIdent>, ClassIdent),
            // UnionOf(ClassIdent),
            // EquivalentTo(ClassIdent),
            // DisjointFrom(ClassIdent),
            // Relationship(RelationIdent, ClassIdent),
            obo::TermClause::IsObsolete(b) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        ctx.prefixes.expand_curie_string("owl:deprecated").unwrap()
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri(xsd::BOOLEAN)),
                        literal: Some(b.to_string()),
                        lang: None,
                    })
                }
            ),

            // ReplacedBy(ClassIdent),

            obo::TermClause::Consider(id) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        obo_in_owl::CONSIDER
                    ),
                    annotation_value: owl::AnnotationValue::IRI(
                        obo::Ident::from(id).into_owl(ctx),
                    )
                }
            ),

            obo::TermClause::CreatedBy(c) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        dc::CREATOR
                    ),
                    annotation_value: owl::AnnotationValue::Literal(
                        owl::Literal {
                            datatype_iri: Some(ctx.build.iri(xsd::STRING)),
                            literal: Some(c.into_string()),
                            lang: None,
                        }
                    )
                }
            ),

            obo::TermClause::CreationDate(dt) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: ctx.build.annotation_property(
                        dc::DATE
                    ),
                    annotation_value: owl::AnnotationValue::Literal(
                        owl::Literal {
                            datatype_iri: Some(ctx.build.iri(xsd::DATETIME)),
                            lang: None,
                            literal: Some(obo::DateTime::to_xsd_datetime(&dt)),
                        }
                    )
                }
            ),

            _ => OwlEntity::None,
        }


    }
}
