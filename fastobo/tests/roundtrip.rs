extern crate pretty_assertions;

extern crate fastobo;

use std::path::PathBuf;
use std::str::FromStr;
use std::fs::read_to_string;

use pretty_assertions::assert_eq;

use fastobo::ast::OboDoc;


macro_rules! roundtriptest {
    ($name:ident) => {
        #[test]
        fn $name() {
            let dir = {
                let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                p.push("tests");
                p.push("data");
                p.push("roundtrip");
                p
            };

            let input = dir.join(format!("{}.obo", stringify!($name)));
            let txt = read_to_string(&input).expect("could not read file");
            let doc = OboDoc::from_str(&txt).expect("could not parse file");
            assert_eq!(txt, doc.to_string());
        }
    }
}



roundtriptest!(msterm);
