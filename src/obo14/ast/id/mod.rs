#[macro_use]
mod _macros;

use iri_string::Url;


/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Debug)]
pub enum Id {
    Prefixed(PrefixedId),
    Unprefixed(UnprefixedId),
    Url(Url),
}

/// An identifier without a prefix.
#[derive(Debug)]
pub struct UnprefixedId {
    value: String,
}

/// An identifier with a prefix.
#[derive(Debug)]
pub struct PrefixedId {
    prefix: IdPrefix,
    local: IdLocal,
}

/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
#[derive(Debug)]
pub enum IdPrefix {
    Canonical(String),
    NonCanonical(String),
}

/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
#[derive(Debug)]
pub enum IdLocal {
    Canonical(String),
    NonCanonical(String),
}


// NB(@althonos): All identifiers are defined as separated typedefs so that
//                `PartialEq` is not implemented and trying to compare a
//                `ClassId` with a `RelationId` would fail at compile-time.
id_subclasses! {
    /// A unique identifier for a class (*i.e.* a term).
    pub struct ClassId;

    /// A unique identifier for a typedef (*i.e.* a relation).
    pub struct RelationId;

    /// A unique identifier for an instance.
    pub struct InstanceId;

    /// A unique identifier for a subset
    pub struct SubsetId;

    /// A unique identifier for a person (used in the `created_by` clause).
    pub struct PersonId;

    /// A unique identifier for a synonym type.
    pub struct SynonymTypeId;

    /// An OBO namespace.
    pub struct NamespaceId;
}
