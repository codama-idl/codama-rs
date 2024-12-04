use crate::define_borsh_types_visitor::utils::get_node_from_item;
use codama_nodes::{
    BooleanTypeNode, DefinedTypeNode, Node, NumberFormat::U64, NumberTypeNode, SizePrefixTypeNode,
    StringTypeNode, TupleTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_unnamed_fields_in_a_defined_tuple() {
    assert_eq!(
        get_node_from_item(quote! {
            pub struct Person(String, u8, bool);
        }),
        Some(Node::DefinedType(DefinedTypeNode::new(
            "person",
            TupleTypeNode::new(vec![
                SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into(),
                NumberTypeNode::le(U8).into(),
                BooleanTypeNode::default().into(),
            ])
        )))
    );
}

#[test]
fn it_uses_the_inner_type_directly_on_single_unnamed_fields() {
    assert_eq!(
        get_node_from_item(quote! { struct Slot(u64); }),
        Some(DefinedTypeNode::new("slot", NumberTypeNode::le(U64)).into())
    );
}
