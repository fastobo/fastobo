/// Write an escaped Obo string to the given formatter.
macro_rules! write_escaped {
    ($f:ident, $s:expr, $( $from:pat => $to:expr ),*) => ({
        use std::fmt::Write;
        match $s
            .chars()
            .map(|c| match c {
                $($from => $f.write_str($to),)*
                _ => $f.write_char(c),
            })
            .flat_map(Result::err)
            .next()
        {
            Some(err) => Err(err),
            None => Ok(()),
        }
    });
    ($f:ident, $s:expr) => {
        write_escaped!($f, $s,
            ' ' => "\\W", '\t' => "\\t", '\u{000c}' => "\\f",
            '\n' => "\\n", '\r' => "\\r"
        )
    };
    ($f:ident, $s:expr, $(+ $from:pat => $to:expr),*) => {
        write_escaped!($f, $s,
            ' ' => "\\W", '\t' => "\\t", '\u{000c}' => "\\f",
            '\n' => "\\n", '\r' => "\\r",
            $($from => $to),*
        )
    }
}
