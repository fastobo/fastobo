extern crate lazy_static;
extern crate obofoundry;
extern crate ureq;

extern crate fastobo;

use std::io::BufRead;
use std::io::BufReader;

lazy_static::lazy_static! {
    /// The latest OBO Foundry listing.
    static ref FOUNDRY: obofoundry::Foundry = {
        let response = ureq::get("http://www.obofoundry.org/registry/ontologies.yml")
            .call()
            .unwrap();
        serde_yaml::from_reader(response.into_reader())
            .expect("could not read the OBO Foundry listing")
    };
}

macro_rules! foundrytest {
    ( $(#[$attr:meta])* $ont:ident) => (
        $(#[$attr])*
        #[test]
        fn $ont() {

            let obsolete = FOUNDRY
                .ontologies
                .iter()
                .find(|onto| onto.id == stringify!($ont))
                .expect("could not find ontology")
                .is_obsolete;

            if obsolete {
                panic!("obsolete ontology");
            }

            // get the URL to the OBO product
            let url = &FOUNDRY
                .ontologies
                .iter()
                .find(|onto| onto.id == stringify!($ont))
                .expect("could not find ontology")
                .products
                .iter()
                .find(|prod| prod.id.ends_with(".obo"))
                .expect("could not find obo product")
                .ontology_purl;

            // get the OBO document
            let res = ureq::get(url.as_str()).call().unwrap();

            // parse the OBO file if it is a correct OBO file.
            let mut buf = BufReader::new(res.into_reader());
            let peek = buf.fill_buf().expect("could not read response");

            if peek.starts_with(b"format-version:") {
                for item in fastobo::parser::DefaultParser::from(buf) {
                    if let Err(e) = item {
                        panic!("parsing failed: {}", e);
                    }
                }
            } else {
                panic!("not an OBO file ({})", url);
            }
        }
    )
}

foundrytest!(po);
foundrytest!(xao);
foundrytest!(bfo);
foundrytest!(pato);
foundrytest!(fao);
foundrytest!(ceph);
foundrytest!(wbbt);
foundrytest!(ddanat);
foundrytest!(ms);
foundrytest!(cio);
foundrytest!(zfs);
foundrytest!(emapa);
foundrytest!(wbls);
foundrytest!(olatdv);
foundrytest!(fbbt);
foundrytest!(oba);
foundrytest!(hp);
foundrytest!(mmusdv);
foundrytest!(hsapdv);
foundrytest!(peco);
foundrytest!(apo);
foundrytest!(ehdaa2);
foundrytest!(taxrank);
foundrytest!(ddpheno);
foundrytest!(wbphenotype);
foundrytest!(fbdv);
foundrytest!(omp);
foundrytest!(mco);
foundrytest!(mmo);
foundrytest!(mp);
foundrytest!(poro);
foundrytest!(fbcv);
foundrytest!(zeco);
foundrytest!(ro);
foundrytest!(trans);
foundrytest!(phipo);
foundrytest!(doid);
foundrytest!(xlmod);
foundrytest!(symp);
foundrytest!(exo);
foundrytest!(rs);
foundrytest!(xco);
foundrytest!(zfa);
foundrytest!(pw);
foundrytest!(fypo);
foundrytest!(cmo);

// --- Too large to run casually ---------------------------------------------

// foundrytest!(
//     #[ignore]
//     mondo
// );
// foundrytest!(
//     #[ignore]
//     ncbitaxon
// );
// foundrytest!(
//     #[ignore]
//     ncit
// );
// foundrytest!(
//     #[ignore]
//     go
// );
// foundrytest!(
//     #[ignore]
//     vto
// );
// foundrytest!(
//     #[ignore]
//     pr
// );
// foundrytest!(
//     #[ignore]
//     tto
// );
// foundrytest!(
//     #[ignore]
//     uberon
// );
// foundrytest!(
//     #[ignore]
//     chebi
// );

// --- Expected failures -----------------------------------------------------

// Syntax errors
foundrytest!(
    #[ignore]
    cl
);
foundrytest!(
    #[ignore]
    so
);
foundrytest!(
    #[ignore]
    fix
);
foundrytest!(
    #[ignore]
    eco
);
foundrytest!(
    #[ignore]
    xpo
);
foundrytest!(
    #[ignore]
    mi
);
foundrytest!(
    #[ignore]
    pdumdv
);
foundrytest!(
    #[ignore]
    to
);
foundrytest!(
    #[ignore]
    ecocore
);

// Invalid syntax caused by ChEBI
foundrytest!(
    #[ignore]
    sibo
);

// Invalid syntax (WIP)
foundrytest!(
    #[ignore]
    envo
);
// Invalid syntax
foundrytest!(
    #[ignore]
    gaz
);
foundrytest!(
    #[ignore]
    hao
);

foundrytest!(
    #[ignore]
    zp
);

foundrytest!(
    #[ignore]
    plana
);
foundrytest!(
    #[ignore]
    planp
);

// Unescaped quotes in QuotedString
foundrytest!(
    #[ignore]
    rnao
);
// Deprecated and unreachable
foundrytest!(
    #[ignore]
    eo
);
