use crate::borsh_visitor::utils::get_node_from_type;
use codama_nodes::{Node, NumberTypeNode, PrefixedCountNode, SetTypeNode, U32, U64};
use quote::quote;

#[test]
fn it_identifies_set_types() {
    assert_eq!(
        get_node_from_type(quote! { HashSet<u64> }),
        Some(Node::Type(
            SetTypeNode::new(
                NumberTypeNode::le(U64),
                PrefixedCountNode::new(NumberTypeNode::le(U32))
            )
            .into()
        ))
    );
    assert!(matches!(
        get_node_from_type(quote! { std::collections::HashSet<u64> }),
        Some(_)
    ));
    assert!(matches!(
        get_node_from_type(quote! { BTreeSet<u64> }),
        Some(_)
    ));
    assert!(matches!(
        get_node_from_type(quote! { std::collections::BTreeSet<u64> }),
        Some(_)
    ));
    assert_eq!(
        get_node_from_type(quote! { some::wrong::HashSet<u64> }),
        None
    );
    assert_eq!(get_node_from_type(quote! { HashSet }), None);
    assert_eq!(get_node_from_type(quote! { HashSet<'a> }), None);
    assert_eq!(get_node_from_type(quote! { HashSet<u8, u64> }), None);
}
