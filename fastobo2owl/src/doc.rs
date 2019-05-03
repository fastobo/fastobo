use fastobo::ast as obo;
use horned_owl::model as owl;

use super::Context;
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

        let header = std::mem::replace(self.header_mut(), Default::default());
        for clause in header.into_iter() {
            match clause.into_owl(ctx) {
                OwlEntity::Axiom(a) => ont.insert(a),
                OwlEntity::Annotation(a) => ont.insert(owl::OntologyAnnotation(a)),
                OwlEntity::None => true,
            };
        }

        // let entities = std::mem::replace(self.entities_mut(), Default::default());
        // for entity in entities.into_iter() {
        //     match entity {
        //         obo::EntityFrame::Term(frame) => {
        //             for axiom in frame.into_owl(ctx)? {
        //                 ont.insert(axiom);
        //             }
        //         }
        //         _ => unimplemented!(),
        //     };
        // }

        ont
    }
}
