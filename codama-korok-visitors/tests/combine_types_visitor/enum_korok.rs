use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefinedTypeNode, EnumTypeNode, EnumVariantTypeNode,
    IntegerFormat::{U32, U8},
    IntegerTypeNode, Node, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode,
    TypeNode, I32,
};

#[test]
fn it_creates_a_defined_type_enum_from_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
        }
    };
    let mut korok = EnumKorok::parse(&item)?;
    let variant_quit = EnumVariantTypeNode::new("quit");
    let variant_move = EnumVariantTypeNode::with_data(
        "move",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("x", IntegerTypeNode::le(I32)),
            StructFieldTypeNode::new("y", IntegerTypeNode::le(I32)),
        ]),
    );
    let variant_write = EnumVariantTypeNode::with_data(
        "write",
        TupleTypeNode::new(vec![StringTypeNode::utf8().into()]),
    );
    korok.variants[0].node = Some(variant_quit.clone().into());
    korok.variants[1].node = Some(variant_move.clone().into());
    korok.variants[2].node = Some(variant_write.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode::new(vec![variant_quit, variant_move, variant_write,])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_does_not_override_existing_nodes_by_default() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { enum Direction { Left } };
    let mut korok = EnumKorok::parse(&item)?;
    korok.variants[0].node = Some(EnumVariantTypeNode::new("left").into());

    let original_node = Some(Node::from(DefinedTypeNode::new(
        "direction",
        EnumTypeNode::new(vec![EnumVariantTypeNode::new("right")]),
    )));
    korok.node = original_node.clone();
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(korok.node, original_node);
    Ok(())
}

#[test]
fn it_ignores_invalid_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! { enum Direction { Left, InvalidNode } };
    let mut korok = EnumKorok::parse(&item)?;
    korok.variants[0].node = Some(EnumVariantTypeNode::new("left").into());
    korok.variants[1].node = Some(IntegerTypeNode::le(I32).into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "direction",
                EnumTypeNode::new(vec![EnumVariantTypeNode::new("left"),])
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_creates_defined_enums_with_custom_sizes_using_repr_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[repr(u32)]
        enum Message { Hello, Bye }
    };
    let mut korok = EnumKorok::parse(&item)?;
    let variant_hello = EnumVariantTypeNode::new("hello");
    let variant_bye = EnumVariantTypeNode::new("bye");
    korok.variants[0].node = Some(variant_hello.clone().into());
    korok.variants[1].node = Some(variant_bye.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode {
                    variants: vec![variant_hello, variant_bye,],
                    size: IntegerTypeNode::le(U32),
                    transforms: vec![],
                    plugins: vec![],
                }
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_creates_defined_enums_with_custom_sizes_using_enum_discriminator_directives(
) -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[codama(enum_discriminator(size = integer(u32)))]
        enum Message { Hello, Bye }
    };
    let mut korok = EnumKorok::parse(&item)?;
    let variant_hello = EnumVariantTypeNode::new("hello");
    let variant_bye = EnumVariantTypeNode::new("bye");
    korok.variants[0].node = Some(variant_hello.clone().into());
    korok.variants[1].node = Some(variant_bye.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode {
                    variants: vec![variant_hello, variant_bye],
                    size: IntegerTypeNode::le(U32),
                    transforms: vec![],
                    plugins: vec![],
                }
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_prefers_enum_discriminator_sizes_over_repr_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[codama(enum_discriminator(size = integer(u32)))]
        #[repr(u8)]
        enum Message { Hello, Bye }
    };
    let mut korok = EnumKorok::parse(&item)?;
    let variant_hello = EnumVariantTypeNode::new("hello");
    let variant_bye = EnumVariantTypeNode::new("bye");
    korok.variants[0].node = Some(variant_hello.clone().into());
    korok.variants[1].node = Some(variant_bye.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode {
                    variants: vec![variant_hello, variant_bye],
                    size: IntegerTypeNode::le(U32),
                    transforms: vec![],
                    plugins: vec![],
                }
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_defaults_defined_enums_to_u8_without_repr_or_enum_discriminator() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        enum Message { Hello, Bye }
    };
    let mut korok = EnumKorok::parse(&item)?;
    let variant_hello = EnumVariantTypeNode::new("hello");
    let variant_bye = EnumVariantTypeNode::new("bye");
    korok.variants[0].node = Some(variant_hello.clone().into());
    korok.variants[1].node = Some(variant_bye.clone().into());

    assert_eq!(korok.node, None);
    korok.accept(&mut CombineTypesVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            DefinedTypeNode::new(
                "message",
                EnumTypeNode {
                    variants: vec![variant_hello, variant_bye],
                    size: IntegerTypeNode::le(U8),
                    transforms: vec![],
                    plugins: vec![],
                }
            )
            .into()
        )
    );
    Ok(())
}

/// The discriminators of the variants of `item` once combined, `None` where the index applies.
fn discriminators(item: syn::Item) -> CodamaResult<Vec<Option<u32>>> {
    let mut korok = EnumKorok::parse(&item)?;
    korok.accept(&mut CombineTypesVisitor::new())?;
    let Some(Node::DefinedType(defined_type)) = korok.node else {
        panic!("Expected a defined type node");
    };
    let TypeNode::Enum(r#enum) = *defined_type.r#type else {
        panic!("Expected an enum type node");
    };
    Ok(r#enum
        .variants
        .iter()
        .map(|variant| variant.discriminator)
        .collect())
}

#[test]
fn it_numbers_the_variants_following_an_explicit_discriminant_like_rust() -> CodamaResult<()> {
    // Rust gives `Frozen` the value 6, whereas an omitted discriminator means the index, i.e. 1.
    let item: syn::Item = syn::parse_quote! {
        enum Status { Active = 5, Frozen, Closed = 10, Deleted }
    };
    assert_eq!(
        discriminators(item)?,
        [Some(5), Some(6), Some(10), Some(11)]
    );
    Ok(())
}

#[test]
fn it_omits_discriminators_matching_the_index_of_the_variant() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        enum Status { Active, Frozen = 1, Closed }
    };
    assert_eq!(discriminators(item)?, [None, None, None]);
    Ok(())
}

#[test]
fn it_rejects_discriminants_that_are_not_unsigned_integer_literals() {
    for discriminant in ["-1", "1 << 2", "SOME_CONST", "4294967296"] {
        let tokens: proc_macro2::TokenStream = discriminant.parse().unwrap();
        let item: syn::Item = syn::parse_quote! {
            enum Status { Active = #tokens, Frozen }
        };
        let error = discriminators(item).unwrap_err();
        assert!(
            error.to_string().contains("integer"),
            "{discriminant}: {error}"
        );
    }
}

#[test]
fn it_accepts_a_last_variant_at_the_largest_discriminator() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        enum Status { Active = 4294967295 }
    };
    assert_eq!(discriminators(item)?, [Some(u32::MAX)]);
    Ok(())
}

#[test]
fn it_rejects_a_variant_following_the_largest_discriminator() {
    let item: syn::Item = syn::parse_quote! {
        enum Status { Active = 4294967295, Frozen }
    };
    assert_eq!(
        discriminators(item).unwrap_err().to_string(),
        "the discriminator of `frozen` overflows a u32"
    );
}
