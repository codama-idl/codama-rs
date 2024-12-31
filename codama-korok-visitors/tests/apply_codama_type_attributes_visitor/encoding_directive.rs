use codama_korok_visitors::{
    ApplyCodamaTypeAttributesVisitor, KorokVisitable, SetBorshTypesVisitor,
};
use codama_koroks::FieldKorok;
use codama_nodes::{
    NumberFormat::U32, NumberTypeNode, SizePrefixTypeNode, StringTypeNode, StructFieldTypeNode,
};

#[test]
fn it_updates_the_encoding_of_string_type_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(encoding = base16)]
        String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(korok.node, Some(StringTypeNode::base16().into()));
    Ok(())
}

#[test]
fn it_updates_the_encoding_of_nested_string_type_nodes() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();
    korok.accept(&mut SetBorshTypesVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32)).into())
    );
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::base16(), NumberTypeNode::le(U32)).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_type_wrapped_in_a_struct_field_type_node() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(encoding = base16)]
        field: String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(StructFieldTypeNode::new("field", StringTypeNode::base16()).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_nested_type_wrapped_in_a_struct_field_type_node() -> syn::Result<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(encoding = base16)]
        field: String
    };
    let mut korok = FieldKorok::parse(&ast).unwrap();
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
    korok.accept(&mut ApplyCodamaTypeAttributesVisitor::new())?;
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
