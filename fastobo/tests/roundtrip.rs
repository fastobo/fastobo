extern crate pretty_assertions;

extern crate fastobo;

use std::path::PathBuf;
use std::str::FromStr;
use std::fs::read_to_string;

use pretty_assertions::assert_eq;

use fastobo::ast::OboDoc;


macro_rules! roundtriptest {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let path = {
                let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                p.push("tests");
                p.push("data");
                p.push("roundtrip");
                p.push($path);
                p
            };
            println!("{:?}", path);
            let txt = read_to_string(&path)
                .expect("could not read file");
            let doc = fastobo::ast::OboDoc::from_str(&txt)
                .expect("could not parse file");
            assert_eq!(txt, doc.to_string());
        }
    }
}



roundtriptest!(roundtrip01, "01.obo");
