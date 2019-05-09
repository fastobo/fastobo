//! Identifiers used in OBO documents.
//!
//! `Ident` refers to an *owned* identifier, while `Id` refers to its *borrowed*
//! counterpart.

mod ident;
mod local;
mod prefix;
mod prefixed;
mod subclasses;
mod unprefixed;

pub use url::Url;
pub use self::ident::Ident;
pub use self::ident::Id;
pub use self::local::IdentLocal;
pub use self::local::IdLocal;
pub use self::prefix::IdentPrefix;
pub use self::prefix::IdPrefix;
pub use self::prefixed::PrefixedId;
pub use self::prefixed::PrefixedIdent;
pub use self::subclasses::ClassIdent;
pub use self::subclasses::ClassId;
pub use self::subclasses::InstanceIdent;
pub use self::subclasses::InstanceId;
pub use self::subclasses::NamespaceIdent;
pub use self::subclasses::NamespaceId;
pub use self::subclasses::RelationIdent;
pub use self::subclasses::RelationId;
pub use self::subclasses::SubsetIdent;
pub use self::subclasses::SubsetId;
pub use self::subclasses::SynonymTypeIdent;
pub use self::subclasses::SynonymTypeId;
pub use self::unprefixed::UnprefixedIdent;
pub use self::unprefixed::UnprefixedId;
