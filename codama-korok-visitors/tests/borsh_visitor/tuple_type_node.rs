use crate::borsh_visitor::utils::get_node_from_struct;
use codama_nodes::{
    BooleanTypeNode, Node, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode, StringTypeNode,
    TupleTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_unnamed_fields_in_a_tuple() {
    assert_eq!(
        get_node_from_struct(quote! {
            pub struct Person(String, u8, bool);
        }),
        Some(Node::Type(RegisteredTypeNode::Tuple(TupleTypeNode::new(
            vec![
                SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into(),
                NumberTypeNode::le(U8).into(),
                BooleanTypeNode::default().into(),
            ]
        ))))
    );
}
