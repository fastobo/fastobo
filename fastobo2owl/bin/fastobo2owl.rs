extern crate curie;
extern crate horned_owl;
extern crate fastobo;
extern crate fastobo2owl;

use fastobo::ast::*;
use fastobo2owl::IntoOwl;
use fastobo2owl::constants::uri;

fn main() {

    let mut prefixes = curie::PrefixMapping::default();
    prefixes.add_prefix("xsd", uri::XSD).unwrap();
    prefixes.add_prefix("owl", uri::OWL).unwrap();
    prefixes.add_prefix("obo", uri::OBO).unwrap();
    prefixes.add_prefix("oboInOwl", uri::OBO_IN_OWL).unwrap();
    prefixes.add_prefix("xml", uri::XML).unwrap();
    prefixes.add_prefix("rdf", uri::RDF).unwrap();
    prefixes.add_prefix("dc", uri::DC).unwrap();
    prefixes.add_prefix("rdfs", uri::RDFS).unwrap();

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
