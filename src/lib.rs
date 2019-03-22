// #![feature(macro_literal_matcher)]
// #![feature(specialization)]
// #![feature(try_from)]
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate pest_derive;

extern crate chrono;
extern crate iri_string;
extern crate pest;

#[cfg(test)]
extern crate textwrap;

pub mod error;
pub mod obo14;
