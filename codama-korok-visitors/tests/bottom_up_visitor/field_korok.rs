use codama_korok_visitors::{BottomUpVisitor, KorokVisitable};
use codama_koroks::FieldKorok;
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, U64};
use codama_syn_helpers::SynBuilder;
use quote::quote;

#[test]
fn it_create_a_struct_field_type_node_when_nammed() {
    let ast = SynBuilder::named_field(quote! { foo: u64 });
    let mut korok = FieldKorok::parse(&ast).unwrap();
    korok.r#type.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut BottomUpVisitor::new());

    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("foo", NumberTypeNode::le(U64)).into())
    );
}

#[test]
fn it_forwards_the_type_when_unnamed() {
    let ast = SynBuilder::unnamed_field(quote! { u64 });
    let mut korok = FieldKorok::parse(&ast).unwrap();
    korok.r#type.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut BottomUpVisitor::new());

    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
}
