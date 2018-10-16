//! Simple macros for common parsers implementations.

/// Helper private macro to define clause parser.
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! tvp {
    ($name:ident, $tag:literal, $clause:path, $ret:ty) => (
        pub fn $name(i: &str) -> $crate::nom::IResult<&str, $ret> {
            use $crate::obo::parser::spacing::ws;
            use $crate::obo::parser::values::unquoted_unescape;
            do_parse!(i,
                    tag!($tag)                                        >>
                    opt!(ws)                                          >>
                v:  unquoted_unescape                                 >>
                    // Trim right because we don't want any extra
                    // space but the greedy `unquoted` parser will
                    // get them anyway
                    ($clause(v.trim_right().to_string()))
            )
        }
    )
}

/// Helper private macros to define boolean clause parser.
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! bt {
    ($name:ident, $tag:literal, $clause:path, $ret:ty) => {
        pub fn $name(i: &str) -> $crate::nom::IResult<&str, $ret> {
            use $crate::obo::parser::spacing::ws;
            use $crate::obo::parser::values::boolean;
            do_parse!(i,
                    tag!($tag)                                        >>
                    opt!(ws)                                          >>
                b:  boolean                                           >>
                    ($clause(b))
            )
        }
    }
}
