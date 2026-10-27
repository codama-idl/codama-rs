use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    BooleanTypeNode, DefinedTypeLinkNode, IntegerTypeNode, MapTypeNode, Node, PrefixedCountNode,
    U32, U64,
};
use quote::quote;

#[test]
fn it_identifies_map_types() {
    assert_eq!(
        get_node_from_type(quote! { HashMap<u64, bool> }),
        Some(Node::Type(
            MapTypeNode::new(
                IntegerTypeNode::le(U64),
                BooleanTypeNode::default(),
                PrefixedCountNode::new(IntegerTypeNode::le(U32))
            )
            .into()
        ))
    );
    assert!(get_node_from_type(quote! { std::collections::HashMap<u64, bool> }).is_some());
    assert!(get_node_from_type(quote! { BTreeMap<u64, bool> }).is_some());
    assert!(get_node_from_type(quote! { std::collections::BTreeMap<u64, bool> }).is_some());
    assert_eq!(
        get_node_from_type(quote! { some::wrong::HashMap<u64, bool> }),
        None
    );
    assert_eq!(get_node_from_type(quote! { HashMap<u8> }), None);
}

#[test]
fn it_identifies_maps_of_custom_types() {
    assert_eq!(
        get_node_from_type(quote! { HashMap<MyCustomType, u64> }),
        Some(Node::Type(
            MapTypeNode::new(
                DefinedTypeLinkNode::new("myCustomType"),
                IntegerTypeNode::le(U64),
                PrefixedCountNode::new(IntegerTypeNode::le(U32))
            )
            .into()
        ))
    );
}
