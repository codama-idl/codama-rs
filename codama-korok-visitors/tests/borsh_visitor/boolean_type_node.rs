use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{BooleanTypeNode, Node};
use quote::quote;

#[test]
fn it_identifies_boolean_types() {
    let expected: Option<Node> = Some(BooleanTypeNode::default().into());
    assert_eq!(get_node_from_type(quote! { bool }), expected);
    assert_eq!(
        get_node_from_type(quote! { std::primitive::bool }),
        expected
    );
    assert_eq!(get_node_from_type(quote! { some::wrong::bool }), None);
    assert_eq!(get_node_from_type(quote! { bool<T> }), None);
}
