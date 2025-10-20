use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    ArrayTypeNode, BooleanTypeNode, DefinedTypeLinkNode, FixedCountNode, Node, NumberTypeNode,
    PrefixedCountNode, U32, U8,
};
use quote::quote;

#[test]
fn it_identifies_vec_types() {
    let u32_prefix = PrefixedCountNode::new(NumberTypeNode::le(U32));
    assert_eq!(
        get_node_from_type(quote! { Vec<u8> }),
        Some(Node::Type(
            ArrayTypeNode::new(NumberTypeNode::le(U8), u32_prefix.clone()).into()
        ))
    );
    assert_eq!(
        get_node_from_type(quote! { std::vec::Vec<bool> }),
        Some(Node::Type(
            ArrayTypeNode::new(BooleanTypeNode::default(), u32_prefix.clone()).into()
        ))
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::Vec<bool> }), None);
}

#[test]
fn it_identifies_vecs_of_custom_types() {
    assert_eq!(
        get_node_from_type(quote! { Vec<MyCustomType> }),
        Some(Node::Type(
            ArrayTypeNode::new(
                DefinedTypeLinkNode::new("myCustomType"),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        ))
    );
}

#[test]
fn it_identifies_fixed_array_types() {
    assert_eq!(
        get_node_from_type(quote! { [u8; 5] }),
        Some(Node::Type(
            ArrayTypeNode::new(NumberTypeNode::le(U8), FixedCountNode::new(5)).into()
        ))
    );
    assert_eq!(
        get_node_from_type(quote! { [bool; 42] }),
        Some(Node::Type(
            ArrayTypeNode::new(BooleanTypeNode::default(), FixedCountNode::new(42)).into()
        ))
    );
    assert_eq!(get_node_from_type(quote! { [bool; 1 + 2 * 4] }), None);
}

#[test]
fn it_identifies_fixed_arrays_of_custom_types() {
    assert_eq!(
        get_node_from_type(quote! { [MyCustomType; 5] }),
        Some(Node::Type(
            ArrayTypeNode::new(
                DefinedTypeLinkNode::new("myCustomType"),
                FixedCountNode::new(5)
            )
            .into()
        ))
    );
}
