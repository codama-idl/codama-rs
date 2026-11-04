use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{CrateKorok, EnumKorok, StructKorok};
use codama_nodes::test_utils::{id, path};
use codama_nodes::{
    BooleanTypeNode, BytesEncoding, ConstantDiscriminatorNode, ConstantValueNode,
    DefaultValueStrategy, DefinedTypeLinkNode, DisplaySkip, Docs, FieldDiscriminatorNode,
    InstructionAccountNode, InstructionNode,
    IntegerFormat::{U32, U64, U8},
    IntegerTypeNode, IntegerValueNode, ProgramNode, SizeDiscriminatorNode, StructFieldDisplayNode,
    StructFieldTypeNode, StructTypeNode,
};
use codama_stores::CrateStore;
use quote::quote;

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize {
                #[codama(account)]
                authority: AccountMeta,
                #[codama(account(signer, writable))]
                payer: AccountMeta,
                amount: u64,
            },
            Update {
                #[codama(account(signer))]
                authority: AccountMeta,
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        accounts: vec![
                            InstructionAccountNode::new(id("authority"), false, false),
                            InstructionAccountNode::new(id("payer"), true, true),
                        ],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
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
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        accounts: vec![InstructionAccountNode::new(id("authority"), false, true)],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(1u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
fn from_enum_with_data_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize {
                amount: u64,
            },
            Update {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
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
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(1u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
fn from_enum_with_accounts_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize {
                #[codama(account)]
                authority: AccountMeta,
                #[codama(account(signer, writable))]
                payer: AccountMeta,
            },
            Update {
                #[codama(account(signer))]
                authority: AccountMeta,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        accounts: vec![
                            InstructionAccountNode::new(id("authority"), false, false),
                            InstructionAccountNode::new(id("payer"), true, true),
                        ],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        accounts: vec![InstructionAccountNode::new(id("authority"), false, true)],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(1u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize {},
            Update,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(1u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
fn from_enum_with_accounts_as_struct_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(account(identifier = "authority"))]
            #[codama(account(identifier = "payer", signer, writable))]
            Initialize {
                amount: u64,
            },
            #[codama(account(identifier = "authority", signer))]
            Update {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        accounts: vec![
                            InstructionAccountNode::new(id("authority"), false, false),
                            InstructionAccountNode::new(id("payer"), true, true),
                        ],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
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
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        accounts: vec![InstructionAccountNode::new(id("authority"), false, true)],
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(1u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        #[repr(u32)]
        enum MyProgramInstructions {
            Initialize,
            Update
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U32).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u32).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            },])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U32).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(1u32).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            },])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize,
            Update = 42,
            Write,
            Close = 100,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("initialize"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("update"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(42u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("write"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(43u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("close"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(100u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable))]
            payer: AccountMeta,
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(korok.node, None);
    // No visitor error because there is already is a compilation error.
    Ok(())
}

#[test]
fn no_overrides() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {}
    };
    let mut korok = EnumKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(identifier = "initialize")]
            InitializeInstruction,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![StructFieldTypeNode {
                            identifier: id("discriminator"),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: Box::new(IntegerTypeNode::le(U8).into()),
                            default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                            display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                            plugins: vec![],
                        }])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(discriminator(size = 100))]
            #[codama(discriminator(bytes = "01020304", offset = 42))]
            Initialize,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![StructFieldTypeNode {
                            identifier: id("discriminator"),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: Box::new(IntegerTypeNode::le(U8).into()),
                            default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                            display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                            plugins: vec![],
                        }])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into(),
                        SizeDiscriminatorNode::new(100).into(),
                        ConstantDiscriminatorNode::new(
                            ConstantValueNode::bytes(BytesEncoding::Base16, "01020304"),
                            42
                        )
                        .into()
                    ],
                    ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        #[codama(enum_discriminator(identifier = "banana", size = integer(u64)))]
        enum MyProgramInstructions {
            Initialize,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![StructFieldTypeNode {
                            identifier: id("banana"),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: Box::new(IntegerTypeNode::le(U64).into()),
                            default_value: Box::new(Some(IntegerValueNode::new(0u64).into())),
                            display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                            plugins: vec![],
                        }])
                        .into()
                    )),
                    discriminators: vec![FieldDiscriminatorNode::new(path("banana"), 0).into()],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_field_attributes_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(field("space", integer(u64)))]
            #[codama(field("lamports", integer(u64)))]
            Initialize,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                default_value: Box::new(Some(IntegerValueNode::new(0u64).into())),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                ..StructFieldTypeNode::new(
                                    id("discriminator"),
                                    IntegerTypeNode::le(U8)
                                )
                            },
                            StructFieldTypeNode::new(id("space"), IntegerTypeNode::le(U64)),
                            StructFieldTypeNode::new(id("lamports"), IntegerTypeNode::le(U64))
                        ])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_prepended_field_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(field("space", integer(u64)))]
            Initialize { lamports: u64 },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                default_value: Box::new(Some(IntegerValueNode::new(0u64).into())),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                ..StructFieldTypeNode::new(
                                    id("discriminator"),
                                    IntegerTypeNode::le(U8)
                                )
                            },
                            StructFieldTypeNode::new(id("space"), IntegerTypeNode::le(U64)),
                            StructFieldTypeNode::new(id("lamports"), IntegerTypeNode::le(U64))
                        ])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_appended_field_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(field(after, "space", integer(u64)))]
            Initialize { lamports: u64 },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("initialize"),
                    data: Box::new(Some(
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                default_value: Box::new(Some(IntegerValueNode::new(0u64).into())),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                ..StructFieldTypeNode::new(
                                    id("discriminator"),
                                    IntegerTypeNode::le(U8)
                                )
                            },
                            StructFieldTypeNode::new(id("lamports"), IntegerTypeNode::le(U64)),
                            StructFieldTypeNode::new(id("space"), IntegerTypeNode::le(U64)),
                        ])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_tuple_variants() -> CodamaResult<()> {
    // Tuple args typically use named types for clarity since they lack field names.
    let store = CrateStore::hydrate(quote! {
        #[derive(CodamaType)]
        struct Percentage(u64);

        #[derive(CodamaInstructions)]
        enum FluxCapacitorInstructions {
            #[codama(account(identifier = "clock_sysvar"))]
            Charge(Percentage, bool),
        }
    })
    .unwrap();
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let codama_koroks::ItemKorok::Enum(instructions_korok) = &korok.items[1] else {
        panic!("Expected enum korok");
    };

    assert_eq!(
        instructions_korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("charge"),
                    accounts: vec![InstructionAccountNode::new(
                        id("clock_sysvar"),
                        false,
                        false
                    )],
                    data: Box::new(Some(
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            },
                            StructFieldTypeNode::new(
                                id("arg0"),
                                DefinedTypeLinkNode::new(id("percentage"))
                            ),
                            StructFieldTypeNode::new(id("arg1"), BooleanTypeNode::default()),
                        ])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum_with_mixed_variants() -> CodamaResult<()> {
    // Tuple args typically use named types for clarity since they lack field names.
    let store = CrateStore::hydrate(quote! {
        #[derive(CodamaType)]
        enum Direction {
            Fore,
            Aft,
            Port,
            Starboard,
        }

        #[derive(CodamaInstructions)]
        enum KobayashiMaruInstructions {
            Warp { factor: u64 },
            FireTorpedoes(Direction),
            Cheat,
        }
    })
    .unwrap();
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    // Get the instructions enum (second item).
    let codama_koroks::ItemKorok::Enum(instructions_korok) = &korok.items[1] else {
        panic!("Expected enum korok");
    };

    assert_eq!(
        instructions_korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: id("warp"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
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
                                StructFieldTypeNode::new(id("factor"), IntegerTypeNode::le(U64)),
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("fire_torpedoes"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: id("discriminator"),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(1u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new(
                                    id("arg0"),
                                    DefinedTypeLinkNode::new(id("direction"))
                                ),
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: id("cheat"),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(2u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
fn from_enum_with_tuple_variants_with_custom_names() -> CodamaResult<()> {
    // Tuple args can have custom names via #[codama(identifier = "...")].
    let store = CrateStore::hydrate(quote! {
        #[derive(CodamaType)]
        struct Percentage(u64);

        #[derive(CodamaInstructions)]
        enum FluxCapacitorInstructions {
            #[codama(account(identifier = "clock_sysvar"))]
            Charge(
                #[codama(identifier = "percentage")]
                Percentage,
                #[codama(identifier = "is_valid")]
                bool,
            ),
        }
    })
    .unwrap();
    let mut korok = CrateKorok::parse(&store)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let codama_koroks::ItemKorok::Enum(instructions_korok) = &korok.items[1] else {
        panic!("Expected enum korok");
    };

    assert_eq!(
        instructions_korok.node,
        Some(
            ProgramNode {
                instructions: vec![InstructionNode {
                    identifier: id("charge"),
                    accounts: vec![InstructionAccountNode::new(
                        id("clock_sysvar"),
                        false,
                        false
                    )],
                    data: Box::new(Some(
                        StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                identifier: id("discriminator"),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            },
                            StructFieldTypeNode::new(
                                id("percentage"),
                                DefinedTypeLinkNode::new(id("percentage"))
                            ),
                            StructFieldTypeNode::new(id("is_valid"), BooleanTypeNode::default()),
                        ])
                        .into()
                    )),
                    discriminators: vec![
                        FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                    ],
                    ..InstructionNode::default()
                }],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
