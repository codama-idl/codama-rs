use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{BooleanTypeNode, Node, RegisteredTypeNode};
use quote::quote;

#[test]
fn it_identifies_boolean_types() {
    let expected = Some(Node::Type(RegisteredTypeNode::Boolean(
        BooleanTypeNode::default(),
    )));
    assert_eq!(get_node_from_type(quote! { bool }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::bool }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::bool }), None);
    assert_eq!(get_node_from_type(quote! { bool<T> }), None);
}
