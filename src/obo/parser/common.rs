//! Parse synonym scopes.
use super::ast::SynonymScope;

/// Parse a `SynonymScope` token.
pub fn synonym_scope(i: &str) -> nom::IResult<&str, SynonymScope> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    alt!(i, value!(SynonymScope::Exact, tag!("EXACT"))
        |   value!(SynonymScope::Broad, tag!("BROAD"))
        |   value!(SynonymScope::Narrow, tag!("NARROW"))
        |   value!(SynonymScope::Related, tag!("RELATED"))
    )
}
