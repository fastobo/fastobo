#[macro_use]
extern crate failure;

extern crate curie;
extern crate either;
extern crate fastobo;
extern crate horned_owl;

use std::collections::BTreeSet;
use std::collections::HashMap;

use either::Either;
use fastobo::ast as obo;
use fastobo::share::Redeem;
use horned_owl::model as owl;

#[derive(Debug, Eq, Fail, PartialEq)]
pub enum IntoOwlError {
    #[fail(display = "missing ontology IRI")]
    MissingOntologyIri,
}

/// A context to pass as argument to `into_owl`.
#[derive(Default)]
pub struct Context {
    build: owl::Build,
    idspaces: HashMap<obo::IdentPrefix, obo::Url>,
    ontology_iri: Option<obo::Url>,
    current_frame: Option<owl::IRI>,
}

impl Context {
    fn in_frame(&mut self, frame_id: owl::IRI) -> &mut Self {
        self.current_frame = Some(frame_id);
        self
    }
}

pub trait IntoOwl {
    type Owl;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError>;
}

impl IntoOwl for obo::OboDoc {
    type Owl = owl::Ontology;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {

        let mut ont = owl::Ontology::new();
        for clause in self.header.into_iter() {
            ont.insert(clause.into_owl(ctx)?);
        }

        for entity in self.entities.into_iter() {
            match entity {
                obo::EntityFrame::Term(frame) => {
                    for axiom in frame.into_owl(ctx)? {
                        ont.insert(axiom);
                    }
                }
                _ => unimplemented!(),
            };
        }

        Ok(ont)
    }
}

impl IntoOwl for obo::HeaderClause {
    type Owl = owl::Axiom;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        let annotation = owl::OntologyAnnotation(match self {
            obo::HeaderClause::FormatVersion(v) => owl::Annotation {
                annotation_property: owl::AnnotationProperty(
                    ctx.build.iri("oboInOwl:hasOBOFormatVersion")
                ),
                annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                    datatype_iri: Some(ctx.build.iri("xsd:string")),
                    literal: Some(v.as_str().to_string()),
                    lang: None,
                }),
            },
            obo::HeaderClause::Remark(v) => owl::Annotation {
                annotation_property: owl::AnnotationProperty(
                    ctx.build.iri("rdfs:comment")
                ),
                annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                    datatype_iri: Some(ctx.build.iri("xsd:string")),
                    literal: Some(v.as_str().to_string()),
                    lang: None,
                })
            },
            _ => unimplemented!(),
        });
        Ok(owl::Axiom::from(annotation))
    }
}

impl IntoOwl for obo::TermFrame {
    type Owl = BTreeSet<owl::AnnotatedAxiom>;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {

        // The abbreviated ID, as declared in the original OBO document.
        let obo_id = self.id().as_ref().to_string();

        // The expanded IRI for the ID.
        let iri_id = self.id().as_ref().clone().into_owl(ctx)?;
        let ctx = ctx.in_frame(iri_id.0.clone());

        // Create the frame axiom.
        let mut decl = owl::AnnotatedAxiom {
            axiom: owl::Axiom::from(owl::DeclareClass(iri_id)),
            annotation: BTreeSet::default(),
        };

        // Add the original ID as an `oboInOwl:id` annotation.
        decl.annotation.insert(
            owl::Annotation {
                annotation_property: owl::AnnotationProperty(
                    ctx.build.iri("oboInOwl:id")
                ),
                annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                    datatype_iri: Some(ctx.build.iri("xsd:string")),
                    literal: Some(obo_id),
                    lang: None
                }),
            }
        );

        // Convert the term clauses to annotations
        let mut axioms = BTreeSet::new();
        for clause in self.into_iter() {
            // FIXME: handle qualifiers as well.
            match clause.into_inner().into_owl(ctx)? {
                Either::Left(annot) => decl.annotation.insert(annot),
                Either::Right(axiom) => axioms.insert(axiom.into()),
            };
        }

        axioms.insert(decl.into());
        Ok(axioms)
    }
}

impl IntoOwl for obo::TermClause {
    type Owl = Either<owl::Annotation, owl::Axiom>;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        match self {
            obo::TermClause::Name(name) => Ok(Either::Left(owl::Annotation {
                annotation_property: owl::AnnotationProperty(
                    ctx.build.iri("rdfs:label")
                ),
                annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                    datatype_iri: Some(ctx.build.iri("xsd:string")),
                    literal: Some(name.as_str().to_string()),
                    lang: None
                }),
            })),

            obo::TermClause::IsA(supercls) => Ok(Either::Right(From::from(
                owl::SubClassOf::new(
                    owl::ClassExpression::Class(supercls.into_owl(ctx)?),
                    owl::ClassExpression::Class(owl::Class(
                        ctx.current_frame.as_ref().unwrap().clone()
                    )),
                )
            ))),

            _ => unimplemented!(),
        }
    }
}

impl IntoOwl for obo::QualifierList {
    type Owl = BTreeSet<owl::Annotation>;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        let mut set = BTreeSet::new();
        for qualifier in self.into_iter() {
            if let Some(q) = qualifier.into_owl(ctx)? {
                set.insert(q);
            }
        }
        Ok(set)
    }
}

impl IntoOwl for obo::Qualifier {
    type Owl = Option<owl::Annotation>;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        // The qualifiers that are not translated as-is (see section 5.7
        // of the OBO Flat File Format 1.4 Syntax and Semantics).
        let IGNORED = [
            obo::UnprefixedIdent::new("cardinality").into(),
            obo::UnprefixedIdent::new("maxCardinality").into(),
            obo::UnprefixedIdent::new("minCardinality").into(),
            obo::UnprefixedIdent::new("gci_relation").into(),
            obo::UnprefixedIdent::new("gci_relation").into(),
        ];
        if IGNORED.contains(&self.key) {
            Ok(None)
        } else {
            Ok(Some(owl::Annotation{
                annotation_property: owl::AnnotationProperty(
                    ctx.build.iri(self.key.to_string()) // FIXME ?
                ),
                annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                    datatype_iri: None,
                    literal: Some(self.value.as_str().to_string()),
                    lang: None,
                })
            }))
        }
    }
}

impl IntoOwl for obo::ClassIdent {
    type Owl = owl::Class;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        obo::Ident::from(self).into_owl(ctx).map(owl::Class)
    }
}

impl IntoOwl for obo::Ident {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        match self {
            obo::Ident::Url(url) => url.into_owl(ctx),
            obo::Ident::Unprefixed(id) => id.into_owl(ctx),
            obo::Ident::Prefixed(id) => id.into_owl(ctx),
        }
    }
}

impl IntoOwl for obo::UnprefixedIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        if let Some(iri) = &ctx.ontology_iri {
            Ok(ctx.build.iri(format!("{}#{}", iri, self.as_str())))
        } else {
            Err(IntoOwlError::MissingOntologyIri)
        }
    }
}

impl IntoOwl for obo::Url {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        Ok(ctx.build.iri(self.to_string()))
    }
}

impl IntoOwl for obo::PrefixedIdent {
    type Owl = owl::IRI;
    fn into_owl(self, ctx: &mut Context) -> Result<Self::Owl, IntoOwlError> {
        let iri = match ctx.idspaces.get(&self.prefix) {
            Some(url) => format!("{}{}", url, self.local.as_str()),
            None => format!(
                "http://purl.obolibrary.org/obo/{}_{}",
                self.prefix.as_str(),
                self.local.as_str()
            ),
        };
        Ok(ctx.build.iri(iri))
    }
}

// #[cfg(test)]
// mod tests {
//
//     use std::str::FromStr;
//     use super::*;
//
//     #[test]
//     fn test() {
//
//         let doc = obo::OboDoc::from_str(
//             "format-version: 1.2\n[Term]\nid: MS:1000031\nname: instrument\n[Term]\nid: MS:1000032\nis_a: MS:1000031\n"
//         ).unwrap();
//
//         let mut ctx = Context::default();
//         let ont = doc.into_owl(&mut ctx).unwrap();
//
//         let mut map = curie::PrefixMapping::default();
//         map.add_prefix("obo", "http://purl.obolibrary.org/obo/");
//         map.add_prefix("owl", "http://www.w3.org/2002/07/owl#");
//         map.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
//         map.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#");
//
//         horned_owl::io::writer::write(&mut std::io::stdout(), &ont, Some(&map));
//         // panic!()
//     }
// }

// impl IntoOwl for obo::Url {
//     type Owl = owl::IRI;
//     fn into_owl(self) -> Self::Owl {
//         owl::Build::new().iri(self.into_string())
//     }
// }
//
// impl IntoOwl for obo::Import {
//     type Owl = owl::Import;
//     fn into_owl(self) -> Self::Owl {
//         use self::obo::Import::*;
//         owl::Import(
//             match self {
//                 Url(url) => url.into_owl(),
//                 Abbreviated(abbr) => owl::Build::new().iri(
//                     format!("http://purl.obolibrary.org/obo/{}.owl", abbr)
//                 )
//             }
//         )
//     }
// }
