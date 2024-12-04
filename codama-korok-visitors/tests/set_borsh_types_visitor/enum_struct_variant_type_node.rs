use crate::set_borsh_types_visitor::utils::get_node_from_enum_variant;
use codama_nodes::{
    EnumStructVariantTypeNode, Node, NumberTypeNode, RegisteredTypeNode, StructFieldTypeNode,
    StructTypeNode, I32,
};
use quote::quote;

#[test]
fn it_identifies_struct_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! {
            Move { x: i32, y: i32 }
        }),
        Some(Node::Type(
            EnumStructVariantTypeNode::new(
                "move",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                    StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
                ])
            )
            .into()
        ))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert!(matches!(
        get_node_from_enum_variant(quote! {
            Move { x: i32, y: i32 } = 42
        }),
        Some(Node::Type(RegisteredTypeNode::EnumStructVariant(
            EnumStructVariantTypeNode {
                discriminator: Some(42),
                ..
            }
        )))
    ));
}

#[test]
fn it_does_not_identify_discriminators_from_complex_expressions() {
    assert!(matches!(
        get_node_from_enum_variant(quote! {
            Move { x: i32, y: i32 } = 1 + 3 * 42
        }),
        Some(Node::Type(RegisteredTypeNode::EnumStructVariant(
            EnumStructVariantTypeNode {
                discriminator: None,
                ..
            }
        )))
    ));
}
