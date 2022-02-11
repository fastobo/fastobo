//! Identifiers used in OBO documents.
//!
//! `Ident` refers to an *owned* identifier, while `Id` refers to its *borrowed*
//! counterpart.

use std::fmt::Error as FmtError;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

mod ident;
mod prefix;
mod prefixed;
mod subclasses;
mod unprefixed;
mod url;

pub use self::ident::Ident;
pub use self::prefix::IdentPrefix;
pub use self::prefixed::PrefixedIdent;
pub use self::subclasses::ClassIdent;
pub use self::subclasses::InstanceIdent;
pub use self::subclasses::NamespaceIdent;
pub use self::subclasses::RelationIdent;
pub use self::subclasses::SubsetIdent;
pub use self::subclasses::SynonymTypeIdent;
pub use self::unprefixed::UnprefixedIdent;
pub use self::url::Url;

fn escape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    s.chars().try_for_each(|char| match char {
        '\r' => f.write_str("\\r"),
        '\n' => f.write_str("\\n"),
        '\u{000c}' => f.write_str("\\f"),
        ' ' => f.write_str("\\ "),
        '\t' => f.write_str("\\t"),
        ':' => f.write_str("\\:"),
        '"' => f.write_str("\\\""),
        '\\' => f.write_str("\\\\"),
        _ => f.write_char(char),
    })
}

fn unescape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    let mut chars = s.chars();
    while let Some(char) = chars.next() {
        if char == '\\' {
            match chars.next() {
                Some('r') => f.write_char('\r')?,
                Some('n') => f.write_char('\n')?,
                Some('f') => f.write_char('\u{000c}')?,
                Some('t') => f.write_char('\t')?,
                Some(other) => f.write_char(other)?,
                None => return Err(FmtError),
            }
        } else {
            f.write_char(char)?;
        }
    }
    Ok(())
}
