use std::borrow::Cow;

use darling::ast::Data;
use darling::ast::Fields;
use darling::util::Ignored;
use darling::FromDeriveInput;
use heck::ToSnakeCase;
use syn::parse_quote;
use syn::Ident;
use syn::Type;

use crate::utils::is_option;

#[derive(Debug, FromVariant)]
#[darling(attributes(clause), supports(newtype, tuple))]
pub struct OboClauseVariant {
    ident: Ident,
    fields: Fields<Type>,
    #[darling(default)]
    tag: Option<syn::Lit>,
    #[darling(default)]
    cardinality: Option<syn::Path>,
    #[darling(default)]
    format: Option<syn::LitStr>,
}

impl OboClauseVariant {
    pub fn field_patterns(&self) -> Vec<syn::Pat> {
        self.fields
            .iter()
            .enumerate()
            .map(|(n, _)| {
                Ident::new(
                    &format!("__{}_{}", &self.ident.to_string().to_lowercase(), n),
                    self.ident.span(),
                )
            })
            .map(|id| parse_quote!(#id))
            .collect()
    }

    pub fn tag(&self) -> syn::Expr {
        // match
        if let Some(tag) = &self.tag {
            if let syn::Lit::Int(i) = tag {
                let id = Ident::new(
                    &format!("__{}_{}", &self.ident.to_string().to_lowercase(), i),
                    tag.span(),
                );
                parse_quote!(#id)
            } else {
                parse_quote!(#tag)
            }
        } else {
            let tag_str = self.ident.to_string().to_snake_case();
            let tag = syn::LitStr::new(&tag_str, self.ident.span());
            parse_quote!(#tag)
        }
    }

    pub fn cardinality(&self) -> Cow<syn::Path> {
        match &self.cardinality {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(parse_quote! { Any }),
        }
    }

    /// Make the arms for a particular variant to be used in `Display` impl.
    pub fn fmt_arms(&self) -> Vec<syn::Arm> {
        // Extract ident and tag to use in `quote!` calls.
        let id = &self.ident;
        let tag = self.tag();

        if let Some(fmt_string) = self.format.as_ref() {
            // If an explicit format string is given, use that string
            // directly using argument bindings from the the format tag.

            let catches: Vec<syn::Pat> = self.field_patterns();
            let c1 = &catches;
            let c2 = &catches;
            vec![parse_quote! {
                #id( #(ref #c1,)* ) => write!(f, #fmt_string, #(#c2,)*),
            }]
        } else if let Some(idx) = self.fields.iter().position(|ty| is_option(ty)) {
            // If the variant contain an option, we need two arms: one where the
            // value is `Some` and the other when the value is `None`.
            // NB: limited to a single `Option` per variant.

            // The arm pattern and expression when the field is `None`
            let mut c1_none: Vec<syn::Pat> = self.field_patterns();
            c1_none[idx] = parse_quote!(None);
            let mut c2_none: Vec<syn::Pat> = self.field_patterns();
            c2_none.remove(idx);

            // The arm pattern and expression when the field is `Some`
            let mut c1_some: Vec<syn::Pat> = self.field_patterns();
            let ident = Ident::new(
                &format!("__{}_{}", id.to_string().to_lowercase(), idx),
                self.ident.span(),
            );
            c1_some[idx] = parse_quote!(Some(#ident));
            let c2_some: Vec<syn::Pat> = self.field_patterns();

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
            let catches: Vec<syn::Pat> = self.field_patterns();
            let c1 = &catches;
            let c2 = &catches;
            vec![parse_quote! {
                #id( #(ref #c1,)* ) => f.write_str(#tag).and(f.write_char(':'))
                    #(.and(f.write_char(' ')).and(#c2.fmt(f)))*,
            }]
        }
    }

    //
    pub fn tag_arm(&self) -> syn::Arm {
        let id = &self.ident;
        let tag = self.tag();
        let pat = self.field_patterns();
        parse_quote!(#id(#(#pat,)*) => #tag)
    }

    //
    pub fn cardinality_arm(&self) -> syn::Arm {
        let id = &self.ident;
        let tag = self.cardinality();
        let pat = self.field_patterns();
        parse_quote!(#id(#(#pat,)*) => crate::semantics::Cardinality::#tag)
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clause), supports(enum_newtype, enum_tuple))]
pub struct OboClauseDerive {
    ident: Ident,
    data: Data<OboClauseVariant, Ignored>,
}

impl OboClauseDerive {
    pub fn variants(&self) -> &[OboClauseVariant] {
        match &self.data {
            Data::Enum(variants) => variants,
            _ => unreachable!("OboClauseDerive only supports enums"),
        }
    }

    pub fn display_impl(&self) -> syn::ItemImpl {
        let id = &self.ident;
        let arms = self.variants().iter().flat_map(|v| v.fmt_arms());
        parse_quote! {
            #[automatically_derived]
            impl std::fmt::Display for #id {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    use std::fmt::Write;
                    use std::fmt::Display;
                    use self::#id::*;
                    match self {
                        #(#arms)*
                    }
                }
            }
        }
    }

    pub fn oboclause_impl(&self) -> syn::ItemImpl {
        let id = &self.ident;
        let arms_tag = self.variants().iter().map(|v| v.tag_arm());
        let arms_card = self.variants().iter().map(|v| v.cardinality_arm());
        parse_quote! {
            #[automatically_derived]
            impl OboClause for #id {
                fn tag(&self) -> &str {
                    use self::#id::*;
                    match self {
                        #(#arms_tag,)*
                    }
                }
                fn cardinality(&self) -> crate::semantics::Cardinality {
                    use self::#id::*;
                    match self {
                        #(#arms_card,)*
                    }
                }
            }
        }
    }
}
