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
            .call();
        serde_yaml::from_reader(response.into_reader())
            .expect("could not read the OBO Foundry listing")
    };
}

macro_rules! foundrytest {
    ( $(#[$attr:meta])* $ont:ident) => (
        $(#[$attr])*
        #[test]
        fn $ont() {
            // get the URL to the OBO product
            let ref url = FOUNDRY
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
            let res = ureq::get(url.as_str()).call();

            // parse the OBO file if it is a correct OBO file.
            let mut buf = BufReader::new(res.into_reader());
            let peek = buf.fill_buf().expect("could not read response");

            if peek.starts_with(b"format-version:") {
                match fastobo::from_reader(&mut buf) {
                    Ok(doc) => println!("{}", doc.header()),
                    Err(e) => panic!("{}", e),
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
foundrytest!(xpo);
foundrytest!(exo);
foundrytest!(wbls);
foundrytest!(olatdv);
foundrytest!(planp);
foundrytest!(fbbt);
foundrytest!(pdumdv);
foundrytest!(oba);
foundrytest!(hp);
foundrytest!(phipo);
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
foundrytest!(mp);
foundrytest!(to);
foundrytest!(poro);
foundrytest!(fbcv);
foundrytest!(zeco);
foundrytest!(ro);
foundrytest!(cl);
foundrytest!(mi);
foundrytest!(eco);
foundrytest!(trans);

// --- Too large to run casually ---------------------------------------------

foundrytest!(
    #[ignore]
    mondo
);
foundrytest!(
    #[ignore]
    ncbitaxon
);
foundrytest!(
    #[ignore]
    ncit
);
foundrytest!(
    #[ignore]
    go
);
foundrytest!(
    #[ignore]
    vto
);
foundrytest!(
    #[ignore]
    pr
);
foundrytest!(
    #[ignore]
    tto
);

// --- Expected failures -----------------------------------------------------

foundrytest!(
    #[ignore]
    so
);

foundrytest!(
    #[ignore]
    fix
);
// fails in Traivs-CI
foundrytest!(
    #[ignore]
    doid
);
// Invalid syntax caused by ChEBI
foundrytest!(
    #[ignore]
    fypo
);
foundrytest!(
    #[ignore]
    sibo
);
// Invalid syntax caused by ENVO
foundrytest!(
    #[ignore]
    ecocore
);
// Invalid Xref syntax
foundrytest!(
    #[ignore]
    chebi
);
foundrytest!(
    #[ignore]
    uberon
);
foundrytest!(
    #[ignore]
    xco
);
foundrytest!(
    #[ignore]
    pw
);
// Invalid syntax (WIP)
foundrytest!(
    #[ignore]
    envo
);
foundrytest!(
    #[ignore]
    mmo
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
    symp
);
foundrytest!(
    #[ignore]
    zp
);

foundrytest!(
    #[ignore]
    xlmod
);
foundrytest!(
    #[ignore]
    plana
);
foundrytest!(
    #[ignore]
    zfa
);
// Unescaped quotes in QuotedString
foundrytest!(
    #[ignore]
    rnao
);
// Download error
foundrytest!(
    #[ignore]
    rs
);
// Deprecated and unreachable
foundrytest!(
    #[ignore]
    eo
);
// Invalid Qualifier List
foundrytest!(
    #[ignore]
    cmo
);
