use crate::set_borsh_types_visitor::utils::get_node_from_type;
use codama_nodes::{BooleanTypeNode, Node, NumberFormat::U64, NumberTypeNode, OptionTypeNode};
use quote::quote;

#[test]
fn it_identifies_option_types() {
    assert_eq!(
        get_node_from_type(quote! { Option<u64> }),
        Some(Node::Type(
            OptionTypeNode::new(NumberTypeNode::le(U64)).into()
        ))
    );
    assert_eq!(
        get_node_from_type(quote! { std::option::Option<bool> }),
        Some(Node::Type(
            OptionTypeNode::new(BooleanTypeNode::default()).into()
        ))
    );
    assert_eq!(
        get_node_from_type(quote! { some::wrong::Option<bool> }),
        None
    );
    assert_eq!(get_node_from_type(quote! { Option }), None);
    assert_eq!(get_node_from_type(quote! { Option<'a> }), None);
}
