use crate::set_borsh_types_visitor::utils::get_node_from_enum_variant;
use codama_nodes::{EnumEmptyVariantTypeNode, Node};
use quote::quote;

#[test]
fn it_identifies_empty_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana }),
        Some(Node::Type(EnumEmptyVariantTypeNode::new("banana").into()))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana = 42 }),
        Some(Node::Type(
            EnumEmptyVariantTypeNode {
                name: "banana".into(),
                discriminator: Some(42),
            }
            .into()
        ))
    );
}

#[test]
fn it_does_not_identify_discriminators_from_complex_expressions() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana = 1 + 3 * 42 }),
        Some(Node::Type(EnumEmptyVariantTypeNode::new("banana").into()))
    );
}
