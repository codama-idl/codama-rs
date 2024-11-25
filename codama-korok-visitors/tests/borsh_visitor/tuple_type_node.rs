use codama_korok_visitors::{BorshVisitor, KorokVisitable};
use codama_koroks::{stores::RootStore, RootKorok};
use codama_nodes::{
    BooleanTypeNode, Node, NumberTypeNode, RegisteredTypeNode, SizePrefixTypeNode, StringTypeNode,
    TupleTypeNode, U32, U8,
};
use quote::quote;

#[test]
fn it_wraps_all_unnamed_fields_in_a_tuple() {
    let store = RootStore::populate_from(quote! {
        pub struct Person (String, u8, bool);
    })
    .unwrap();
    let mut korok = RootKorok::parse(&store).unwrap();
    korok.accept(&mut BorshVisitor::new());

    assert_eq!(
        korok.first_item_as_struct().node,
        Some(Node::Type(RegisteredTypeNode::Tuple(TupleTypeNode::new(
            vec![
                SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into(),
                NumberTypeNode::le(U8).into(),
                BooleanTypeNode::default().into(),
            ]
        ))))
    );
}
