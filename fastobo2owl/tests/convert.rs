extern crate fastobo;
extern crate fastobo2owl;
extern crate lazy_static;
extern crate pretty_assertions;

use std::path::PathBuf;
use std::str::FromStr;
use std::fs::read_to_string;

use pretty_assertions::assert_eq;

use fastobo::ast::OboDoc;
use fastobo2owl::IntoOwl;



lazy_static::lazy_static! {
    /// The latest OBO Foundry listing.
    static ref PREFIXES: curie::PrefixMapping = {

        let mut prefixes = curie::PrefixMapping::default();


        prefixes

    };
}



macro_rules! converttest {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {

            let dir = {
                let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                p.push("tests");
                p.push("data");
                p.push("to_owl");
                p
            };

            let input_path = dir.join(format!("{}.input.obo", $path));
            let output_path = dir.join(format!("{}.output.owl", $path));

            // Parse the OBO doc and convert it to OWL.
            let obo_doc = OboDoc::from_file(&input_path)
                .expect("could not parse input file");
            let actual = obo_doc.into_owl();

            // Read the expected OWL
            let (expected, prefixes) = horned_owl::io::reader::read(
                &mut std::io::BufReader::new(
                    std::fs::File::open(&output_path)
                        .expect("could not open output file")
                )
            ).expect("could not parse output file");

            // Test equality
            assert_eq!(actual, expected);
        }
    }
}



converttest!(convert_01, "01");
