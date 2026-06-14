use codama_nodes::{
    DefinedTypeLinkNode, DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, TypeNode, U8,
};
use codama_visitors::{update_defined_types, DefinedTypeUpdate, TransformVisitor};
use pretty_assertions::assert_eq;

/// `oldType` (a number) plus `user` (a struct with a field linking to `oldType`).
fn sample_root() -> RootNode {
    let user = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "ref",
        DefinedTypeLinkNode {
            name: "oldType".into(),
            program: None,
        },
    )]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("oldType", NumberTypeNode::le(U8)));
    program
        .defined_types
        .push(DefinedTypeNode::new("user", user));
    RootNode::new(program)
}

fn ref_link_name(root: &RootNode) -> String {
    match &*root.program.defined_types[1].r#type {
        TypeNode::Struct(s) => match &*s.fields[0].r#type {
            TypeNode::Link(link) => link.name.to_string(),
            other => panic!("expected link, got {other:?}"),
        },
        other => panic!("expected struct, got {other:?}"),
    }
}

#[test]
fn renames_a_defined_type_and_rewrites_its_links() {
    let root = update_defined_types([("oldType", DefinedTypeUpdate::new().name("newType"))])
        .visit_root(sample_root());

    assert_eq!(root.program.defined_types[0].name.as_ref(), "newType");
    // The link inside `user.ref` now points at the new name.
    assert_eq!(ref_link_name(&root), "newType");
}

#[test]
fn non_rename_update_leaves_links_alone() {
    let root = update_defined_types([(
        "oldType",
        DefinedTypeUpdate::new().docs(vec!["a number".to_string()]),
    )])
    .visit_root(sample_root());

    assert_eq!(root.program.defined_types[0].name.as_ref(), "oldType");
    assert_eq!(ref_link_name(&root), "oldType");
}

#[test]
fn renames_inner_struct_fields() {
    let root =
        update_defined_types([("user", DefinedTypeUpdate::new().rename("ref", "reference"))])
            .visit_root(sample_root());

    let TypeNode::Struct(s) = &*root.program.defined_types[1].r#type else {
        panic!("expected struct")
    };
    assert_eq!(s.fields[0].name.as_ref(), "reference");
}

#[test]
fn deletes_a_defined_type() {
    let root = update_defined_types([("oldType", DefinedTypeUpdate::new().delete())])
        .visit_root(sample_root());

    let names: Vec<_> = root
        .program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect();
    assert_eq!(names, vec!["user".to_string()]);
}
