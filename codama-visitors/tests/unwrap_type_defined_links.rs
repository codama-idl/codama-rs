use codama_nodes::{
    AccountNode, DefinedTypeLinkNode, DefinedTypeNode, NestedTypeNodeTrait, NumberTypeNode,
    ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, TypeNode, U8,
};
use codama_visitors::unwrap_type_defined_links;
use pretty_assertions::assert_eq;

fn link(name: &str) -> DefinedTypeLinkNode {
    DefinedTypeLinkNode {
        name: name.into(),
        program: None,
    }
}

/// foo = u8; bar = struct{ inner: link(foo) }; account acc{ y: link(foo), z: link(bar) }.
fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("foo", NumberTypeNode::le(U8)));
    program.defined_types.push(DefinedTypeNode::new(
        "bar",
        StructTypeNode::new(vec![StructFieldTypeNode::new("inner", link("foo"))]),
    ));
    program.accounts.push(AccountNode::new(
        "acc",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("y", link("foo")),
            StructFieldTypeNode::new("z", link("bar")),
        ]),
    ));
    RootNode::new(program)
}

fn account_fields(root: &RootNode) -> Vec<TypeNode> {
    let data: &StructTypeNode = root.program.accounts[0].data.get_nested_type_node();
    data.fields.iter().map(|f| (*f.r#type).clone()).collect()
}

fn defined_type_names(root: &RootNode) -> Vec<String> {
    root.program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect()
}

#[test]
fn rewrites_matching_links_and_keeps_defined_types() {
    let root = unwrap_type_defined_links(sample_root(), ["foo"]);

    // Both defined types are kept (no removal).
    assert_eq!(
        defined_type_names(&root),
        vec!["foo".to_string(), "bar".to_string()]
    );

    let fields = account_fields(&root);
    // `acc.y` was a link to foo -> now foo's number body.
    assert!(matches!(fields[0], TypeNode::Number(_)));
    // `acc.z` was a link to bar -> untouched (bar not selected).
    assert!(matches!(fields[1], TypeNode::Link(_)));
}

#[test]
fn is_single_level_links_inside_inlined_body_are_not_rewritten() {
    // Selecting `bar` inlines bar's struct body, but the foo link *inside* that
    // body is left as-is (single level only).
    let root = unwrap_type_defined_links(sample_root(), ["bar"]);

    let fields = account_fields(&root);
    match &fields[1] {
        TypeNode::Struct(s) => assert!(matches!(&*s.fields[0].r#type, TypeNode::Link(_))),
        other => panic!("expected struct from inlined bar, got {other:?}"),
    }
}

#[test]
fn unselected_links_are_untouched() {
    let root = unwrap_type_defined_links(sample_root(), ["other"]);
    let fields = account_fields(&root);
    assert!(matches!(fields[0], TypeNode::Link(_)));
    assert!(matches!(fields[1], TypeNode::Link(_)));
}

#[test]
fn program_qualified_selector_matches() {
    let root = unwrap_type_defined_links(sample_root(), ["p.foo"]);
    let fields = account_fields(&root);
    assert!(matches!(fields[0], TypeNode::Number(_)));
}
