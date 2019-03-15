use iri_string::AbsoluteIriString;
use iri_string::RelativeIriString;

use super::Id;
use super::QuotedString;
use super::RelationId;

/// An Internationalized Resource Identifier, either absolute or relative.
pub enum Iri {
    Absolute(AbsoluteIriString),
    Relative(RelativeIriString),
}

impl From<AbsoluteIriString> for Iri {
    fn from(abs: AbsoluteIriString) -> Self {
        Iri::Absolute(abs)
    }
}

impl From<RelativeIriString> for Iri {
    fn from(rel: RelativeIriString) -> Self {
        Iri::Relative(rel)
    }
}

/// A clause value binding a property to a value in the relevant entity.
pub enum PropertyValue {
    Identified(RelationId, Id),
    // FIXME(@althonos): maybe replaced `String` with `DatatypeId` newtype.
    Typed(RelationId, QuotedString, String),
}

/// A qualifier, possibly used as a trailing modifier.
pub struct Qualifier {
    key: RelationId,
    value: QuotedString,
}

/// A database cross-reference definition.
pub struct Xref {
    id: Id,
    desc: Option<QuotedString>,
}
