use crate::identify_field_types_visitor::utils::get_node_from_type;
use codama_nodes::PublicKeyTypeNode;
use quote::quote;

#[test]
fn it_identifies_pubkey_types() {
    assert_eq!(
        get_node_from_type(quote! { Pubkey }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        get_node_from_type(quote! { solana_sdk::pubkey::Pubkey }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        get_node_from_type(quote! { solana_pubkey::Pubkey }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        get_node_from_type(quote! { solana_program::Pubkey }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(get_node_from_type(quote! { Pubkey<T> }), None);
    assert_eq!(
        get_node_from_type(quote! { Address }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        get_node_from_type(quote! { solana_address::Address }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(
        get_node_from_type(quote! { solana_address::address::Address }),
        Some(PublicKeyTypeNode::new().into())
    );
    assert_eq!(get_node_from_type(quote! { Address<T> }), None);
}
