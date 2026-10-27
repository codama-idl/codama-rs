use crate::identify_field_types_visitor::utils::get_node_from_enum_variant;
use codama_nodes::{EnumVariantTypeNode, Node};
use quote::quote;

#[test]
fn it_identifies_empty_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana }),
        Some(Node::Type(EnumVariantTypeNode::new("banana").into()))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana = 42 }),
        Some(Node::Type(
            EnumVariantTypeNode {
                discriminator: Some(42),
                ..EnumVariantTypeNode::new("banana")
            }
            .into()
        ))
    );
}

#[test]
fn it_does_not_identify_discriminators_from_complex_expressions() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana = 1 + 3 * 42 }),
        Some(Node::Type(EnumVariantTypeNode::new("banana").into()))
    );
}
