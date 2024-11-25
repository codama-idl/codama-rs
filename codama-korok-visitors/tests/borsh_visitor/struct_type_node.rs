use codama_korok_visitors::{BorshVisitor, KorokVisitable};
use codama_koroks::{stores::RootStore, RootKorok};
use codama_nodes::{
    BooleanTypeNode, Node, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode, StringTypeNode,
    StructFieldTypeNode, StructTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_named_fields_in_a_struct() {
    let store = RootStore::populate_from(quote! {
        pub struct Person {
            pub name: String,
            pub age: u8,
            pub member: bool,
        }
    })
    .unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());

    assert_eq!(
        korok.first_item_as_struct().node,
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
