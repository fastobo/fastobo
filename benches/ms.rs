#![feature(test)]

extern crate test;
extern crate fastobo;

use std::io::Cursor;
use std::io::BufRead;
use std::convert::TryFrom;

#[bench]
fn bench_baseline_readline(b: &mut test::Bencher) {
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| {
        std::io::BufReader::new(Cursor::new(&s)).lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    });
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
fn bench_baseline_lexer(b: &mut test::Bencher) {
    use fastobo::parser::OboParser;
    use fastobo::parser::Rule;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| OboParser::parse(Rule::OboDoc, &s).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
fn bench_sequential(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::SequentialReader;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();
    b.iter(|| OboDoc::try_from(SequentialReader::new(Cursor::new(&s))).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
#[cfg(feature = "threading")]
fn bench_threaded(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::ThreadedReader;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();

    b.iter(|| OboDoc::try_from(ThreadedReader::new(Cursor::new(&s))).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}

#[bench]
#[cfg(feature = "threading")]
fn bench_threaded_ordered(b: &mut test::Bencher) {
    use fastobo::ast::OboDoc;
    use fastobo::parser::ThreadedReader;
    let s = std::fs::read_to_string("tests/data/ms.obo").unwrap();

    b.iter(|| OboDoc::try_from(ThreadedReader::new(Cursor::new(&s)).ordered(true)).unwrap());
    b.bytes = s.as_bytes().len() as u64;
}
