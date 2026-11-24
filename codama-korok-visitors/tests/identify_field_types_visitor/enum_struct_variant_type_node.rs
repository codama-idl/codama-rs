use crate::identify_field_types_visitor::utils::{
    get_error_from_enum_variant, get_node_from_enum_variant,
};
use codama_nodes::{
    EnumVariantTypeNode, IntegerTypeNode, Node, StructFieldTypeNode, StructTypeNode, I32,
};
use quote::quote;

#[test]
fn it_identifies_struct_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! {
            Move { x: i32, y: i32 }
        }),
        Some(Node::from(EnumVariantTypeNode::with_data(
            "move",
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("x", IntegerTypeNode::le(I32)),
                StructFieldTypeNode::new("y", IntegerTypeNode::le(I32)),
            ])
        )))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert!(matches!(
        get_node_from_enum_variant(quote! {
            Move { x: i32, y: i32 } = 42
        }),
        Some(Node::EnumVariantType(EnumVariantTypeNode {
            discriminator: Some(42),
            ..
        }))
    ));
}

#[test]
fn it_rejects_discriminants_from_complex_expressions() {
    // Codama cannot evaluate the expression, and an omitted discriminator would mean the index.
    let error = get_error_from_enum_variant(quote! { Move { x: i32, y: i32 } = 1 + 3 * 42 });
    assert!(
        error.contains("must be an unsigned integer literal"),
        "{error}"
    );
}
