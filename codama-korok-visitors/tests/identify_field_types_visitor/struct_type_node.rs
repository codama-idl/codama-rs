use crate::identify_field_types_visitor::utils::get_node_from_item;
use codama_nodes::test_utils::id;
use codama_nodes::{
    BooleanTypeNode, DefinedTypeNode, HasTransforms, IntegerTypeNode, Node,
    SizePrefixTransformNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_named_fields_in_a_defined_struct() {
    assert_eq!(
        get_node_from_item(quote! {
            pub struct Person<'a> {
                pub name: String,
                pub age: u8,
                pub member: bool,
                pub is_alive: &'a u8
            }
        }),
        Some(Node::DefinedType(DefinedTypeNode::new(
            id("person"),
            StructTypeNode::new(vec![
                StructFieldTypeNode::new(
                    id("name"),
                    StringTypeNode::utf8()
                        .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                ),
                StructFieldTypeNode::new(id("age"), IntegerTypeNode::le(U8)),
                StructFieldTypeNode::new(id("member"), BooleanTypeNode::default()),
                StructFieldTypeNode::new(id("is_alive"), IntegerTypeNode::le(U8))
            ])
        )))
    );
}
