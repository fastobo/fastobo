#![feature(test)]

extern crate fastobo;
extern crate test;

use std::convert::TryFrom;
use std::io::BufRead;
use std::io::Cursor;

use fastobo::parser::Parser;

#[bench]
fn bench_baseline_readline(b: &mut test::Bencher) {
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| {
        std::io::BufReader::new(Cursor::new(&s))
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    });
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
fn bench_baseline_lexer(b: &mut test::Bencher) {
    use fastobo::syntax::Lexer;
    use fastobo::syntax::Rule;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| Lexer::tokenize(Rule::OboDoc, &s).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
fn bench_sequential(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::SequentialParser;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| OboDoc::try_from(SequentialParser::new(Cursor::new(&s))).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
#[cfg(feature = "threading")]
fn bench_threaded(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::ThreadedParser;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();

    b.iter(|| OboDoc::try_from(ThreadedParser::new(Cursor::new(&s))).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
#[cfg(feature = "threading")]
fn bench_threaded_ordered(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::ThreadedParser;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();

    b.iter(|| OboDoc::try_from(ThreadedParser::new(Cursor::new(&s)).ordered(true)).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}
