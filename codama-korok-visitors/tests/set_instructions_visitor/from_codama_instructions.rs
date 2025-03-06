use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetBorshTypesVisitor, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    BooleanTypeNode, DefaultValueStrategy, Docs, FieldDiscriminatorNode, InstructionAccountNode,
    InstructionArgumentNode, InstructionNode,
    NumberFormat::{U32, U64, U8},
    NumberTypeNode, NumberValueNode, ProgramNode,
};

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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "initialize".into(),
                        accounts: vec![
                            InstructionAccountNode::new("authority", false, false),
                            InstructionAccountNode::new("payer", true, true),
                        ],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        accounts: vec![InstructionAccountNode::new("authority", false, true)],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
fn from_enum_with_arguments_only() -> CodamaResult<()> {
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "initialize".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "initialize".into(),
                        accounts: vec![
                            InstructionAccountNode::new("authority", false, false),
                            InstructionAccountNode::new("payer", true, true),
                        ],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            }
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        accounts: vec![InstructionAccountNode::new("authority", false, true)],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            }
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "initialize".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            }
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            }
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
            #[codama(account(name = "authority"))]
            #[codama(account(name = "payer", signer, writable))]
            Initialize {
                amount: u64,
            },
            #[codama(account(name = "authority", signer))]
            Update {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetBorshTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "initialize".into(),
                        accounts: vec![
                            InstructionAccountNode::new("authority", false, false),
                            InstructionAccountNode::new("payer", true, true),
                        ],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        accounts: vec![InstructionAccountNode::new("authority", false, true)],
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(1u8).into()),
                            },
                            InstructionArgumentNode::new("amount", NumberTypeNode::le(U64))
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
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
fn with_custom_enum_size() -> CodamaResult<()> {
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
                        name: "initialize".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U32).into(),
                                default_value: Some(NumberValueNode::new(0u32).into()),
                            },
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "update".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U32).into(),
                                default_value: Some(NumberValueNode::new(1u32).into()),
                            },
                        ],
                        discriminators: vec![FieldDiscriminatorNode::new("discriminator", 0).into()],
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
