use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, BooleanTypeNode, BytesEncoding, ConstantDiscriminatorNode, ConstantPdaSeedNode,
    ConstantValueNode, DefaultValueStrategy, Docs, FieldDiscriminatorNode,
    NumberFormat::{U32, U64, U8},
    NumberTypeNode, NumberValueNode, OptionTypeNode, PdaLinkNode, PdaNode, ProgramNode,
    PublicKeyTypeNode, SizeDiscriminatorNode, StringTypeNode, StringValueNode, StructFieldTypeNode,
    StructTypeNode, VariablePdaSeedNode,
};

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            Mint {
                mint_authority: Pubkey,
                freeze_authority: Option<Pubkey>,
                supply: u64,
            },
            Token {
                mint: Pubkey,
                owner: Pubkey,
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        name: "mint".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            StructFieldTypeNode::new("mint_authority", PublicKeyTypeNode::new()),
                            StructFieldTypeNode::new("freeze_authority", OptionTypeNode::new(PublicKeyTypeNode::new())),
                            StructFieldTypeNode::new("supply", NumberTypeNode::le(U64)),
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "token".into(),
                        size: None,
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
                            StructFieldTypeNode::new("owner", PublicKeyTypeNode::new()),
                            StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            Mint {},
            Token,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        name: "mint".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "token".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
        #[derive(CodamaAccounts)]
        #[repr(u32)]
        enum MyProgramAccounts {
            Mint,
            Token,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        name: "mint".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U32).into(),
                                default_value: Some(NumberValueNode::new(0u32).into()),
                            },
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "token".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U32).into(),
                                default_value: Some(NumberValueNode::new(1u32).into()),
                            },
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            Mint,
            Token = 42,
            AssociatedToken,
            Buffer = 100,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        name: "mint".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "token".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(42u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "associatedToken".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(43u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    },
                    AccountNode {
                        name: "buffer".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(100u8).into()),
                            }
                        ]).into(),
                        pda: None,
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
        #[derive(CodamaAccounts)]
        struct Mint {
            mint_authority: Pubkey,
            freeze_authority: Option<Pubkey>,
            supply: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already is a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {}
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(name = "token")]
            TokenAccount,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    name: "token".into(),
                    size: None,
                    docs: Docs::default(),
                    data: StructTypeNode::new(vec![StructFieldTypeNode {
                        name: "discriminator".into(),
                        default_value_strategy: Some(DefaultValueStrategy::Omitted),
                        docs: Docs::default(),
                        r#type: NumberTypeNode::le(U8).into(),
                        default_value: Some(NumberValueNode::new(0u8).into()),
                    }])
                    .into(),
                    pda: None,
                    discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_discriminator_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(discriminator(size = 100))]
            #[codama(discriminator(bytes = "01020304", offset = 42))]
            Token,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    name: "token".into(),
                    size: None,
                    docs: Docs::default(),
                    data: StructTypeNode::new(vec![StructFieldTypeNode {
                        name: "discriminator".into(),
                        default_value_strategy: Some(DefaultValueStrategy::Omitted),
                        docs: Docs::default(),
                        r#type: NumberTypeNode::le(U8).into(),
                        default_value: Some(NumberValueNode::new(0u8).into()),
                    }])
                    .into(),
                    pda: None,
                    discriminators: vec![
                        FieldDiscriminatorNode::new("discriminator", 0).into(),
                        SizeDiscriminatorNode::new(100).into(),
                        ConstantDiscriminatorNode::new(
                            ConstantValueNode::bytes(BytesEncoding::Base16, "01020304"),
                            42
                        )
                        .into()
                    ],
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_enum_discriminator_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        #[codama(enum_discriminator(name = "banana", size = number(u64)))]
        enum MyProgramAccounts {
            Token,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    name: "token".into(),
                    size: None,
                    docs: Docs::default(),
                    data: StructTypeNode::new(vec![StructFieldTypeNode {
                        name: "banana".into(),
                        default_value_strategy: Some(DefaultValueStrategy::Omitted),
                        docs: Docs::default(),
                        r#type: NumberTypeNode::le(U64).into(),
                        default_value: Some(NumberValueNode::new(0u64).into()),
                    }])
                    .into(),
                    pda: None,
                    discriminators: vec![FieldDiscriminatorNode::new("banana", 0).into()],
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_seed_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(seed(type = string(utf8), value = "counter_pda"))]
            #[codama(seed(name = "authority"))]
            Counter {
                authority: Pubkey,
            },
            #[codama(seed(name = "owner", type = public_key))]
            #[codama(seed(name = "mint", type = public_key))]
            Token,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        pda: Some(PdaLinkNode::new("counter")),
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..AccountNode::new(
                            "counter",
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    name: "discriminator".into(),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: NumberTypeNode::le(U8).into(),
                                    default_value: Some(NumberValueNode::new(0u8).into()),
                                },
                                StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
                            ])
                        )
                    },
                    AccountNode {
                        pda: Some(PdaLinkNode::new("token")),
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..AccountNode::new(
                            "token",
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            },])
                        )
                    }
                ],
                pdas: vec![
                    PdaNode::new(
                        "counter",
                        vec![
                            ConstantPdaSeedNode::new(
                                StringTypeNode::utf8(),
                                StringValueNode::new("counter_pda")
                            )
                            .into(),
                            VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),
                        ]
                    ),
                    PdaNode::new(
                        "token",
                        vec![
                            VariablePdaSeedNode::new("owner", PublicKeyTypeNode::new()).into(),
                            VariablePdaSeedNode::new("mint", PublicKeyTypeNode::new()).into(),
                        ]
                    )
                ],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_pda_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(pda = "my_counter_pda")]
            Counter,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    pda: Some(PdaLinkNode::new("myCounterPda")),
                    discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    ..AccountNode::new(
                        "counter",
                        StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(0u8).into()),
                        },])
                    )
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_pda_and_seed_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(pda = "my_counter_pda")]
            #[codama(seed(name = "authority"))]
            Counter {
                authority: Pubkey,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![AccountNode {
                    pda: Some(PdaLinkNode::new("myCounterPda")),
                    discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                    ..AccountNode::new(
                        "counter",
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
                        ])
                    )
                }],
                pdas: vec![PdaNode::new(
                    "myCounterPda",
                    vec![VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into(),]
                )],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
