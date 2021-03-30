extern crate pretty_assertions;

extern crate fastobo;

use std::fs::read_to_string;
use std::path::PathBuf;

use pretty_assertions::assert_eq;

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
            let doc = fastobo::from_str(&txt).expect("could not parse file");

            for (i, (l, r)) in doc.to_string().split('\n').zip(txt.split('\n')).enumerate() {
                assert_eq!(l, r, "line {} differs", i);
            }
        }
    };
}

roundtriptest!(msterm);
roundtriptest!(mslite);
roundtriptest!(importlist);
roundtriptest!(rhea);
