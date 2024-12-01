use codama_korok_visitors::{BottomUpVisitor, KorokVisitable};
use codama_koroks::FieldsKorok;
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, I32, U64};
use codama_syn_helpers::syn_build;
use quote::quote;

#[test]
fn it_create_a_struct_type_node_from_struct_field_type_nodes() {
    let ast = syn_build::fields(quote! { { x: i32, y: i32 } });
    let mut korok = FieldsKorok::parse(&ast).unwrap();
    korok.all[0].node = Some(StructFieldTypeNode::new("x", NumberTypeNode::le(I32)).into());
    korok.all[1].node = Some(StructFieldTypeNode::new("y", NumberTypeNode::le(I32)).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
            ])
            .into()
        )
    );
}

#[test]
fn it_create_a_tuple_type_node_from_multiple_type_nodes() {
    let ast = syn_build::fields(quote! { (i32, u64) });
    let mut korok = FieldsKorok::parse(&ast).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.all[1].node = Some(NumberTypeNode::le(U64).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(
            TupleTypeNode::new(vec![
                NumberTypeNode::le(I32).into(),
                NumberTypeNode::le(U64).into(),
            ])
            .into()
        )
    );
}

#[test]
fn it_create_a_tuple_type_node_from_single_type_nodes() {
    let ast = syn_build::fields(quote! { (i32) });
    let mut korok = FieldsKorok::parse(&ast).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
}

#[test]
fn it_sets_node_to_none_from_empty_fields() {
    let ast = syn_build::fields(quote! {});
    let mut korok = FieldsKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(korok.node, None);
}

#[test]
fn it_does_not_override_existing_nodes_by_default() {
    let ast = syn_build::fields(quote! { (i32) });
    let mut korok = FieldsKorok::parse(&ast).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut BottomUpVisitor::new());
    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
}

#[test]
fn it_can_override_existing_nodes() {
    let ast = syn_build::fields(quote! { (i32) });
    let mut korok = FieldsKorok::parse(&ast).unwrap();
    korok.all[0].r#type.node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut BottomUpVisitor { r#override: true });
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
}
