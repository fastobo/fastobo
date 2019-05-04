use fastobo::ast as obo;
use horned_owl::model as owl;

use super::Context;
use super::IntoOwl;
use super::IntoOwlCtx;
use super::OwlEntity;

impl IntoOwlCtx for obo::OboDoc {
    type Owl = owl::Ontology;
    fn into_owl(mut self, ctx: &mut Context) -> Self::Owl {

        let mut ont = owl::Ontology::new();

        // TODO: declare the IRI and Version IRI for the ontology.
        // ont.id = owl::OntologyID {
        //     iri: Some(), // http://purl.obolibrary.org/obo/{ontology}.owl
        //     viri: Some(), // http://purl.obolibrary.org/obo/{ontology}/{data-version}/{ontology}.owl
        // }:

        // Convert the header frame: most frames end up as Ontology annotations, but some of
        // them require extra axioms.
        let header = std::mem::replace(self.header_mut(), Default::default());
        for clause in header.into_iter() {
            match clause.into_owl(ctx) {
                OwlEntity::Axiom(a) => ont.insert(a),
                OwlEntity::Annotation(a) => ont.insert(owl::OntologyAnnotation(a)),
                OwlEntity::None => true,
            };
        }

        // Convert each entity to a set of OWL axioms that are then added to the ontologys
        let entities = std::mem::replace(self.entities_mut(), Default::default());
        for entity in entities.into_iter() {
            ctx.current_frame = entity.id().clone().into_owl(ctx);
            match entity {
                obo::EntityFrame::Term(frame) => {
                    for axiom in frame.into_owl(ctx) {
                        ont.insert(axiom);
                    }
                }
                _ => unimplemented!(),
            };
        }

        // Return the produced OWL ontology
        ont
    }
}


impl IntoOwl for obo::OboDoc {
    type Owl = owl::Ontology;
    fn into_owl(self) -> Self::Owl {

        let build: horned_owl::model::Build = Default::default();
        let ontology_iri = obo::Url::parse("http://purl.obolibrary.org/obo/something.obo").unwrap();
        let current_frame = build.iri(ontology_iri.clone().into_string());
        let idspaces = Default::default();

        let mut ctx = Context {
            build,
            idspaces,
            ontology_iri,
            current_frame,
        };

        <Self as IntoOwlCtx>::into_owl(self, &mut ctx)
    }
}
