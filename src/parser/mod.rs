//! Parser and parsing-related traits for the OBO format.

use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;
use std::str::FromStr;

use pest::Parser;

use crate::ast::EntityFrame;
use crate::ast::HeaderClause;
use crate::ast::HeaderFrame;
use crate::ast::OboDoc;
use crate::error::Error;
use crate::error::SyntaxError;

#[macro_use]
mod macros;
mod from_pair;
mod from_slice;
mod quickfind;
mod sequential;
#[cfg(feature = "threading")]
mod threaded;
#[cfg(feature = "threading")]
mod consumer;

#[doc(inline)]
pub use fastobo_syntax::OboParser;
#[doc(inline)]
pub use fastobo_syntax::Rule;
pub use self::from_pair::FromPair;
pub use self::from_slice::FromSlice;
pub use self::quickfind::QuickFind;
pub use self::sequential::SequentialReader;
#[cfg(feature = "threading")]
pub use self::threaded::ThreadedReader;

// ---

#[cfg(feature = "threading")]
pub type FrameReader<B> = ThreadedReader<B>;

#[cfg(not(feature = "threading"))]
pub type FrameReader<B> = SequentialReader<B>;

// ---

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use textwrap_macros::dedent;

    use super::*;
    use crate::semantics::Identified;

    mod sequential {
        use super::*;

        #[test]
        fn empty() {
            let mut reader = SequentialReader::new(Cursor::new(""));
            assert!(reader.next().is_none());
            assert!(reader.header().unwrap().is_empty());
        }

        #[test]
        fn two_frames() {
            let txt = dedent!(
                r#"
                format-version: 1.2

                [Term]
                id: TST:001

                [Term]
                id: TST:002
                "#
            );
            let mut reader = SequentialReader::new(Cursor::new(&txt));
            assert_eq!(
                reader.next().unwrap().unwrap().as_id().to_string(),
                "TST:001"
            );
            assert_eq!(
                reader.next().unwrap().unwrap().as_id().to_string(),
                "TST:002"
            );
            assert!(reader.next().is_none());
        }
    }

    #[cfg(feature = "threading")]
    mod threaded {
        use super::*;

        #[test]
        fn empty() {
            let n = std::num::NonZeroUsize::new(1).unwrap();
            let mut reader = ThreadedReader::with_threads(Cursor::new(""), n);
            assert!(reader.next().is_none());
            assert!(reader.header().unwrap().is_empty());
        }

        #[test]
        fn two_frames() {
            let txt = dedent!(
                r#"
                format-version: 1.2

                [Term]
                id: TST:001

                [Term]
                id: TST:002
                "#
            );
            let n = std::num::NonZeroUsize::new(1).unwrap();
            let mut reader = ThreadedReader::with_threads(Cursor::new(&txt), n);
            assert_eq!(
                reader.next().unwrap().unwrap().as_id().to_string(),
                "TST:001"
            );
            assert_eq!(
                reader.next().unwrap().unwrap().as_id().to_string(),
                "TST:002"
            );
            assert!(reader.next().is_none());
        }
    }
}
