use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::{
    DefinedTypeLinkNode, IntegerTypeNode, Node, PrefixedCountNode, SetTypeNode, U32, U64,
};
use quote::quote;

#[test]
fn it_identifies_set_types() {
    assert_eq!(
        get_node_from_type(quote! { HashSet<u64> }),
        Some(Node::Type(
            SetTypeNode::new(
                IntegerTypeNode::le(U64),
                PrefixedCountNode::new(IntegerTypeNode::le(U32))
            )
            .into()
        ))
    );
    assert!(get_node_from_type(quote! { std::collections::HashSet<u64> }).is_some());
    assert!(get_node_from_type(quote! { BTreeSet<u64> }).is_some());
    assert!(get_node_from_type(quote! { std::collections::BTreeSet<u64> }).is_some());
    assert_eq!(
        get_node_from_type(quote! { some::wrong::HashSet<u64> }),
        None
    );
    assert_eq!(get_node_from_type(quote! { HashSet<u8, u64> }), None);
}

#[test]
fn it_identifies_sets_of_custom_types() {
    assert_eq!(
        get_node_from_type(quote! { HashSet<MyCustomType> }),
        Some(Node::Type(
            SetTypeNode::new(
                DefinedTypeLinkNode::new("myCustomType"),
                PrefixedCountNode::new(IntegerTypeNode::le(U32))
            )
            .into()
        ))
    );
}
