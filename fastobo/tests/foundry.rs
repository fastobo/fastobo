#[macro_use]
extern crate lazy_static;

extern crate fastobo;
extern crate obofoundry;
extern crate reqwest;

use std::io::BufRead;
use std::io::BufReader;

use reqwest::Client;
use reqwest::RedirectPolicy;

lazy_static! {
    /// The HTTP client to download test resources.
    static ref CLIENT: Client = Client::builder()
        .redirect(RedirectPolicy::limited(10))
        .build()
        .unwrap();

    /// The latest OBO Foundry listing.
    static ref FOUNDRY: obofoundry::Foundry = {
        let response = CLIENT.get("http://www.obofoundry.org/registry/ontologies.yml")
            .send()
            .expect("could not download the OBO Foundry listing");
        serde_yaml::from_reader(response)
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
            // request the OBO file
            let res = CLIENT
                .get(url.as_str())
                .send()
                .expect(&format!("could not download {} from {}", &stringify!($ont), url));
            // parse the OBO file if it is a correct OBO file.
            let mut buf = BufReader::new(res);

            let peek = buf.fill_buf().expect("could not read response");
            // println!("{:?}", std::str::from_utf8(peek));

            if peek.starts_with(b"format-version:") {
                match fastobo::ast::OboDoc::from_stream(&mut buf) {
                    Ok(doc) => println!("{}", doc.header()),
                    Err(e) => panic!("{}", e),
                }
            } else {
                let mut lines = String::new();
                for _ in 0..20 {
                    buf.read_line(&mut lines).unwrap();
                }
                panic!("not an OBO file ({})\n: {}", url, lines);
            }
        }
    )
}

foundrytest!(po);
foundrytest!(xao);
foundrytest!(zfa);
foundrytest!(bfo);
foundrytest!(pato);
foundrytest!(fao);
foundrytest!(eco);
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
foundrytest!(cmo);
foundrytest!(hp);
foundrytest!(phipo);
foundrytest!(so);
foundrytest!(mmusdv);
foundrytest!(hsapdv);
foundrytest!(peco);
foundrytest!(apo);
foundrytest!(ehdaa2);
foundrytest!(wbphenotype);
foundrytest!(taxrank);
foundrytest!(plana);
foundrytest!(ddpheno);

// --- Too large to run casually ---------------------------------------------

foundrytest!(#[ignore] mondo);
foundrytest!(#[ignore] ncbitaxon);
foundrytest!(#[ignore] ncit);
foundrytest!(#[ignore] go);
foundrytest!(#[ignore] vto);
foundrytest!(#[ignore] pr);
foundrytest!(#[ignore] tto);



// --- Expected failures -----------------------------------------------------

// Outdated syntax (`exact_synonym`, `xref_analog`)
foundrytest!(#[ignore] trans);
foundrytest!(#[ignore] fix);
// Invalid syntax caused by ChEBI
foundrytest!(#[ignore] fypo);
foundrytest!(#[ignore] sibo);
foundrytest!(#[ignore] fbcv);
// Invalid syntax caused by ENVO
foundrytest!(#[ignore] ecocore);
// Invalid Xref syntax
foundrytest!(#[ignore] chebi);
foundrytest!(#[ignore] uberon);
foundrytest!(#[ignore] xco);
foundrytest!(#[ignore] to);
foundrytest!(#[ignore] pw);
// Invalid syntax (WIP)
foundrytest!(#[ignore] envo);
foundrytest!(#[ignore] mmo);
foundrytest!(#[ignore] mi);
foundrytest!(#[ignore] mco);
// Invalid date
foundrytest!(#[ignore] doid);
// Invalid syntax (reported)
foundrytest!(#[ignore] cl);
foundrytest!(#[ignore] omp);
// Invalid syntax
foundrytest!(#[ignore] gaz);
foundrytest!(#[ignore] hao);
foundrytest!(#[ignore] mp);
foundrytest!(#[ignore] symp);
foundrytest!(#[ignore] zp);
foundrytest!(#[ignore] zeco);
// Unescaped quotes in QuotedString
foundrytest!(#[ignore] rnao);
// Undefined "is_asymmetric"
foundrytest!(#[ignore] ro);
// Download error
foundrytest!(#[ignore] rs);
// OBO Foundry related error
foundrytest!(#[ignore] fbdv);
foundrytest!(#[ignore] xlmod);
// Deprecated and unreachable
foundrytest!(#[ignore] eo);
