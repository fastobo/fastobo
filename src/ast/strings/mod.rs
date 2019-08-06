use std::fmt::Result as FmtResult;
use std::fmt::Write;

mod quoted;
mod unquoted;

pub use self::quoted::*;
pub use self::unquoted::*;
