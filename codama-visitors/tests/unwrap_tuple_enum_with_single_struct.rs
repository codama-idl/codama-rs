use codama_nodes::{
    DefinedTypeLinkNode, DefinedTypeNode, EnumTupleVariantTypeNode, EnumTypeNode,
    EnumVariantTypeNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, TypeNode, U8,
};
use codama_visitors::unwrap_tuple_enum_with_single_struct;
use pretty_assertions::assert_eq;

/// `inner = struct{x:u8}`; `myEnum` has a tuple variant `v` of `(inner)`.
fn sample_root() -> RootNode {
    let inner = StructTypeNode::new(vec![StructFieldTypeNode::new("x", NumberTypeNode::le(U8))]);
    let variant = EnumVariantTypeNode::Tuple(EnumTupleVariantTypeNode::new(
        "v",
        TupleTypeNode::new(vec![DefinedTypeLinkNode {
            name: "inner".into(),
            program: None,
        }
        .into()]),
    ));
    let my_enum = EnumTypeNode::new(vec![variant]);

    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("inner", inner));
    program
        .defined_types
        .push(DefinedTypeNode::new("myEnum", my_enum));
    RootNode::new(program)
}

#[test]
fn converts_single_struct_tuple_variant_and_removes_orphaned_type() {
    let root = unwrap_tuple_enum_with_single_struct(sample_root());

    // `inner` was inlined into the variant and is no longer referenced -> removed.
    let names: Vec<_> = root
        .program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect();
    assert_eq!(names, vec!["myEnum".to_string()]);

    // The variant `v` is now a struct variant (not a tuple).
    match &*root.program.defined_types[0].r#type {
        TypeNode::Enum(e) => assert!(matches!(e.variants[0], EnumVariantTypeNode::Struct(_))),
        other => panic!("expected enum, got {other:?}"),
    }
}

#[test]
fn leaves_multi_item_tuple_variants_alone() {
    let variant = EnumVariantTypeNode::Tuple(EnumTupleVariantTypeNode::new(
        "pair",
        TupleTypeNode::new(vec![
            NumberTypeNode::le(U8).into(),
            NumberTypeNode::le(U8).into(),
        ]),
    ));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("e", EnumTypeNode::new(vec![variant])));

    let root = unwrap_tuple_enum_with_single_struct(RootNode::new(program));
    match &*root.program.defined_types[0].r#type {
        TypeNode::Enum(e) => assert!(matches!(e.variants[0], EnumVariantTypeNode::Tuple(_))),
        other => panic!("expected enum, got {other:?}"),
    }
}
