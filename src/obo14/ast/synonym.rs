use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use super::QuotedString;
use super::SynonymTypeId;
use super::Xref;

/// A synonym scope specifier.
pub enum SynonymScope {
    Exact,
    Broad,
    Narrow,
    Related,
}

impl Display for SynonymScope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::SynonymScope::*;
        match self {
            Exact => f.write_str("EXACT"),
            Broad => f.write_str("BROAD"),
            Narrow => f.write_str("NARROW"),
            Related => f.write_str("RELATED"),
        }
    }
}

/// A synonym, denoting an alternative name for the embedding entity.
pub struct Synonym {
    text: QuotedString,
    scope: SynonymScope,
    syntype: Option<SynonymTypeId>,
    xrefs: Option<Vec<Xref>>,
}
