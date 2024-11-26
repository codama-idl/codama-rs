use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{Node, NumberTypeNode, SizePrefixTypeNode, StringTypeNode, U32};
use quote::quote;

#[test]
fn it_identifies_string_types() {
    let expected: Option<Node> =
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into());
    assert_eq!(get_node_from_type(quote! { String }), expected);
    assert_eq!(get_node_from_type(quote! { std::string::String }), expected);
    assert_eq!(get_node_from_type(quote! { some::wrong::String }), None);
    assert_eq!(get_node_from_type(quote! { String<T> }), None);
}
