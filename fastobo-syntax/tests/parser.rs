
extern crate fastobo_syntax;
use fastobo_syntax::OboParser;
use fastobo_syntax::Rule;


macro_rules! test_parse {
    ($rule:ident, $input:literal) => ({
        match OboParser::parse(Rule::$rule, $input) {
            Ok(_) => (),
            Err(e) => panic!("could not parse {:?}:\n{}", $input, e),
        }
    })
}

#[test]
fn header_clause() {
    test_parse!(
        HeaderClause,
        "treat-xrefs-as-reverse-genus-differentia: TEST part_of something"
    )
}

#[test]
fn qualifier_list() {
    test_parse!(
        QualifierList,
        r#"{comment="NYBG:Dario_Cavaliere", comment="NYBG:Brandon_Sinn"}"#
    )
}


#[test]
fn qualifier() {
    test_parse!(
        Qualifier,
        r#"comment="NYBG:Dario_Cavaliere""#
    )
}
