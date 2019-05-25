//! Parser and parsing-related traits for the OBO format.

use std::str::FromStr;

use pest::Parser;

use crate::error::Error;

#[macro_use]
mod macros;
mod from_pair;
mod from_slice;
mod quickfind;

#[doc(inline)]
pub use fastobo_syntax::OboParser;
#[doc(inline)]
pub use fastobo_syntax::Rule;

pub use self::from_pair::FromPair;
pub use self::from_slice::FromSlice;
pub use self::quickfind::QuickFind;
