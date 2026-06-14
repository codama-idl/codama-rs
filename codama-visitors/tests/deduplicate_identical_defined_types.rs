use codama_nodes::{
    DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8,
};
use codama_visitors::deduplicate_identical_defined_types;
use pretty_assertions::assert_eq;

fn struct_foo() -> StructTypeNode {
    StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))])
}

fn defined_type_names(program: &ProgramNode) -> Vec<String> {
    program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect()
}

#[test]
fn removes_identical_duplicates_keeping_the_first() {
    let mut a = ProgramNode::new("a", "Myprogram1111111111111111111111111111111111");
    a.defined_types
        .push(DefinedTypeNode::new("foo", struct_foo()));
    a.defined_types
        .push(DefinedTypeNode::new("bar", NumberTypeNode::le(U8)));

    let mut b = ProgramNode::new("b", "Bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
    // Same name + structure as a.foo (docs differ) -> duplicate.
    let mut foo_with_docs = DefinedTypeNode::new("foo", struct_foo());
    foo_with_docs.docs = vec!["a copy".to_string()].into();
    b.defined_types.push(foo_with_docs);
    b.defined_types
        .push(DefinedTypeNode::new("baz", NumberTypeNode::le(U8)));

    let mut root = RootNode::new(a);
    root.additional_programs.push(b);

    let root = deduplicate_identical_defined_types(root);

    // a keeps both; b's duplicate `foo` is gone (docs ignored), `baz` stays.
    assert_eq!(defined_type_names(&root.program), vec!["foo", "bar"]);
    assert_eq!(
        defined_type_names(&root.additional_programs[0]),
        vec!["baz"]
    );
}

#[test]
fn keeps_same_named_types_that_differ_structurally() {
    let mut a = ProgramNode::new("a", "Myprogram1111111111111111111111111111111111");
    a.defined_types
        .push(DefinedTypeNode::new("foo", struct_foo()));
    let mut b = ProgramNode::new("b", "Bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
    // Same name but different type -> NOT a duplicate.
    b.defined_types
        .push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));

    let mut root = RootNode::new(a);
    root.additional_programs.push(b);

    let root = deduplicate_identical_defined_types(root);
    assert_eq!(defined_type_names(&root.program), vec!["foo"]);
    assert_eq!(
        defined_type_names(&root.additional_programs[0]),
        vec!["foo"]
    );
}
