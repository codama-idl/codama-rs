use codama_nodes::{
    AccountNode, DefinedTypeLinkNode, DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, TypeNode, U8,
};
use codama_visitors::{unwrap_defined_types, unwrap_defined_types_named};
use pretty_assertions::assert_eq;

fn link(name: &str) -> DefinedTypeLinkNode {
    DefinedTypeLinkNode {
        name: name.into(),
        program: None,
    }
}

/// foo = struct{x:u8}; bar = struct{f: link(foo)}; account acc{ y: link(foo) }.
fn sample_root() -> RootNode {
    let foo_struct =
        StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))]);
    let bar_struct = StructTypeNode::new(vec![StructFieldTypeNode::new("f", link("foo"))]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("foo", foo_struct));
    program
        .defined_types
        .push(DefinedTypeNode::new("bar", bar_struct));
    program.accounts.push(AccountNode::new(
        "acc",
        StructTypeNode::new(vec![StructFieldTypeNode::new("y", link("foo"))]),
    ));
    RootNode::new(program)
}

fn account_field_type(root: &RootNode) -> TypeNode {
    use codama_nodes::NestedTypeNodeTrait;
    let data: &StructTypeNode = root.program.accounts[0].data.get_nested_type_node();
    (*data.fields[0].r#type).clone()
}

#[test]
fn unwrap_all_inlines_everywhere_and_removes_defined_types() {
    let root = unwrap_defined_types(sample_root());

    assert!(root.program.defined_types.is_empty());
    // account `acc.y` is now `foo`'s body (a struct), not a link.
    assert!(matches!(account_field_type(&root), TypeNode::Struct(_)));
}

#[test]
fn unwrap_named_keeps_others_but_rewrites_their_links() {
    let root = unwrap_defined_types_named(sample_root(), ["foo"]);

    // `foo` removed; `bar` kept.
    let names: Vec<_> = root
        .program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect();
    assert_eq!(names, vec!["bar".to_string()]);

    // `bar.f` had a link to foo, now inlined to foo's struct body.
    match &*root.program.defined_types[0].r#type {
        TypeNode::Struct(bar) => assert!(matches!(&*bar.fields[0].r#type, TypeNode::Struct(_))),
        other => panic!("expected struct, got {other:?}"),
    }
    // account link also inlined.
    assert!(matches!(account_field_type(&root), TypeNode::Struct(_)));
}
