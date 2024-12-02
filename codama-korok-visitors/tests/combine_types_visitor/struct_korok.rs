use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::StructKorok;
use codama_nodes::{
    DefinedTypeNode, Node, NumberFormat::U64, NumberTypeNode, StringTypeNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, U32, U8,
};
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_creates_a_defined_type_struct_from_nammed_fields() {
    let ast: syn::ItemStruct = syn_build::parse(quote! {
        struct Person {
            age: u8,
            name: String,
        }
    });
    let mut korok = StructKorok::parse(&ast).unwrap();
    let struct_node = StructTypeNode::new(vec![
        StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("name", StringTypeNode::utf8()),
    ]);
    korok.fields.node = Some(struct_node.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("person", struct_node).into())
    );
}

#[test]
fn it_creates_a_defined_type_tuple_from_unnammed_fields() {
    let ast: syn::ItemStruct = syn_build::parse(quote! {
        struct Coordinates(u32, u32);
    });
    let mut korok = StructKorok::parse(&ast).unwrap();
    let tuple_node = TupleTypeNode::new(vec![
        NumberTypeNode::le(U32).into(),
        NumberTypeNode::le(U32).into(),
    ]);
    korok.fields.node = Some(tuple_node.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("coordinates", tuple_node).into())
    );
}

#[test]
fn it_creates_a_defined_type_from_single_unnammed_fields() {
    let ast: syn::ItemStruct = syn_build::parse(quote! {
        struct Slot(u64);
    });
    let mut korok = StructKorok::parse(&ast).unwrap();
    korok.fields.node = Some(TupleTypeNode::new(vec![NumberTypeNode::le(U64).into()]).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("slot", NumberTypeNode::le(U64)).into())
    );
}

#[test]
fn it_does_not_override_existing_nodes_by_default() {
    let ast: syn::ItemStruct = syn_build::parse(quote! {
        struct Overriden(u32, u32);
    });
    let mut korok = StructKorok::parse(&ast).unwrap();
    korok.fields.node = Some(
        TupleTypeNode::new(vec![
            NumberTypeNode::le(U32).into(),
            NumberTypeNode::le(U32).into(),
        ])
        .into(),
    );

    let original_node = Some(Node::from(DefinedTypeNode::new(
        "original",
        StringTypeNode::utf8(),
    )));
    korok.node = original_node.clone();
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(korok.node, original_node);
}
