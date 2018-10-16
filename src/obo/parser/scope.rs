use super::ast::SynonymScope;
use super::ast::SynonymScope::*;

named!(pub synonym_scope<&str, SynonymScope>,
    alt!(
            value!(Exact, tag!("EXACT"))
        |   value!(Broad, tag!("BROAD"))
        |   value!(Narrow, tag!("NARROW"))
        |   value!(Related, tag!("RELATED"))
    )
);
