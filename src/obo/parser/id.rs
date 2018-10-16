use super::super::ast::ClassId;
use super::super::ast::Id;
use super::super::ast::InstanceId;
use super::super::ast::PersonId;
use super::super::ast::RelationId;
use super::chars::nonws_char;
use super::chars::OboChar;

// /// Parse an OBO identifier using the sub-macro to recognize input.
// macro_rules! obo_id (
//
//   ($i:expr, $prefix:ident!( $($args_p:tt)* ), $suffix:ident!( $($args_s:tt)* ) ) => (
//       ({
//             match alt!($i, tag!("http:") | tag!("https")) {
//                 Err(e) => Err(e),
//                 Ok((rest1, scheme)) => {
//                     match many0!(rest1, $suffix!($($args_s)*)) {
//                         Err(e) => Err(e),
//                         Ok((rest2, path)) => {
//                             let mut _url = String::with_capacity(path.len() + scheme.len());
//                             _url.push_str(scheme);
//                             for x in path {
//                                 match x {
//                                     OboChar::Unescaped(c) => _url.push(c),
//                                     OboChar::Escaped(c) => _url.push(c),
//                                 }
//                             }
//                             Ok((rest2, Id::Url(_url)))
//                         }
//                     }
//                 }
//             }
//       })
//
//   );
//
//   ($i:expr, $submac:ident!( $($args:tt)* ) ) => (
//       obo_id!($i, $submac!($($args)*))
//   );
//
//   ($i:expr, $prefix:expr, $suffix:expr) => (
//       obo_id!($i, call!($prefix), call!($suffix))
//   )
// );

// named!(dbxref_id<&str, Id>,
//     obo_id!(call!(nonws_char), call!(nonws_char))
// );

/// Parse an URL identifier.
named!(url_id<&str, Id>,
    do_parse!(
        s:  alt!(tag!("http") | tag!("https"))        >>
            tag!(":")                                 >>
        p:  many0!(complete!(nonws_char))             >>
            // Unescape using vector of OboChars
            ({
                use crate::obo::parser::chars::OboChar;
                let mut url = String::with_capacity(p.len() + 6);
                url.push_str(s);
                url.push(':');
                for &c in p.iter() {
                    match c {
                        OboChar::Escaped(e) => url.push(e),
                        OboChar::Unescaped(e) => url.push(e),
                    }
                }
                Id::Url(url)
            })
    )
);

/// Parse a prefixed identifier.
named!(prefixed_id<&str, Id>,
    do_parse!(
        pre:    many0!(verify!(nonws_char, |c| c != OboChar::Unescaped(':'))) >>
                tag!(":")                                                     >>
        loc:    many0!(complete!(nonws_char))                                 >>
                (Id::Prefixed(
                    pre.iter().collect(),
                    loc.iter().collect()
                ))
    )
);

/// Parse an unprefixed identifier.
named!(unprefixed_id<&str, Id>,
    map!(
        many0!(verify!(complete!(nonws_char), |c| c != OboChar::Unescaped(':'))),
        |chars| Id::Unprefixed(chars.iter().collect())
    )
);

/// Parse an identifier.
named!(pub id<&str, Id>,
    alt_complete!(url_id | prefixed_id | unprefixed_id)
);

named!(pub class_id<&str, ClassId>, map!(id, |i| ClassId(i)));
named!(pub instance_id<&str, InstanceId>, map!(id, |i| InstanceId(i)));
named!(pub relation_id<&str, RelationId>, map!(id, |i| RelationId(i)));
named!(pub person_id<&str, PersonId>, map!(id, |i| PersonId(i)));

#[cfg(test)]
mod tests {
    use super::*;

    mod url_id {
        use super::*;

        #[test]
        fn http() {
            let (r, id) = url_id("http://dx.doi.org/").expect("parser failed");
            assert_eq!(r, "");
            assert_eq!(id, Id::Url("http://dx.doi.org/".to_string()));
        }

        #[test]
        fn prefixed() {
            let (r, id) = prefixed_id("PSI:MS").expect("parser failed");
            assert_eq!(r, "");
            assert_eq!(id, Id::Prefixed("PSI".to_string(), "MS".to_string()));
        }
    }
}
