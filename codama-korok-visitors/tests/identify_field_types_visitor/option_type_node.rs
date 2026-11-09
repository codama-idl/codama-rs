use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    BooleanTypeNode, DefinedTypeLinkNode, IntegerFormat::U64, IntegerTypeNode, Node, OptionTypeNode,
};
use quote::quote;

#[test]
fn it_identifies_option_types() {
    assert_eq!(
        get_node_from_type(quote! { Option<u64> }),
        Some(Node::from(OptionTypeNode::new(IntegerTypeNode::le(U64))))
    );
    assert_eq!(
        get_node_from_type(quote! { std::option::Option<bool> }),
        Some(Node::from(OptionTypeNode::new(BooleanTypeNode::default())))
    );
    assert_eq!(
        get_node_from_type(quote! { some::wrong::Option<bool> }),
        None
    );
}

#[test]
fn it_fallsback_to_defined_type_link_nodes() {
    assert_eq!(
        get_node_from_type(quote! { some::wrong::Option }),
        Some(DefinedTypeLinkNode::new("option").into())
    );
    assert_eq!(
        get_node_from_type(quote! { Option }),
        Some(DefinedTypeLinkNode::new("option").into())
    );
}

#[test]
fn it_identifies_option_of_custom_types() {
    assert_eq!(
        get_node_from_type(quote! { Option<MyCustomType> }),
        Some(Node::from(OptionTypeNode::new(DefinedTypeLinkNode::new(
            "my_custom_type"
        ))))
    );
}
