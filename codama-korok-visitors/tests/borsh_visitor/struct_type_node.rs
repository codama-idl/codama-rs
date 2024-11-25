use crate::borsh_visitor::utils::get_node_from_struct;
use codama_nodes::{
    BooleanTypeNode, Node, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode, StringTypeNode,
    StructFieldTypeNode, StructTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_named_fields_in_a_struct() {
    assert_eq!(
        get_node_from_struct(quote! {
            pub struct Person {
                pub name: String,
                pub age: u8,
                pub member: bool,
            }
        }),
        Some(Node::Type(RegisteredTypeNode::Struct(StructTypeNode::new(
            vec![
                StructFieldTypeNode::new(
                    "name",
                    SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32))
                ),
                StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                StructFieldTypeNode::new("member", BooleanTypeNode::default()),
            ]
        ))))
    );
}
