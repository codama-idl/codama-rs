use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{
    ArrayTypeNode, BooleanTypeNode, Node, NumberTypeNode, PrefixedCountNode, U32, U8,
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
    assert_eq!(get_node_from_type(quote! { Vec }), None);
    assert_eq!(get_node_from_type(quote! { Vec<'a> }), None);
}
