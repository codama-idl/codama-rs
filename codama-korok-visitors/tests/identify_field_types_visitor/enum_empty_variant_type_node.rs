use crate::identify_field_types_visitor::utils::{
    get_error_from_enum_variant, get_node_from_enum_variant,
};
use codama_nodes::{EnumVariantTypeNode, Node};
use quote::quote;

#[test]
fn it_identifies_empty_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana }),
        Some(Node::from(EnumVariantTypeNode::new("banana")))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert_eq!(
        get_node_from_enum_variant(quote! { Banana = 42 }),
        Some(Node::from(EnumVariantTypeNode {
            discriminator: Some(42),
            ..EnumVariantTypeNode::new("banana")
        }))
    );
}

#[test]
fn it_rejects_discriminants_from_complex_expressions() {
    // Codama cannot evaluate the expression, and an omitted discriminator would mean the index.
    let error = get_error_from_enum_variant(quote! { Banana = 1 + 3 * 42 });
    assert!(
        error.contains("must be an unsigned integer literal"),
        "{error}"
    );
}
