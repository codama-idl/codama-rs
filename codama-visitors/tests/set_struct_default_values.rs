use codama_nodes::{
    DefinedTypeNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode,
    StructTypeNode, TypeNode, ValueNode, U8,
};
use codama_visitors::{set_struct_default_values, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("bump", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("count", NumberTypeNode::le(U8)),
    ]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("config", data));
    RootNode::new(program)
}

fn field_default(root: &RootNode, index: usize) -> Option<ValueNode> {
    match &*root.program.defined_types[0].r#type {
        TypeNode::Struct(s) => (*s.fields[index].default_value).clone(),
        other => panic!("expected struct, got {other:?}"),
    }
}

#[test]
fn sets_default_values_for_named_fields() {
    let root = set_struct_default_values([(
        "[definedTypeNode]config",
        vec![("bump".to_string(), NumberValueNode::new(255u8).into())],
    )])
    .visit_root(sample_root());

    assert_eq!(
        field_default(&root, 0),
        Some(ValueNode::Number(NumberValueNode::new(255u8))),
    );
    // `count` was not in the map, so it keeps no default.
    assert_eq!(field_default(&root, 1), None);
}
