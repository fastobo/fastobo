use std::collections::HashMap;

use fastobo::ast as obo;
use fastobo::ext::OboSemantics;
use horned_owl::model as owl;

use crate::constants::uri;
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

        // Convert each entity to a set of OWL axioms that are then added to the ontology.
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

        // Return the produced OWL ontology.
        ont
    }
}

impl IntoOwl for obo::OboDoc {
    type Owl = owl::Ontology;
    fn into_owl(mut self) -> Self::Owl {

        // Process the xref header macros.
        // Assigning the default namespace is not needed since we are only
        // processing the current document, so there should be no namespace
        // collision.
        self.treat_xrefs();

        // Create idspace and prefix mapping with default prefixes.
        let mut prefixes = curie::PrefixMapping::default();
        prefixes.add_prefix("xsd", uri::XSD).unwrap();
        prefixes.add_prefix("owl", uri::OWL).unwrap();
        prefixes.add_prefix("obo", uri::OBO).unwrap();
        prefixes.add_prefix("oboInOwl", uri::OBO_IN_OWL).unwrap();
        prefixes.add_prefix("xml", uri::XML).unwrap();
        prefixes.add_prefix("rdf", uri::RDF).unwrap();
        prefixes.add_prefix("dc", uri::DC).unwrap();
        prefixes.add_prefix("rdfs", uri::RDFS).unwrap();

        // Create idspace mapping with implicit IDspaces.
        let mut idspaces = HashMap::new();
        idspaces.insert(
            obo::IdentPrefix::new("BFO"),
            obo::Url::parse(&format!("{}BFO_", uri::OBO, )).unwrap()
        );
        idspaces.insert(
            obo::IdentPrefix::new("RO"),
            obo::Url::parse(&format!("{}RO", uri::OBO, )).unwrap()
        );

        // Add the prefixes and IDspaces from the OBO header.
        for clause in self.header() {
            if let obo::HeaderClause::Idspace(prefix, url, _) = clause {
                prefixes.add_prefix(prefix.as_str(), url.as_str()).unwrap();
                idspaces.insert(prefix.clone(), url.clone());
            }
        }

        // Create the conversion context.
        let build: horned_owl::model::Build = Default::default();
        let ontology_iri = obo::Url::parse(uri::OBO).unwrap(); // FIXME
        let current_frame = build.iri(ontology_iri.clone().into_string());
        let mut ctx = Context {
            build,
            prefixes,
            idspaces,
            ontology_iri,
            current_frame,
        };

        // Return the converted document.
        <Self as IntoOwlCtx>::into_owl(self, &mut ctx)
    }
}
