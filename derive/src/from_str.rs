use darling::FromDeriveInput;
use syn::parse_quote;
use syn::Ident;

#[derive(Debug, FromDeriveInput)]
pub struct FromStrDerive {
    ident: Ident,
}

impl FromStrDerive {
    pub fn fromstr_impl(&self) -> syn::ItemImpl {
        let id = &self.ident;
        parse_quote! {
            impl std::str::FromStr for #id {
                type Err = crate::error::SyntaxError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    use crate::error::Error;
                    use crate::syntax::Lexer;
                    use crate::pest::error::ErrorVariant;
                    use crate::pest::Parser;
                    use crate::pest::Position;

                    // Parse the input string
                    let mut pairs = Lexer::tokenize(Self::RULE, s)?;
                    let pair = pairs.next().unwrap();
                    // Check EOI was reached
                    if pair.as_span().end() != s.len() {
                        let span = pair
                            .as_span()
                            .end_pos()
                            .span(&Position::new(s, s.len()).unwrap());
                        let variant = ErrorVariant::CustomError {
                            message: "remaining input".to_string(),
                        };
                        Err(crate::pest::error::Error::new_from_span(variant, span).into())
                    } else {
                        let cache = crate::parser::Cache::default();
                        unsafe { <Self as FromPair>::from_pair_unchecked(pair, &cache) }
                    }
                }
            }
        }
    }
}
