//! Parser and parsing-related traits for the OBO format.

use std::io::BufRead;
use std::iter::Iterator;
use std::num::NonZeroUsize;

use crate::ast::Frame;
use crate::error::Error;

mod from_pair;
mod from_slice;
mod quickfind;
mod sequential;
#[cfg(feature = "threading")]
mod threaded;

pub use self::from_pair::Cache;
pub use self::from_pair::FromPair;
pub use self::from_slice::FromSlice;
pub use self::quickfind::QuickFind;
pub use self::sequential::SequentialParser;
#[cfg(feature = "threading")]
pub use self::threaded::ThreadedParser;

// ---

/// The common interface for OBO parsers.
pub trait Parser<B: BufRead>:
    From<B> + Iterator<Item = Result<Frame, Error>> + AsRef<B> + AsMut<B>
{
    /// Create a new `Parser` reading from the reader.
    fn new(stream: B) -> Self {
        Self::from(stream)
    }

    /// Create a new `Parser` with the given number of threads.
    ///
    /// If multithreading is not supported by the parser, this method will
    /// default to `Self::new`.
    #[allow(unused)]
    fn with_threads(stream: B, threads: NonZeroUsize) -> Self {
        Self::new(stream)
    }

    /// Force the parser to yield frames in the order they appear in the document.
    ///
    /// Since this can have a small performance impact, parser are expected
    /// not to care about frame order by default unless forced to by this
    /// method.
    #[allow(unused)]
    fn ordered(&mut self, ordered: bool) -> &mut Self {
        self
    }

    /// Extract the internal buffered reader from the parser.
    fn into_inner(self) -> B;
}

// ---

#[cfg(feature = "threading")]
/// The default parser used by [`fastobo::from_reader`] and [`fastobo::from_file`].
///
/// [`fastobo::from_reader`]: ../fn.from_reader.html
/// [`fastobo::from_file`]: ../fn.from_file.html
pub type DefaultParser<B> = ThreadedParser<B>;

#[cfg(not(feature = "threading"))]
/// The default parser used by [`fastobo::from_reader`] and [`fastobo::from_file`].
///
/// [`fastobo::from_reader`]: ../fn.from_reader.html
/// [`fastobo::from_file`]: ../fn.from_file.html
pub type DefaultParser<B> = SequentialParser<B>;

// ---

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::convert::TryFrom;
    use std::io::Cursor;

    use textwrap_macros::dedent;

    use super::*;
    use crate::ast::OboDoc;
    use crate::error::SyntaxError;
    use crate::semantics::Identified;

    macro_rules! tests {
        ($constructor:expr) => {
            #[test]
            fn empty() {
                let res = OboDoc::try_from($constructor(Cursor::new("")));
                let doc = res.expect("document should parse properly");
                assert!(doc.header().is_empty());
                assert!(doc.entities().is_empty());
            }

            #[test]
            fn ordered() {
                let txt = dedent!(
                    r#"
                    format-version: 1.2

                    [Term]
                    id: TST:001

                    [Term]
                    id: TST:002

                    [Term]
                    id: TST:003

                    [Term]
                    id: TST:004
                    "#
                );
                let res = OboDoc::try_from($constructor(Cursor::new(&txt)).ordered(true));
                let doc = res.expect("document should parse fine");

                assert_eq!(doc.entities().len(), 4);
                assert_eq!(doc.entities()[0].as_id().to_string(), "TST:001");
                assert_eq!(doc.entities()[1].as_id().to_string(), "TST:002");
                assert_eq!(doc.entities()[2].as_id().to_string(), "TST:003");
                assert_eq!(doc.entities()[3].as_id().to_string(), "TST:004");
            }

            #[test]
            fn unordered() {
                let txt = dedent!(
                    r#"
                    format-version: 1.2

                    [Term]
                    id: TST:001

                    [Term]
                    id: TST:002

                    [Term]
                    id: TST:003

                    [Term]
                    id: TST:004
                    "#
                );
                let res = OboDoc::try_from($constructor(Cursor::new(&txt)).ordered(false));
                let doc = res.expect("document should parse fine");

                assert_eq!(doc.entities().len(), 4);
                let ids: HashSet<String> = doc
                    .entities()
                    .iter()
                    .map(|c| c.as_id().to_string())
                    .collect();

                assert!(ids.contains("TST:001"));
                assert!(ids.contains("TST:002"));
                assert!(ids.contains("TST:003"));
                assert!(ids.contains("TST:004"));
            }

            mod errloc {
                use super::*;

                use pest::error::InputLocation;
                use pest::error::LineColLocation;

                #[test]
                fn invalid_header_date() {
                    let txt = "format-version: 1.4\ndate: nope";
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
                    let res = OboDoc::try_from($constructor(Cursor::new(&txt)));
                    let err = res.expect_err("document should fail to parse");

                    let se = match err {
                        Error::SyntaxError { error: se } => se,
                        _ => panic!("error should be a SyntaxError"),
                    };

                    let pe = match se {
                        SyntaxError::ParserError { error: pe } => pe,
                        _ => panic!("syntax error should be a ParserError"),
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
        };
    }

    mod sequential {
        use super::*;
        tests!(SequentialParser::new);
    }

    #[cfg(feature = "threading")]
    mod threaded {
        use super::*;
        tests!(ThreadedParser::new);
    }
}
