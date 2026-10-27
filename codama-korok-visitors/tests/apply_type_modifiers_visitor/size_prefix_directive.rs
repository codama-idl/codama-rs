use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeModifiersVisitor, ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable,
};
use codama_koroks::FieldKorok;
use codama_nodes::{
    ConstantValueNode, FixedSizeTransformNode, HasTransforms, HiddenPrefixTransformNode,
    IntegerFormat::{U16, U32, U8},
    IntegerTypeNode, IntegerValueNode, SizePrefixTransformNode, StringTypeNode,
    StructFieldTypeNode,
};

#[test]
fn it_adds_a_size_prefix_transform_to_any_type() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = integer(u8))]
        u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            IntegerTypeNode::le(U32)
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_accepts_integer_type_nodes_with_transforms_as_size_prefixes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = fixed_size(integer(u8), 42))]
        u32
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            IntegerTypeNode::le(U32)
                .add_transform(SizePrefixTransformNode::new(
                    IntegerTypeNode::le(U8).add_transform(FixedSizeTransformNode::new(42))
                ))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_adds_a_size_prefix_transform_to_any_overridden_type() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(size_prefix = integer(u8))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_replaces_existing_size_prefix_transforms() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(size_prefix = integer(u8))]
        #[codama(size_prefix = integer(u16))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U16)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_replaces_fixed_size_transforms() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(fixed_size = 42)]
        #[codama(size_prefix = integer(u8))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_appends_size_transforms_after_other_transforms() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = hidden_prefix(string, constant(integer(u8), 1)))]
        #[codama(size_prefix = integer(u8))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyTypeModifiersVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            StringTypeNode::utf8()
                .add_transform(HiddenPrefixTransformNode::new(vec![
                    ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(1u8))
                ]))
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_matches_the_size_prefix_written_in_the_dsl() -> CodamaResult<()> {
    // Via the modifier directive.
    let modifier_ast: syn::Field = syn::parse_quote! {
        #[codama(type = string)]
        #[codama(size_prefix = integer(u8))]
        String
    };
    let mut modifier_korok = FieldKorok::parse(&modifier_ast)?;
    modifier_korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    modifier_korok.accept(&mut ApplyTypeModifiersVisitor::new())?;

    // Via the type node in the DSL.
    let dsl_ast: syn::Field = syn::parse_quote! {
        #[codama(type = size_prefix(string, integer(u8)))]
        String
    };
    let mut dsl_korok = FieldKorok::parse(&dsl_ast)?;
    dsl_korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    dsl_korok.accept(&mut ApplyTypeModifiersVisitor::new())?;

    assert_eq!(modifier_korok.node, dsl_korok.node);
    assert_eq!(
        dsl_korok.node,
        Some(
            StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
                .into()
        )
    );
    Ok(())
}

#[test]
fn it_serializes_to_the_expected_json() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = size_prefix(string, integer(u32)))]
        String
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;

    let expected = r#"{"kind":"stringTypeNode","encoding":"utf8","transforms":[{"kind":"sizePrefixTransformNode","prefix":{"kind":"integerTypeNode","format":"u32","endian":"le"}}]}"#;
    assert_eq!(serde_json::to_string(&korok.node)?, expected);
    assert_eq!(
        serde_json::to_string(
            &StringTypeNode::utf8()
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
        )?,
        expected
    );
    Ok(())
}

#[test]
fn it_keeps_the_type_wrapped_in_a_struct_field_type_node() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(size_prefix = integer(u8))]
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
                IntegerTypeNode::le(U32)
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U8)))
            )
            .into()
        )
    );
    Ok(())
}
