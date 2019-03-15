
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

/// A synonym, denoting an alternative name for the embedding entity.
pub struct Synonym {
    text: QuotedString,
    scope: SynonymScope,
    syntype: Option<SynonymTypeId>,
    xrefs: Option<Vec<Xref>>,
}
