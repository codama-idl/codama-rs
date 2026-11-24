use crate::identify_field_types_visitor::utils::{
    get_error_from_enum_variant, get_node_from_enum_variant,
};
use codama_nodes::{
    BooleanTypeNode, EnumVariantTypeNode, IntegerTypeNode, Node, TupleTypeNode, U8,
};
use quote::quote;

#[test]
fn it_identifies_tuple_variants() {
    assert_eq!(
        get_node_from_enum_variant(quote! {
            Push(u8, bool)
        }),
        Some(Node::from(EnumVariantTypeNode::with_data(
            "push",
            TupleTypeNode::new(vec![
                IntegerTypeNode::le(U8).into(),
                BooleanTypeNode::default().into()
            ])
        )))
    );
}

#[test]
fn it_identifies_literal_discriminators() {
    assert!(matches!(
        get_node_from_enum_variant(quote! {
            Push(u8, bool) = 42
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
    let error = get_error_from_enum_variant(quote! { Push(u8, bool) = 1 + 3 * 42 });
    assert!(
        error.contains("must be an unsigned integer literal"),
        "{error}"
    );
}
