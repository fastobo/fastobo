#[macro_use]
extern crate darling;
extern crate heck;
extern crate proc_macro;
extern crate syn;

use std::borrow::Cow;

use heck::SnakeCase;

use darling::{ast, util, FromDeriveInput};
use proc_macro::TokenStream;
use quote::quote;
use syn::export::Span;
use syn::parse_quote;
use syn::{Ident, Path, Type};

/// Return `true` if a type is an `Option` type.
fn is_option(ty: &Type) -> bool {
    if let Type::Path(syn::TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.iter().next() {
            segment.ident == "Option"
        } else {
            false
        }
    } else {
        false
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(clause), supports(newtype, tuple))]
struct OboClauseVariant {
    ident: Ident,
    fields: ast::Fields<Type>,
    attrs: Vec<syn::Attribute>,
    #[darling(default)]
    tag: Option<syn::LitStr>,
    #[darling(default)]
    cardinality: Option<syn::Path>,
}

impl OboClauseVariant {
    fn tag(&self) -> Cow<syn::LitStr> {
        // match
        if let Some(tag) = &self.tag {
            Cow::Borrowed(tag)
        } else {
            let tag_str = self.ident.to_string().to_snake_case();
            let tag = syn::LitStr::new(&tag_str, self.ident.span());
            Cow::Owned(tag)
        }
    }

    fn cardinality(&self) -> Cow<syn::Path> {
        match &self.cardinality {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(syn::parse_quote! { Any }),
        }
    }

    /// Make the arms for a particular variant to be used in `Display` impl.
    fn fmt_arms(&self) -> Vec<syn::Arm> {
        // Extract ident and tag to use in `quote!` calls.
        let id = &self.ident;
        let tag = self.tag();
        let mk_ident = |n| syn::Ident::new(&format!("__{}_{}", id, n), Span::call_site());

        // If the variant contain an option, we need two arms: one where the
        // value is `Some` and the other when the value is `None`.
        // NB: limited to a single `Option` per variant.
        if let Some(idx) = self.fields.iter().position(|ty| is_option(ty)) {
            // The arm pattern and expression when the field is `None`
            let c1_none: Vec<syn::Expr> = {
                self.fields
                    .iter()
                    .enumerate()
                    .map(|(n, _)| {
                        if n == idx {
                            parse_quote!(None)
                        } else {
                            let ident =
                                syn::Ident::new(&format!("__{}_{}", id, n), Span::call_site());
                            parse_quote!(#ident)
                        }
                    })
                    .collect()
            };
            let c2_none: Vec<syn::Ident> = {
                self.fields
                    .iter()
                    .enumerate()
                    .flat_map(|(n, _)| {
                        if n == idx {
                            None
                        } else {
                            let ident =
                                syn::Ident::new(&format!("__{}_{}", id, n), Span::call_site());
                            Some(parse_quote!(#ident))
                        }
                    })
                    .collect()
            };
            // The arm pattern and expression when the field is `Some`
            let c1_some: Vec<syn::Expr> = {
                self.fields
                    .iter()
                    .enumerate()
                    .map(|(n, _)| {
                        let ident = mk_ident(n);
                        if n == idx {
                            parse_quote!( Some(#ident) )
                        } else {
                            parse_quote!(#ident)
                        }
                    })
                    .collect()
            };
            let c2_some: Vec<syn::Ident> = {
                self.fields
                    .iter()
                    .enumerate()
                    .map(|(n, _)| mk_ident(n))
                    .collect()
            };
            //
            vec![
                parse_quote! {
                    #id( #(#c1_none,)* ) => f.write_str(#tag).and(f.write_char(':'))
                        #(.and(f.write_char(' ')).and(#c2_none.fmt(f)))*,
                },
                parse_quote! {
                    #id( #(#c1_some,)* ) => f.write_str(#tag).and(f.write_char(':'))
                        #(.and(f.write_char(' ')).and(#c2_some.fmt(f)))*,
                },
            ]
        } else {
            let catches: Vec<syn::Ident> = {
                self.fields
                    .iter()
                    .enumerate()
                    .map(|(n, _)| mk_ident(n))
                    .collect()
            };
            let c1 = &catches;
            let c2 = &catches;
            vec![parse_quote! {
                #id( #(ref #c1,)* ) => f.write_str(#tag).and(f.write_char(':'))
                    #(.and(f.write_char(' ')).and(#c2.fmt(f)))*,
            }]
        }
    }

    //
    fn tag_arm(&self) -> syn::Arm {
        let id = &self.ident;
        let tag = self.tag();
        let pat: Vec<syn::Pat> = vec![parse_quote!(_); self.fields.len()];
        parse_quote!(#id(#(#pat,)*) => #tag)
    }

    //
    fn cardinality_arm(&self) -> syn::Arm {
        let id = &self.ident;
        let tag = self.cardinality();
        let pat: Vec<syn::Pat> = vec![parse_quote!(_); self.fields.len()];
        parse_quote!(#id(#(#pat,)*) => crate::ext::Cardinality::#tag)
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clause), supports(enum_newtype, enum_tuple))]
struct OboClauseDerive {
    ident: Ident,
    data: ast::Data<OboClauseVariant, util::Ignored>,
}

impl OboClauseDerive {

    fn variants(&self) -> &[OboClauseVariant] {
        match &self.data {
            darling::ast::Data::Enum(variants) => variants,
            _ => unreachable!("OboClauseDerive only supports enums"),
        }
    }

    fn display_impl(&self) -> syn::ItemImpl {
        let id = &self.ident;
        let arms = self.variants().iter().flat_map(|v| v.fmt_arms());
        parse_quote! {
            #[cfg_attr(feature = "_doc", doc(cfg = "display"))]
            #[automatically_derived]
            impl std::fmt::Display for #id {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    use self::#id::*;
                    match self {
                        #(#arms)*
                    }
                }
            }
        }
    }

    fn oboclause_impl(&self) -> syn::ItemImpl {
        let id = &self.ident;
        let arms_tag = self.variants().iter().map(|v| v.tag_arm());
        let arms_card = self.variants().iter().map(|v| v.cardinality_arm());

        parse_quote! {
            #[automatically_derived]
            impl crate::ast::OboClause for #id {
                fn tag(&self) -> &'static str {
                    use self::#id::*;
                    match self {
                        #(#arms_tag,)*
                    }
                }
                #[cfg(feature = "ext")]
                fn cardinality(&self) -> crate::ext::Cardinality {
                    use self::#id::*;
                    match self {
                        #(#arms_card,)*
                    }
                }
            }
        }
    }
}

#[proc_macro_derive(OboClause, attributes(clause))]
pub fn oboclause_derive(input: TokenStream) -> TokenStream {
    let parsed = syn::parse(input).unwrap();
    let receiver = OboClauseDerive::from_derive_input(&parsed).unwrap();

    let oboclause_impl = receiver.oboclause_impl();
    let display_impl = receiver.display_impl();
    // let cardinalitybound_impl = receiver.cardinalitybound_impl();

    TokenStream::from(quote!(

        #oboclause_impl

        // #[cfg(feature = "display")]
        #display_impl
    ))
}
