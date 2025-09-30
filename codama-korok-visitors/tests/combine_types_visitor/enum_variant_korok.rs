use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::EnumVariantKorok;
use codama_nodes::{
    EnumEmptyVariantTypeNode, EnumStructVariantTypeNode, EnumTupleVariantTypeNode, NumberTypeNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, I32, U64,
};

#[test]
fn it_creates_enum_empty_variants() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo };
    let mut korok = EnumVariantKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(EnumEmptyVariantTypeNode::new("foo").into())
    );
    Ok(())
}

#[test]
fn it_creates_enum_struct_variants() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo { x: i32, y: i32 } };
    let mut korok = EnumVariantKorok::parse(&ast)?;
    korok.fields[0].node = Some(StructFieldTypeNode::new("x", NumberTypeNode::le(I32)).into());
    korok.fields[1].node = Some(StructFieldTypeNode::new("y", NumberTypeNode::le(I32)).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            EnumStructVariantTypeNode::new(
                "foo",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                    StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_creates_enum_tuple_variants() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo (u64, String) };
    let mut korok = EnumVariantKorok::parse(&ast)?;
    korok.fields[0].node = Some(NumberTypeNode::le(U64).into());
    korok.fields[1].node = Some(StringTypeNode::utf8().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            EnumTupleVariantTypeNode::new(
                "foo",
                TupleTypeNode::new(vec![
                    NumberTypeNode::le(U64).into(),
                    StringTypeNode::utf8().into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_keeps_track_of_the_variant_discriminant() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo = 42 };
    let mut korok = EnumVariantKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            EnumEmptyVariantTypeNode {
                name: "foo".into(),
                discriminator: Some(42)
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_does_not_override_existing_nodes_by_default() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo };
    let mut korok = EnumVariantKorok::parse(&ast)?;

    korok.node = Some(EnumEmptyVariantTypeNode::new("bar").into());
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(EnumEmptyVariantTypeNode::new("bar").into())
    );
    Ok(())
}

#[test]
fn it_fails_if_struct_variant_fields_are_missing() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo { bar: u64 } };
    let mut korok = EnumVariantKorok::parse(&ast)?;
    korok.fields[0].node = None;

    assert_eq!(korok.node, None);
    let mut visitor = CombineTypesVisitor {
        parent_enum: "MyEnum".into(),
        ..CombineTypesVisitor::strict()
    };
    let error = korok.accept(&mut visitor).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `bar` in variant `Foo` of enum `MyEnum` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_tuple_variant_fields_are_missing() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! { Foo (u64) };
    let mut korok = EnumVariantKorok::parse(&ast)?;
    korok.fields[0].node = None;

    assert_eq!(korok.node, None);
    let mut visitor = CombineTypesVisitor {
        parent_enum: "MyEnum".into(),
        ..CombineTypesVisitor::strict()
    };
    let error = korok.accept(&mut visitor).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in variant `Foo` of enum `MyEnum` does not resolve to a `TypeNode`"
    );
    Ok(())
}

#[test]
fn it_uses_the_name_directive() -> CodamaResult<()> {
    let ast: syn::Variant = syn::parse_quote! {
        #[codama(name = "bar")]
        Foo
    };
    let mut korok = EnumVariantKorok::parse(&ast)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(EnumEmptyVariantTypeNode::new("bar").into())
    );
    Ok(())
}
