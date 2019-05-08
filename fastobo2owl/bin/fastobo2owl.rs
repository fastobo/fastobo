extern crate curie;
extern crate horned_owl;
extern crate fastobo;
extern crate fastobo2owl;

use fastobo::ast::*;
use fastobo2owl::IntoOwl;

fn main() {



    let mut prefixes = curie::PrefixMapping::default();
    prefixes.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#").unwrap();
    prefixes.add_prefix("owl", "http://www.w3.org/2002/07/owl#").unwrap();
    prefixes.add_prefix("obo", "http://purl.obolibrary.org/obo/").unwrap();
    prefixes.add_prefix("oboInOwl", "http://www.geneontology.org/formats/oboInOwl#").unwrap();
    prefixes.add_prefix("xml", "http://www.w3.org/XML/1998/namespace").unwrap();
    prefixes.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    prefixes.add_prefix("dc", "http://purl.org/dc/elements/1.1/").unwrap();
    prefixes.add_prefix("rdfs", "http://www.w3.org/2000/01/rdf-schema#").unwrap();

    for arg in std::env::args().skip(1) {

        let path = std::path::PathBuf::from(arg);

        // Parse the document
        let mut obodoc = match OboDoc::from_file(&path) {
            Ok(doc) => doc,
            Err(e) => panic!("{:?} could not be parsed:\n{}", path, e),
        };

        // Convert to OWL
        let owldoc = obodoc.into_owl();

        // Write it back
        let file = std::fs::File::create(path.with_extension("owl")).unwrap();
        let mut w = std::io::BufWriter::new(file);
        horned_owl::io::writer::write(&mut w, &owldoc, Some(&prefixes)).unwrap();
    }
}
