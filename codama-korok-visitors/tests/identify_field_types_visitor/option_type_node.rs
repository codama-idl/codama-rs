use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    BooleanTypeNode, DefinedTypeLinkNode, Node, NumberFormat::U64, NumberTypeNode, OptionTypeNode,
};
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
        Some(Node::Type(
            OptionTypeNode::new(DefinedTypeLinkNode::new("myCustomType")).into()
        ))
    );
}
