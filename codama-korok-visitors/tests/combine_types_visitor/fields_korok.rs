use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::FieldsKorok;
use codama_nodes::{NumberTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, I32, U64};

#[test]
fn it_create_a_struct_type_node_from_struct_field_type_nodes() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo { x: i32, y: i32 } };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();
    korok.all[0].node = Some(StructFieldTypeNode::new("x", NumberTypeNode::le(I32)).into());
    korok.all[1].node = Some(StructFieldTypeNode::new("y", NumberTypeNode::le(I32)).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
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
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32, u64); };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.all[1].node = Some(NumberTypeNode::le(U64).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
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
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
}

#[test]
fn it_sets_node_to_none_from_empty_fields() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo; };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(korok.node, None);
}

#[test]
fn it_does_not_override_existing_nodes_by_default() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut CombineTypesVisitor::new());
    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
}

#[test]
fn it_can_override_existing_nodes() {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields).unwrap();
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut CombineTypesVisitor { r#override: true });
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
}
