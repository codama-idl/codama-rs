use codama_nodes::{
    ArrayTypeNode, BytesTypeNode, CountNode, FixedSizeTypeNode, NumberTypeNode, PrefixedCountNode,
    RemainderCountNode, SizePrefixTypeNode, StructFieldTypeNode, StructTypeNode, TypeNode, U32, U8,
};
use codama_visitors::{TransformU8ArraysToBytes, TransformVisitor};
use pretty_assertions::assert_eq;

fn transform(node: TypeNode) -> TypeNode {
    TransformU8ArraysToBytes.visit_type_node(node)
}

#[test]
fn fixed_u8_array_becomes_fixed_size_bytes() {
    let input: TypeNode = ArrayTypeNode::fixed(NumberTypeNode::le(U8), 32).into();
    assert_eq!(
        transform(input),
        FixedSizeTypeNode::<TypeNode>::new(BytesTypeNode::new(), 32).into(),
    );
}

#[test]
fn prefixed_u8_array_becomes_size_prefixed_bytes() {
    let input: TypeNode = ArrayTypeNode::new(
        NumberTypeNode::le(U8),
        PrefixedCountNode::new(NumberTypeNode::le(U32)),
    )
    .into();
    assert_eq!(
        transform(input),
        SizePrefixTypeNode::<TypeNode>::new(BytesTypeNode::new(), NumberTypeNode::le(U32)).into(),
    );
}

#[test]
fn remainder_u8_array_becomes_bytes() {
    let input: TypeNode = ArrayTypeNode::new(
        NumberTypeNode::le(U8),
        CountNode::Remainder(RemainderCountNode {}),
    )
    .into();
    assert_eq!(transform(input), BytesTypeNode::new().into());
}

#[test]
fn non_u8_arrays_are_left_unchanged() {
    let input: TypeNode = ArrayTypeNode::fixed(NumberTypeNode::le(U32), 4).into();
    assert_eq!(transform(input.clone()), input);
}

#[test]
fn nested_u8_arrays_are_converted_bottom_up() {
    // A struct field holding a `[u8; 8]` is rewritten in place.
    let input: TypeNode = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "blob",
        ArrayTypeNode::fixed(NumberTypeNode::le(U8), 8),
    )])
    .into();

    let expected: TypeNode = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "blob",
        FixedSizeTypeNode::<TypeNode>::new(BytesTypeNode::new(), 8),
    )])
    .into();

    assert_eq!(transform(input), expected);
}
