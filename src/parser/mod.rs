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
use crate::ast::TermFrame;
use crate::ast::TypedefFrame;
use crate::ast::InstanceFrame;
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


    macro_rules! tests {
        ($constructor:expr) => {
            #[test]
            fn empty() {
                let mut reader = $constructor(Cursor::new(""));
                let header = reader.next().unwrap().unwrap().into_header_frame().unwrap();
                assert!(header.is_empty());
                assert!(reader.next().is_none());
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
                let mut reader = $constructor(Cursor::new(&txt));
                reader.next().unwrap();
                assert_eq!(
                    reader.next().unwrap().unwrap().into_entity_frame().unwrap().as_id().to_string(),
                    "TST:001"
                );
                assert_eq!(
                    reader.next().unwrap().unwrap().into_entity_frame().unwrap().as_id().to_string(),
                    "TST:002"
                );
                assert!(reader.next().is_none());
            }

            mod errloc {
                pub use super::*;

                #[test]
                fn invalid_def_1() {
                    use pest::error::LineColLocation;
                    use pest::error::InputLocation;

                    let txt = "[Term]\nid: OK\ndef: no quote\n";

                    let mut reader = $constructor(Cursor::new(&txt));
                    reader.next().expect("header should be read")
                        .expect("header should read fine");

                    let err = reader.next().expect("somethhing should be produced")
                        .expect_err("error should be produced");

                    if let Error::SyntaxError { error: se } = err {
                        if let SyntaxError::ParserError { error: pe } = se {
                            match pe.line_col {
                                LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                                LineColLocation::Pos((l, c)) => {
                                    assert_eq!(l, 3);
                                    assert_eq!(c, 6);
                                }
                            }
                            match pe.location {
                                InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                                InputLocation::Pos(s) => {
                                    assert_eq!(s, 19);
                                }
                            }
                        } else {
                            panic!("error should be a parser error");
                        }
                    } else {
                        panic!("error should be a syntax error")
                    }
                }

                #[test]
                #[ignore]
                fn invalid_def_2() {
                    use pest::error::LineColLocation;
                    use pest::error::InputLocation;

                    let txt = "[Term]\nid: OK\n\n[Term]\nid: NO\ndef: no quote\n";

                    let mut reader = $constructor(Cursor::new(&txt));
                    reader.next().expect("header should be read")
                        .expect("header should read fine");
                    reader.next().expect("first frame should be read")
                        .expect("first frame should read fine");

                    let err = reader.next().expect("somethhing should be produced")
                        .expect_err("error should be produced");

                    if let Error::SyntaxError { error: se } = err {
                        if let SyntaxError::ParserError { error: pe } = se {
                            match pe.line_col {
                                LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                                LineColLocation::Pos((l, c)) => {
                                    assert_eq!(l, 6, "line position differs");
                                    assert_eq!(c, 6, "column position differs");
                                }
                            }
                            match pe.location {
                                InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                                InputLocation::Pos(s) => {
                                    assert_eq!(s, 34);
                                }
                            }
                        } else {
                            panic!("error should be a parser error");
                        }
                    } else {
                        panic!("error should be a syntax error")
                    }
                }
            }
        }
    }

    mod sequential {
        use super::*;
        tests!(|x| SequentialReader::new(x));
    }

    #[cfg(feature = "threading")]
    mod threaded {
        use super::*;
        tests!(|stream|
            ThreadedReader::with_threads(
                stream,
                std::num::NonZeroUsize::new(1).unwrap()
            )
        );
    }
}
