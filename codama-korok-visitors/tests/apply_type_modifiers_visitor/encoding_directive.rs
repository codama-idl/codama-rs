use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeModifiersVisitor, ApplyTypeOverridesVisitor, KorokVisitable, SetBorshTypesVisitor,
};
use codama_koroks::{FieldKorok, KorokTrait};
use codama_nodes::{
    BooleanTypeNode, FixedSizeTypeNode, NumberFormat::U32, NumberTypeNode, SizePrefixTypeNode,
    StringTypeNode, StringValueNode, StructFieldTypeNode,
};

#[test]
fn it_updates_the_encoding_of_string_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(encoding = base16)]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(korok.node, Some(StringTypeNode::base16().into()));
    Ok(())
}

#[test]
fn it_updates_the_encoding_of_nested_string_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.accept(&mut SetBorshTypesVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into())
    );
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::base16(), NumberTypeNode::le(U32)).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_type_wrapped_in_a_struct_field_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(encoding = base16)]
        field: String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("field", StringTypeNode::base16()).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_nested_type_wrapped_in_a_struct_field_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        field: String
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.accept(&mut SetBorshTypesVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "field",
                SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32))
            )
            .into()
        )
    );
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "field",
                SizePrefixTypeNode::new(StringTypeNode::base16(), NumberTypeNode::le(U32))
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_fails_on_empty_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        Untyped
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut ApplyTypeModifiersVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot apply attribute `#[codama(encoding)]` on an empty node"
    );
    Ok(())
}

#[test]
fn it_fails_on_non_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        ValueNode
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.set_node(Some(StringValueNode::new("Some string value").into()));

    let error = korok
        .accept(&mut ApplyTypeModifiersVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot apply attribute `#[codama(encoding)]` on a node of kind `stringValueNode`"
    );
    Ok(())
}

#[test]
fn it_fails_on_nested_type_nodes_that_are_not_string_types() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        NestedValueNode
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.set_node(Some(
        FixedSizeTypeNode::new(BooleanTypeNode::default(), 42).into(),
    ));

    let error = korok
        .accept(&mut ApplyTypeModifiersVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cannot apply attribute `#[codama(encoding)]` on a node of kind `NestedTypeNode<booleanTypeNode>`"
    );
    Ok(())
}
