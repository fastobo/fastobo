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
                // _ => unimplemented!(),
                _ => (),
            };
        }

        // Return the produced OWL ontology
        ont
    }
}


impl IntoOwl for obo::OboDoc {
    type Owl = owl::Ontology;
    fn into_owl(self) -> Self::Owl {

        // Create prefix mapping with default prefixes
        let mut prefixes = curie::PrefixMapping::default();
        prefixes.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#").unwrap();
        prefixes.add_prefix("owl", "http://www.w3.org/2002/07/owl#").unwrap();
        prefixes.add_prefix("obo", "http://purl.obolibrary.org/obo/").unwrap();
        prefixes.add_prefix("oboInOwl", "http://www.geneontology.org/formats/oboInOwl#").unwrap();
        prefixes.add_prefix("xml", "http://www.w3.org/XML/1998/namespace").unwrap();
        prefixes.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
        prefixes.add_prefix("dc", "http://purl.org/dc/elements/1.1/").unwrap();
        prefixes.add_prefix("rdfs", "http://www.w3.org/2000/01/rdf-schema#").unwrap();

        // Add the prefixes from the OBO header
        for clause in self.header() {
            if let obo::HeaderClause::Idspace(prefix, url, _) = clause {
                prefixes.add_prefix(prefix.as_str(), url.as_str()).unwrap();
            }
        }

        // Create context
        let build: horned_owl::model::Build = Default::default();
        let ontology_iri = obo::Url::parse("http://purl.obolibrary.org/obo/something.obo").unwrap();
        let current_frame = build.iri(ontology_iri.clone().into_string());
        let idspaces = Default::default();
        let mut ctx = Context {
            build,
            prefixes,
            idspaces,
            ontology_iri,
            current_frame,
        };

        <Self as IntoOwlCtx>::into_owl(self, &mut ctx)
    }
}
