use codama_nodes::{
    DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode,
    TypeNode, U8,
};
use codama_visitors::{transform_defined_types_into_accounts, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let counter = StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("counter", counter));
    program
        .defined_types
        .push(DefinedTypeNode::new("scalar", NumberTypeNode::le(U8)));
    RootNode::new(program)
}

#[test]
fn moves_named_struct_into_an_account() {
    let root = transform_defined_types_into_accounts(["counter"]).visit_root(sample_root());

    // `counter` is now an account...
    assert_eq!(root.program.accounts.len(), 1);
    let account = &root.program.accounts[0];
    assert_eq!(account.name.as_ref(), "counter");
    assert!(account.discriminators.is_empty());
    assert_eq!(account.size, None);

    // ...and removed from defined types; `scalar` stays.
    let names: Vec<_> = root
        .program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect();
    assert_eq!(names, vec!["scalar".to_string()]);
}

#[test]
fn non_struct_named_types_are_left_as_defined_types() {
    // `scalar` is a number, not a struct -> cannot become an account.
    let root = transform_defined_types_into_accounts(["scalar"]).visit_root(sample_root());
    assert!(root.program.accounts.is_empty());
    match &*root
        .program
        .defined_types
        .iter()
        .find(|d| d.name.as_ref() == "scalar")
        .unwrap()
        .r#type
    {
        TypeNode::Number(_) => {}
        other => panic!("expected number, got {other:?}"),
    }
}
