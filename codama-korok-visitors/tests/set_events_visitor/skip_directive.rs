use crate::helpers::path;
use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefaultValueStrategy, DisplaySkip, Docs, EventNode, FieldDiscriminatorNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, IntegerValueNode, ProgramNode, StructFieldDisplayNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn skip_variant_in_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            Transfer {
                amount: u64,
            },
            #[codama(skip)]
            InternalEvent {},
            Burn {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        identifier: "transfer".into(),
                        docs: Docs::default(),
                        data: Box::new(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: "discriminator".into(),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(0u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                            ])
                            .into()
                        ),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        plugins: vec![],
                    },
                    EventNode {
                        identifier: "burn".into(),
                        docs: Docs::default(),
                        data: Box::new(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: "discriminator".into(),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(2u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                            ])
                            .into()
                        ),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        plugins: vec![],
                    }
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn skip_preserves_sibling_discriminator_counting() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            First,
            #[codama(skip)]
            Second,
            Third,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetEventsVisitor::new())?;

    // Third should have discriminator value 2 (not 1), because
    // Second still occupies slot 1 even though it's skipped.
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        identifier: "first".into(),
                        docs: Docs::default(),
                        data: Box::new(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        ),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        plugins: vec![],
                    },
                    EventNode {
                        identifier: "third".into(),
                        docs: Docs::default(),
                        data: Box::new(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(2u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        ),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        plugins: vec![],
                    }
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn skip_all_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            #[codama(skip)]
            First,
            #[codama(skip)]
            Second,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
