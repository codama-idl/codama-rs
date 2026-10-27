use codama_errors::CodamaResult;
use codama_korok_visitors::{CombineTypesVisitor, KorokVisitable};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefinedTypeNode, EnumTypeNode, EnumVariantTypeNode,
    IntegerFormat::{U32, U8},
    IntegerTypeNode, Node, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, I32,
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
