use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::FieldsKorok;
use codama_nodes::{
    BooleanTypeNode, NumberTypeNode, StringValueNode, StructFieldTypeNode, StructTypeNode,
    TupleTypeNode, I32, U64,
};

#[test]
fn it_create_a_struct_type_node_from_struct_field_type_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo { x: i32, y: i32 } };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(StructFieldTypeNode::new("x", NumberTypeNode::le(I32)).into());
    korok.all[1].node = Some(StructFieldTypeNode::new("y", NumberTypeNode::le(I32)).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
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
    Ok(())
}

#[test]
fn it_create_a_tuple_type_node_from_multiple_type_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32, u64); };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.all[1].node = Some(NumberTypeNode::le(U64).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
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
    Ok(())
}

#[test]
fn it_create_a_tuple_type_node_from_single_type_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
    Ok(())
}

#[test]
fn it_creates_an_empty_struct_from_unit_fields() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo; };
    let mut korok = FieldsKorok::parse(&ast.fields)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(korok.node, Some(StructTypeNode::new(vec![]).into()));
    Ok(())
}

#[test]
fn it_does_not_override_existing_nodes_by_default() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(korok.node, Some(NumberTypeNode::le(U64).into()));
    Ok(())
}

#[test]
fn it_can_override_existing_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32); };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.node = Some(NumberTypeNode::le(U64).into());

    korok.accept(&mut CombineTypesVisitor { r#override: true })?;
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
    Ok(())
}

#[test]
fn it_ignores_nammed_fields_with_invalid_or_missing_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct =
        syn::parse_quote! { struct Foo { valid: i32, invalid: i32, missing: i32 } };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(StructFieldTypeNode::new("valid", NumberTypeNode::le(I32)).into());
    korok.all[1].node = Some(BooleanTypeNode::default().into());
    korok.all[2].node = None;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "valid",
                NumberTypeNode::le(I32)
            )])
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_ignores_unnammed_fields_with_invalid_or_missing_nodes() -> CodamaResult<()> {
    let ast: syn::ItemStruct = syn::parse_quote! { struct Foo (i32, i32, i32); };
    let mut korok = FieldsKorok::parse(&ast.fields)?;
    korok.all[0].node = Some(NumberTypeNode::le(I32).into());
    korok.all[1].node = Some(StringValueNode::new("Invalid type").into());
    korok.all[2].node = None;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(TupleTypeNode::new(vec![NumberTypeNode::le(I32).into()]).into())
    );
    Ok(())
}
