use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetDefinedTypesVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, DefinedTypeNode, EnumEmptyVariantTypeNode, EnumStructVariantTypeNode,
    EnumTupleVariantTypeNode, EnumTypeNode,
    NumberFormat::{I32, U32, U8},
    NumberTypeNode, SizePrefixTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
    TupleTypeNode,
};

#[test]
fn it_sets_defined_types_on_structs_with_nammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Person {
            age: u8,
            name: String,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "person",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                    StructFieldTypeNode::new(
                        "name",
                        SizePrefixTypeNode::new(StringTypeNode::utf8(), NumberTypeNode::le(U32))
                    ),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_sets_defined_types_on_structs_with_unnammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Coordinates(u32, u32);
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "coordinates",
                TupleTypeNode::new(vec![
                    NumberTypeNode::le(U32).into(),
                    NumberTypeNode::le(U32).into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_sets_defined_types_on_structs_with_single_unnammed_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Age(u8);
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("age", NumberTypeNode::le(U8)).into())
    );
    Ok(())
}

#[test]
fn it_sets_defined_types_on_structs_with_unit_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Empty;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("empty", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}

#[test]
fn it_sets_defined_types_on_enums() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode::new(vec![
                    EnumEmptyVariantTypeNode::new("quit").into(),
                    EnumStructVariantTypeNode::new(
                        "move",
                        StructTypeNode::new(vec![
                            StructFieldTypeNode::new("x", NumberTypeNode::le(I32)),
                            StructFieldTypeNode::new("y", NumberTypeNode::le(I32)),
                        ])
                    )
                    .into(),
                    EnumTupleVariantTypeNode::new(
                        "write",
                        TupleTypeNode::new(vec![SizePrefixTypeNode::new(
                            StringTypeNode::utf8(),
                            NumberTypeNode::le(U32)
                        )
                        .into()])
                    )
                    .into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_sets_defined_types_on_enums_with_explicit_discriminators() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Message {
            Write,
            Move = 42,
            Run,
            Quit = 100
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode::new(vec![
                    EnumEmptyVariantTypeNode::new("write").into(),
                    EnumEmptyVariantTypeNode {
                        name: "move".into(),
                        discriminator: Some(42)
                    }
                    .into(),
                    EnumEmptyVariantTypeNode::new("run").into(),
                    EnumEmptyVariantTypeNode {
                        name: "quit".into(),
                        discriminator: Some(100)
                    }
                    .into(),
                ])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_fails_if_nammed_fields_have_no_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Foo { bar: NoNode }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `bar` in struct `Foo` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_nammed_fields_are_not_struct_field_type_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Foo { bar: NotStructFieldTypeNode }
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node = Some(StringTypeNode::utf8().into());

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `bar` in struct `Foo` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_unnammed_fields_have_no_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Foo (NoNode);
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in struct `Foo` does not resolve to a `TypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_unnammed_fields_are_not_type_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        struct Foo (NotTypeNode);
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.fields[0].node =
        Some(StructFieldTypeNode::new("notATypeNode", BooleanTypeNode::default()).into());

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in struct `Foo` does not resolve to a `TypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_struct_enum_variant_fields_have_no_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Foo { Bar { baz: NoNode } }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `baz` in variant `Bar` of enum `Foo` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_struct_enum_variant_fields_are_not_struct_field_type_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Foo { Bar { baz: NotStructFieldTypeNode } }
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.variants[0].fields[0].node = Some(StringTypeNode::utf8().into());

    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `baz` in variant `Bar` of enum `Foo` does not resolve to a `structFieldTypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_tuple_enum_variant_fields_have_no_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Foo { Bar(NoNode) }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in variant `Bar` of enum `Foo` does not resolve to a `TypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_tuple_enum_variant_fields_are_not_type_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Foo { Bar(NotTypeNode) }
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.variants[0].fields[0].node =
        Some(StructFieldTypeNode::new("notATypeNode", BooleanTypeNode::default()).into());

    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Field `0` in variant `Bar` of enum `Foo` does not resolve to a `TypeNode`"
    );
    Ok(())
}

#[test]
fn it_fails_if_enum_variants_are_not_valid_enum_variant_nodes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        enum Foo { Bar }
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.variants[0].node = Some(StringTypeNode::utf8().into());

    let error = korok
        .accept(&mut SetDefinedTypesVisitor::new())
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Variant `Bar` of enum `Foo` does not resolve to a `EnumVariantTypeNode`"
    );
    Ok(())
}

#[test]
fn it_uses_the_name_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaType)]
        #[codama(name = "person")]
        struct HumanData;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetDefinedTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(DefinedTypeNode::new("person", StructTypeNode::new(vec![])).into())
    );
    Ok(())
}
