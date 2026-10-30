use crate::identify_field_types_visitor::utils::get_node_from_enum_variant;
use codama_nodes::{
    BooleanTypeNode, EnumVariantTypeNode, IntegerTypeNode, Node, RegisteredTypeNode, TupleTypeNode,
    U8,
};
use quote::quote;

#[test]
fn it_identifies_tuple_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! {
            Push(u8, bool)
        }),
        Some(Node::Type(
            EnumVariantTypeNode::with_data(
                "push",
                TupleTypeNode::new(vec![
                    IntegerTypeNode::le(U8).into(),
                    BooleanTypeNode::default().into()
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
            Push(u8, bool) = 42
        }),
        Some(Node::Type(RegisteredTypeNode::EnumVariant(
            EnumVariantTypeNode {
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
            Push(u8, bool) = 1 + 3 * 42
        }),
        Some(Node::Type(RegisteredTypeNode::EnumVariant(
            EnumVariantTypeNode {
                discriminator: None,
                ..
            }
        )))
    ));
}
