use codama_korok_visitors::{
    ApplyCodamaTypeAttributesVisitor, KorokVisitable, SetBorshTypesVisitor,
};
use codama_koroks::FieldKorok;
use codama_nodes::{
    FixedSizeTypeNode, NumberFormat::U32, NumberTypeNode, StringTypeNode, StructFieldTypeNode,
};

#[test]
fn it_wraps_any_type_into_a_fixed_size_type_node() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(fixed_size = 8)]
        u32
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(FixedSizeTypeNode::new(NumberTypeNode::le(U32), 8).into())
    );
    Ok(())
}

#[test]
fn it_wraps_any_overridden_type_into_a_fixed_size_type_node() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(fixed_size = 42)]
        String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(FixedSizeTypeNode::new(StringTypeNode::utf8(), 42).into())
    );
    Ok(())
}

#[test]
fn it_replaces_the_size_of_existing_fixed_size_type_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(fixed_size = 111)]
        #[codama(fixed_size = 222)]
        String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(FixedSizeTypeNode::new(StringTypeNode::utf8(), 222).into())
    );
    Ok(())
}

#[test]
fn it_replaces_size_prefixed_type_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(fixed_size = 42)]
        String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(FixedSizeTypeNode::new(StringTypeNode::utf8(), 42).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_type_wrapped_in_a_struct_field_type_node() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(fixed_size = 8)]
        field: u32
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new("field", FixedSizeTypeNode::new(NumberTypeNode::le(U32), 8))
                .into()
        )
    );
    Ok(())
}
