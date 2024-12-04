use crate::define_borsh_types_visitor::utils::get_node_from_item;
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode,
    EnumTypeNode, Node, NumberTypeNode, SizePrefixTypeNode, StringTypeNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, I32, U32,
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
                EnumEmptyVariantTypeNode::new("quit").into(),
                EnumStructVariantTypeNode::new(
                    "move",
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                        StructFieldTypeNode::new("y", NumberTypeNode::le(I32))
                    ])
                )
                .into(),
                EnumTupleVariantTypeNode::new(
                    "write",
                    TupleTypeNode::new(vec![SizePrefixTypeNode::new(
                        StringTypeNode::utf8(),
                        NumberTypeNode::le(U32)
                    )
                    .into()])
                )
                .into(),
            ])
        )))
    );
}
