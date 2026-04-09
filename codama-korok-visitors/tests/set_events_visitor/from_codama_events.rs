use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, DefaultValueStrategy, Docs, EventNode, FieldDiscriminatorNode,
    NumberFormat::{U32, U64, U8},
    NumberTypeNode, NumberValueNode, ProgramNode, PublicKeyTypeNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            Transfer {
                authority: Pubkey,
                amount: u64,
            },
            Burn {
                mint: Pubkey,
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "transfer".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
                            StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                        ])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "burn".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            },
                            StructFieldTypeNode::new("mint", PublicKeyTypeNode::new()),
                            StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                        ])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
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
fn from_enum_with_empty_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            Ping {},
            Pong,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "ping".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(0u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "pong".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(1u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
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
fn from_enum_with_custom_enum_size() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        #[repr(u32)]
        enum MyProgramEvents {
            Transfer,
            Burn,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "transfer".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U32).into(),
                            default_value: Some(NumberValueNode::new(0u32).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "burn".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U32).into(),
                            default_value: Some(NumberValueNode::new(1u32).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
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
fn from_enum_with_explicit_discriminators() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            Transfer,
            Burn = 42,
            Mint,
            Close = 100,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "transfer".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(0u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "burn".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(42u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "mint".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(43u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "close".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(100u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        struct MyEvent {
            mint: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {}
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            #[codama(name = "token_transfer")]
            Transfer,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![EventNode {
                    name: "tokenTransfer".into(),
                    docs: Docs::default(),
                    data: StructTypeNode::new(vec![StructFieldTypeNode {
                        name: "discriminator".into(),
                        default_value_strategy: Some(DefaultValueStrategy::Omitted),
                        docs: Docs::default(),
                        r#type: NumberTypeNode::le(U8).into(),
                        default_value: Some(NumberValueNode::new(0u8).into()),
                    }])
                    .into(),
                    discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
