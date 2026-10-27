use crate::identify_field_types_visitor::utils::get_node_from_item;
use codama_nodes::{
    DefinedTypeNode, EnumTypeNode, EnumVariantTypeNode, HasTransforms, IntegerTypeNode, Node,
    SizePrefixTransformNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode,
    I32, U32,
};
use quote::quote;

#[test]
fn it_wraps_all_variants_in_a_defined_enum() {
    assert_eq!(
        get_node_from_item(quote! {
            pub enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
            }
        }),
        Some(Node::DefinedType(DefinedTypeNode::new(
            "message",
            EnumTypeNode::new(vec![
                EnumVariantTypeNode::new("quit"),
                EnumVariantTypeNode::with_data(
                    "move",
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("x", IntegerTypeNode::le(I32)),
                        StructFieldTypeNode::new("y", IntegerTypeNode::le(I32))
                    ])
                ),
                EnumVariantTypeNode::with_data(
                    "write",
                    TupleTypeNode::new(vec![StringTypeNode::utf8()
                        .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                        .into()])
                ),
            ])
        )))
    );
}
