macro_rules! tvp_h {
    ($name:ident, $tag:literal, $clause:path) => {
        fn $name(i: &str) -> $crate::nom::IResult<&str, HeaderClause> {
            do_parse!(
                i,
                tag!($tag)                                        >>
                                opt!(ws)                                          >>
                            v:  unquoted_unescape                                 >>
                                // Trim right because we don't want any extra
                                // space but the greedy `unquoted` parser will
                                // get them anyway
                                ($clause(v.trim_right().to_string()))
            )
        }
    };
}
