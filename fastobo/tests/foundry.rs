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
    ($ont:ident) => {

        #[test]
        #[ignore]
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
                    buf.read_line(&mut lines);
                }
                panic!("not an OBO file ({})\n: {}", url, lines);
            }
        }
    }
}

foundrytest!(po);
foundrytest!(xao);
foundrytest!(zfa);
foundrytest!(go);
foundrytest!(bfo);
foundrytest!(pato);
foundrytest!(doid);
foundrytest!(chebi);
foundrytest!(pr);
foundrytest!(fao);
foundrytest!(gaz);
foundrytest!(hao);
foundrytest!(rs);
foundrytest!(rnao);
foundrytest!(eco);
foundrytest!(sibo);
foundrytest!(ceph);
foundrytest!(wbbt);
foundrytest!(ro);
foundrytest!(ddanat);
foundrytest!(ms);
foundrytest!(cio);
foundrytest!(fix);
foundrytest!(mi);
foundrytest!(zfs);
foundrytest!(mco);
foundrytest!(emapa);
foundrytest!(trans);
foundrytest!(cl);
foundrytest!(xpo);
foundrytest!(mp);
foundrytest!(xco);
foundrytest!(exo);
foundrytest!(wbls);
foundrytest!(pw);
foundrytest!(olatdv);
foundrytest!(fbcv);
foundrytest!(planp);
foundrytest!(fbbt);
foundrytest!(pdumdv);
foundrytest!(ecocore);
foundrytest!(oba);
foundrytest!(cmo);
foundrytest!(hp);
foundrytest!(phipo);
foundrytest!(so);
foundrytest!(omp);
foundrytest!(ncbitaxon);
foundrytest!(vto);
foundrytest!(mmusdv);
foundrytest!(fypo);
foundrytest!(zp);
foundrytest!(hsapdv);
foundrytest!(peco);
foundrytest!(mondo);
foundrytest!(apo);
foundrytest!(ehdaa2);
foundrytest!(fbdv);
foundrytest!(symp);
foundrytest!(xlmod);
foundrytest!(wbphenotype);
foundrytest!(uberon);
foundrytest!(mmo);
foundrytest!(taxrank);
foundrytest!(tto);
foundrytest!(envo);
foundrytest!(plana);
foundrytest!(to);
foundrytest!(ddpheno);
foundrytest!(ncit);
foundrytest!(zeco);
// foundrytest!(eo); DEPRECATED and UNREACHABLE
