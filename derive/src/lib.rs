#[macro_use]
extern crate darling;
extern crate heck;
extern crate proc_macro;
extern crate syn;

mod obo_clause;
mod utils;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;

use self::obo_clause::OboClauseDerive;

#[proc_macro_derive(OboClause, attributes(clause))]
pub fn oboclause_derive(input: TokenStream) -> TokenStream {
    let parsed = syn::parse(input).unwrap();
    let receiver = OboClauseDerive::from_derive_input(&parsed).unwrap();

    let oboclause_impl = receiver.oboclause_impl();
    let display_impl = receiver.display_impl();

    TokenStream::from(quote!(
        #oboclause_impl
        #display_impl
    ))
}
