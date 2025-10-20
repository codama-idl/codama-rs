use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeModifiersVisitor, ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable,
};
use codama_koroks::FieldKorok;
use codama_nodes::{
    FixedSizeTypeNode,
    NumberFormat::{U16, U32, U8},
    NumberTypeNode, SizePrefixTypeNode, StringTypeNode, StructFieldTypeNode,
};

#[test]
fn it_wraps_any_type_into_a_size_prefix_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = number(u8))]
        u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(NumberTypeNode::le(U32), NumberTypeNode::le(U8)).into())
    );
    Ok(())
}

#[test]
fn it_accepts_nested_number_type_nodes_as_size_prefixes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = fixed_size(number(u8), 42))]
        u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            SizePrefixTypeNode::new(
                NumberTypeNode::le(U32),
                FixedSizeTypeNode::new(NumberTypeNode::le(U8), 42)
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_wraps_any_overridden_type_into_a_size_prefix_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(size_prefix = number(u8))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U8)).into())
    );
    Ok(())
}

#[test]
fn it_replaces_the_size_of_existing_size_prefix_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(size_prefix = number(u8))]
        #[codama(size_prefix = number(u16))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U16)).into())
    );
    Ok(())
}

#[test]
fn it_replaces_fixed_size_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(fixed_size = 42)]
        #[codama(size_prefix = number(u8))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U8)).into())
    );
    Ok(())
}

#[test]
fn it_keeps_the_type_wrapped_in_a_struct_field_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = number(u8))]
        field: u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "field",
                SizePrefixTypeNode::new(NumberTypeNode::le(U32), NumberTypeNode::le(U8))
            )
            .into()
        )
    );
    Ok(())
}
