macro_rules! test_roundtrip_header {
    ($name:ident, $path:tt) => {
        #[test]
        fn $name() {
            use std::str::FromStr;
            use std::string::ToString;
            use $crate::ontology::obo::ast::HeaderFrame;

            let data = include_str!($path);
            let i = data.find("\n\n").unwrap();
            let header = &data[..i + 1];

            println!("{}", header);

            let tree = HeaderFrame::from_str(header).expect("parser failed");
            let out = tree.to_string();

            assert_eq!(out, header)
        }
    };
}

pub mod header {
    test_roundtrip_header!(psi_ms, "../data/psi-ms.obo");
    test_roundtrip_header!(ro, "../data/ro.obo");
    test_roundtrip_header!(pato, "../data/pato.obo");
}
