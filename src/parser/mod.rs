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
/// The default frame reader used by `fastobo`.
pub type FrameReader<B> = ThreadedReader<B>;

#[cfg(not(feature = "threading"))]
/// The default frame reader used by `fastobo`.
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
                let res = $constructor(Cursor::new(""));
                let doc = res.expect("document should parse properly");
                assert!(doc.header().is_empty());
                assert!(doc.entities().is_empty());
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
                let res = $constructor(Cursor::new(&txt));
                let doc = res.expect("document should parse fine");

                assert_eq!(doc.entities().len(), 2);
                assert_eq!(doc.entities()[0].as_id().to_string(), "TST:001");
                assert_eq!(doc.entities()[1].as_id().to_string(), "TST:002");
            }

            mod errloc {
                use super::*;

                use pest::error::LineColLocation;
                use pest::error::InputLocation;

                #[test]
                fn invalid_header_date() {
                    let txt = "format-version: 1.4\ndate: nope";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

                    match pe.line_col {
                        LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                        LineColLocation::Pos((l, c)) => {
                            assert_eq!(l, 2);
                            assert_eq!(c, 7);
                        }
                    }
                    match pe.location {
                        InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                        InputLocation::Pos(s) => {
                            assert_eq!(s, 26);
                        }
                    }
                }

                #[test]
                fn invalid_header_date_indented() {
                    let txt = "format-version: 1.4\n  date: nope";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

                    match pe.line_col {
                        LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                        LineColLocation::Pos((l, c)) => {
                            assert_eq!(l, 2);
                            assert_eq!(c, 9);
                        }
                    }
                    match pe.location {
                        InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                        InputLocation::Pos(s) => {
                            assert_eq!(s, 28);
                        }
                    }
                }

                #[test]
                fn invalid_frame_def() {
                    let txt = "[Term]\nid: OK\ndef: no quote\n";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

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
                }

                #[test]
                fn invalid_frame_def_2() {
                    let txt = "[Term]\nid: OK\n\n[Term]\nid: NO\ndef: no quote\n";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

                    match pe.line_col {
                        LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                        LineColLocation::Pos((l, c)) => {
                            assert_eq!(l, 6);
                            assert_eq!(c, 6);
                        }
                    }
                    match pe.location {
                        InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                        InputLocation::Pos(s) => {
                            assert_eq!(s, 34);
                        }
                    }
                }

                #[test]
                fn invalid_frame_def_indented() {
                    let txt = "[Term]\nid: OK\n   def: no quote\n";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

                    match pe.line_col {
                        LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                        LineColLocation::Pos((l, c)) => {
                            assert_eq!(l, 3);
                            assert_eq!(c, 9);
                        }
                    }
                    match pe.location {
                        InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                        InputLocation::Pos(s) => {
                            assert_eq!(s, 22);
                        }
                    }
                }

                #[test]
                fn invalid_frame_def_indented_2() {
                    let txt = "[Term]\nid: OK\n\n[Term]\nid: NO\n   def: no quote\n";
                    let res = $constructor(Cursor::new(&txt));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError")
                    };

                    match pe.line_col {
                        LineColLocation::Span(_, _) => panic!("position should be `pos`"),
                        LineColLocation::Pos((l, c)) => {
                            assert_eq!(l, 6);
                            assert_eq!(c, 9);
                        }
                    }
                    match pe.location {
                        InputLocation::Span((_, _)) => panic!("location should be `pos`"),
                        InputLocation::Pos(s) => {
                            assert_eq!(s, 37);
                        }
                    }
                }
            }
        }
    }

    mod sequential {
        use super::*;
        tests!(|x| OboDoc::try_from(SequentialReader::new(x)));
    }

    #[cfg(feature = "threading")]
    mod threaded {
        use super::*;
        tests!(|stream|
            OboDoc::try_from(ThreadedReader::with_threads(
                stream,
                std::num::NonZeroUsize::new(1).unwrap()
            ))
        );
    }
}
