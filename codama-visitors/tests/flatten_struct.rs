use codama_nodes::{
    DefinedTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode,
    TypeNode, U8,
};
use codama_visitors::{flatten_struct, FlattenStruct, TransformVisitor};
use pretty_assertions::assert_eq;

fn field(name: &str) -> StructFieldTypeNode {
    StructFieldTypeNode::new(name, NumberTypeNode::le(U8))
}

/// `config` defined type holding `{ inner: { a, b }, c }`.
fn sample_root() -> RootNode {
    let inner = StructTypeNode::new(vec![field("a"), field("b")]);
    let outer = StructTypeNode::new(vec![StructFieldTypeNode::new("inner", inner), field("c")]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("config", outer));
    RootNode::new(program)
}

fn defined_type_fields(root: &RootNode) -> Vec<String> {
    match &*root.program.defined_types[0].r#type {
        TypeNode::Struct(s) => s.fields.iter().map(|f| f.name.to_string()).collect(),
        other => panic!("expected struct, got {other:?}"),
    }
}

#[test]
fn inlines_all_nested_structs() {
    let root =
        flatten_struct([("[definedTypeNode]config", FlattenStruct::All)]).visit_root(sample_root());
    assert_eq!(
        defined_type_fields(&root),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
}

#[test]
fn inlines_only_selected_fields() {
    // Only `inner` is requested, which is the one struct field anyway.
    let root = flatten_struct([(
        "[definedTypeNode]config",
        FlattenStruct::Fields(vec!["inner".to_string()]),
    )])
    .visit_root(sample_root());
    assert_eq!(
        defined_type_fields(&root),
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
}

#[test]
fn unselected_fields_are_left_nested() {
    let root = flatten_struct([(
        "[definedTypeNode]config",
        FlattenStruct::Fields(vec!["nope".to_string()]),
    )])
    .visit_root(sample_root());
    // `inner` was not selected, so it stays a nested struct field.
    assert_eq!(
        defined_type_fields(&root),
        vec!["inner".to_string(), "c".to_string()]
    );
}
