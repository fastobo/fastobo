extern crate fastobo;

use fastobo::ast::*;

#[test]
fn assign_namespaces_no_default_namespace() {
    // build an ontology without a `default-namespace` clause in which
    // all entities have a namespace
    let mut doc = OboDoc::new();
    for i in 0..10 {
        let ns = NamespaceIdent::from(UnprefixedIdent::new("test_namespace"));
        let id = PrefixedIdent::new("GO", format!("{:08}", i));
        let mut frame = TermFrame::new(Line::from(ClassIdent::from(id)));
        frame
            .clauses_mut()
            .push(Line::from(TermClause::Namespace(Box::new(ns))));
        doc.entities_mut().push(EntityFrame::from(frame));
    }

    // assign namesapces shouldn't fail
    doc.assign_namespaces()
        .expect("all frames have a namespace so `default-namespace` is not required");

    // now add a frame without a namespace
    let id = PrefixedIdent::new("GO", format!("{:08}", 10));
    let frame = TermFrame::new(Line::from(ClassIdent::from(id)));
    doc.entities_mut().push(EntityFrame::from(frame));

    // now assign namesapces should fail
    doc.assign_namespaces()
        .expect_err("one frame is missing a namespace so this shouldn't succeed");
}
