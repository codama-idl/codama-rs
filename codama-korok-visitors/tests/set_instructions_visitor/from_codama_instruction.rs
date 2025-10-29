use codama_errors::CodamaResult;
use codama_korok_visitors::{
    IdentifyFieldTypesVisitor, KorokVisitable, SetDefaultValuesVisitor, SetInstructionsVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountValueNode, ArgumentValueNode, BooleanTypeNode, BooleanValueNode, DefaultValueStrategy,
    Docs, FieldDiscriminatorNode, InstructionAccountNode, InstructionArgumentNode, InstructionNode,
    NumberFormat::{U64, U8},
    NumberTypeNode, NumberValueNode, PayerValueNode, PublicKeyValueNode, SizeDiscriminatorNode,
    StringTypeNode,
};

#[test]
fn from_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable))]
            payer: AccountMeta,
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_arguments_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_accounts_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable))]
            payer: AccountMeta,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_accounts_as_struct_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "authority"))]
        #[codama(account(name = "payer", signer, writable))]
        struct Initialize {
            amount: u64,
            is_canonical: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                arguments: vec![
                    InstructionArgumentNode::new("amount", NumberTypeNode::le(U64)),
                    InstructionArgumentNode::new("is_canonical", BooleanTypeNode::default(),),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_default_values_in_accounts() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "rent_sysvar", default_value = sysvar("rent")))]
        #[codama(account(name = "token_program", default_value = program("token")))]
        struct Initialize {
            #[codama(account)]
            authority: AccountMeta,
            #[codama(account(signer, writable, default_value = payer))]
            payer: AccountMeta,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode {
                        name: "rentSysvar".into(),
                        default_value: Some(
                            PublicKeyValueNode::new("SysvarRent111111111111111111111111111111111")
                                .into()
                        ),
                        is_writable: false,
                        is_signer: false.into(),
                        is_optional: false,
                        docs: Docs::default(),
                    },
                    InstructionAccountNode {
                        name: "tokenProgram".into(),
                        default_value: Some(
                            PublicKeyValueNode::new("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
                                .into()
                        ),
                        is_writable: false,
                        is_signer: false.into(),
                        is_optional: false,
                        docs: Docs::default(),
                    },
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode {
                        name: "payer".into(),
                        is_signer: true.into(),
                        is_writable: true,
                        default_value: Some(PayerValueNode::new().into()),
                        is_optional: false,
                        docs: Docs::default(),
                    },
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_default_values_in_arguments() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(argument("discriminator", number(u8), default_value = 0))]
        struct Initialize {
            capacity: u64,
            #[codama(default_value = argument("capacity"))]
            max_capacity: u64,
            #[codama(default_value = false)]
            with_metadata: bool,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetDefaultValuesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                arguments: vec![
                    InstructionArgumentNode {
                        default_value: Some(NumberValueNode::new(0u8).into()),
                        ..InstructionArgumentNode::new("discriminator", NumberTypeNode::le(U8))
                    },
                    InstructionArgumentNode::new("capacity", NumberTypeNode::le(U64)),
                    InstructionArgumentNode {
                        default_value: Some(ArgumentValueNode::new("capacity").into()),
                        ..InstructionArgumentNode::new("max_capacity", NumberTypeNode::le(U64))
                    },
                    InstructionArgumentNode {
                        default_value: Some(BooleanValueNode::new(false).into()),
                        ..InstructionArgumentNode::new("with_metadata", BooleanTypeNode::default())
                    }
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_default_values_linking_to_other_accounts() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(account(name = "authority"))]
        #[codama(account(name = "payer", default_value = account("authority")))]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode {
                        default_value: Some(AccountValueNode::new("authority").into()),
                        ..InstructionAccountNode::new("payer", false, false)
                    },
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_default_values_linking_to_other_arguments() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            capacity: u64,
            #[codama(default_value = argument("capacity"))]
            max_capacity: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                arguments: vec![
                    InstructionArgumentNode::new("capacity", NumberTypeNode::le(U64)),
                    InstructionArgumentNode {
                        default_value: Some(ArgumentValueNode::new("capacity").into()),
                        ..InstructionArgumentNode::new("max_capacity", NumberTypeNode::le(U64))
                    },
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_empty_struct() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        enum MyInstructions {
            Initialize,
            Update,
            Close,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

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
        #[derive(CodamaInstruction)]
        struct Initialize;
    };
    let mut korok = StructKorok::parse(&item)?;
    korok.node = Some(BooleanTypeNode::default().into());

    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(korok.node, Some(BooleanTypeNode::default().into()));
    Ok(())
}

#[test]
fn with_name_directive() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(name = "initialize")]
        struct InitializeInstruction;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "initialize".into(),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_discriminator_directives() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(discriminator(size = 100))]
        #[codama(discriminator(field = "discriminator"))]
        struct MyInstruction;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "myInstruction".into(),
                discriminators: vec![
                    SizeDiscriminatorNode::new(100).into(),
                    FieldDiscriminatorNode::new("discriminator", 0).into(),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_argument_attributes_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(argument("age", number(u8)))]
        #[codama(argument("name", string(utf8)))]
        struct MyInstruction;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "myInstruction".into(),
                arguments: vec![
                    InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
                    InstructionArgumentNode::new("name", StringTypeNode::utf8()),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_prepended_argument_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(argument("discriminator", number(u8), default_value = 0, default_value_omitted))]
        #[codama(argument("name", string(utf8)))]
        struct MyInstruction {
            age: u8,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "myInstruction".into(),
                arguments: vec![
                    InstructionArgumentNode {
                        default_value: Some(NumberValueNode::new(0u8).into()),
                        default_value_strategy: Some(DefaultValueStrategy::Omitted),
                        ..InstructionArgumentNode::new("discriminator", NumberTypeNode::le(U8))
                    },
                    InstructionArgumentNode::new("name", StringTypeNode::utf8()),
                    InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_appended_argument_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(argument(after, "name", string(utf8)))]
        #[codama(argument(after, "is_member", boolean))]
        struct MyInstruction {
            age: u8,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "myInstruction".into(),
                arguments: vec![
                    InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
                    InstructionArgumentNode::new("name", StringTypeNode::utf8()),
                    InstructionArgumentNode::new("is_member", BooleanTypeNode::default()),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_prepended_and_appended_argument_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field(after, "first_name", string(utf8)))]
        #[codama(field("year_of_birth", number(u8)))]
        #[codama(field(after, "last_name", string(utf8)))]
        #[codama(field("is_member", boolean))]
        struct MyInstruction {
            age: u8,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "myInstruction".into(),
                arguments: vec![
                    InstructionArgumentNode::new("year_of_birth", NumberTypeNode::le(U8)),
                    InstructionArgumentNode::new("is_member", BooleanTypeNode::default()),
                    InstructionArgumentNode::new("age", NumberTypeNode::le(U8)),
                    InstructionArgumentNode::new("first_name", StringTypeNode::utf8()),
                    InstructionArgumentNode::new("last_name", StringTypeNode::utf8()),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}
