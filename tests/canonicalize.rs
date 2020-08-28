extern crate fastobo;
extern crate pretty_assertions;

use std::path::PathBuf;

use pretty_assertions::assert_eq;

macro_rules! canonicalizetest {
    ($name:ident) => {
        #[test]
        fn $name() {
            let dir = {
                let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                p.push("tests");
                p.push("data");
                p.push("canonicalize");
                p
            };

            let input_path = dir.join(format!("{}.input.obo", stringify!($name)));
            let output_path = dir.join(format!("{}.output.obo", stringify!($name)));

            let mut doc = fastobo::from_file(&input_path).unwrap();
            doc.header_mut().sort_unstable();

            println!("{}", doc.header());

            let output = std::fs::read_to_string(&output_path).unwrap();
            assert_eq!(doc.to_string(), output);
        }
    };
}

canonicalizetest!(header);
