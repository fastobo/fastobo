//! A simple `cargo script` to check ISBN identifiers in OBO ontologies.
//!
//! ```cargo
//! [dependencies]
//! fastobo = "0.1.0"
//! isbn = "0.2.0"
//! ```

extern crate fastobo;
extern crate isbn;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ToString;

use fastobo::ast::*;
use fastobo::visit::Visit;
use isbn::Isbn;
use isbn::IsbnError;

#[derive(Default)]
struct IsbnChecker<'a> {
    valid: HashSet<&'a PrefixedIdent>,
    invalid: HashMap<&'a PrefixedIdent, IsbnError>,
}

impl<'a> Visit<'a> for IsbnChecker<'a> {
    fn visit_prefixed_ident(&mut self, id: &'a PrefixedIdent) {
        if id.prefix() == "ISBN" {
            if let Err(e) = Isbn::from_str(id.local()) {
                self.invalid.insert(id, e);
            } else {
                self.valid.insert(id);
            }
        }
    }
}

fn main() {
    for path in std::env::args().skip(1) {
        // Parse the document
        let mut doc = match fastobo::from_file(&path) {
            Ok(doc) => doc,
            Err(e) => panic!("{} could not be parsed:\n{}", path, e),
        };

        // Collect all ISBNs, valid and invalid;
        let mut checker = IsbnChecker::default();
        checker.visit_doc(&mut doc);

        // Report the invalid ISBNs
        println!(
            "Found {} valid ISBN, {} invalid",
            checker.valid.len(),
            checker.invalid.len(),
        );
        for (id, err) in checker.invalid.iter() {
            println!("- {:<16}\t{:?}", id.to_string(), err)
        }
    }
}
