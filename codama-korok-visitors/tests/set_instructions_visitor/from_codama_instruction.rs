use crate::helpers::path;
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    IdentifyFieldTypesVisitor, KorokVisitable, SetDefaultValuesVisitor, SetInstructionsVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountValueNode, BooleanTypeNode, BooleanValueNode, DataValueNode, DefaultValueStrategy, Docs,
    FieldDiscriminatorNode, InjectedValueNode, InstructionAccountNode, InstructionNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, IntegerValueNode, PayerValueNode, ProvidedNode, PublicKeyValueNode,
    SizeDiscriminatorNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
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
                identifier: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode::new("is_canonical", BooleanTypeNode::default(),),
                    ])
                    .into()
                )),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn from_struct_with_data_only() -> CodamaResult<()> {
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
                identifier: "initialize".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode::new("is_canonical", BooleanTypeNode::default(),),
                    ])
                    .into()
                )),
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
                identifier: "initialize".into(),
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
                identifier: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode::new("payer", true, true),
                ],
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode::new("is_canonical", BooleanTypeNode::default(),),
                    ])
                    .into()
                )),
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
                identifier: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode {
                        identifier: "rentSysvar".into(),
                        default_value: Box::new(Some(
                            PublicKeyValueNode::new("SysvarRent111111111111111111111111111111111")
                                .into(),
                        )),
                        is_writable: false,
                        is_signer: false.into(),
                        is_optional: None,
                        docs: Docs::default(),
                        account_link: None,
                        display: None,
                        plugins: vec![],
                    },
                    InstructionAccountNode {
                        identifier: "tokenProgram".into(),
                        default_value: Box::new(Some(
                            PublicKeyValueNode::new("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
                                .into(),
                        )),
                        is_writable: false,
                        is_signer: false.into(),
                        is_optional: None,
                        docs: Docs::default(),
                        account_link: None,
                        display: None,
                        plugins: vec![],
                    },
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode {
                        identifier: "payer".into(),
                        is_signer: true.into(),
                        is_writable: true,
                        default_value: Box::new(Some(PayerValueNode::new().into())),
                        is_optional: None,
                        docs: Docs::default(),
                        account_link: None,
                        display: None,
                        plugins: vec![],
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
fn from_struct_with_default_values_in_data() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field("discriminator", integer(u8), default_value = 0))]
        struct Initialize {
            capacity: u64,
            #[codama(default_value = data("capacity"))]
            max_capacity: u64,
            #[codama(value = false)]
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
                identifier: "initialize".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode {
                            default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                            ..StructFieldTypeNode::new("discriminator", IntegerTypeNode::le(U8))
                        },
                        StructFieldTypeNode::new("capacity", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode {
                            default_value: Box::new(Some(
                                InjectedValueNode::new("maxCapacity").into()
                            )),
                            ..StructFieldTypeNode::new("max_capacity", IntegerTypeNode::le(U64))
                        },
                        StructFieldTypeNode {
                            default_value: Box::new(Some(BooleanValueNode::new(false).into())),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            ..StructFieldTypeNode::new("with_metadata", BooleanTypeNode::default())
                        }
                    ])
                    .into()
                )),
                provides: vec![ProvidedNode::new(
                    "maxCapacity",
                    DataValueNode::new(path("capacity")),
                )],
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
                identifier: "initialize".into(),
                accounts: vec![
                    InstructionAccountNode::new("authority", false, false),
                    InstructionAccountNode {
                        default_value: Box::new(Some(AccountValueNode::new("authority").into())),
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
fn from_struct_with_default_values_linking_to_other_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Initialize {
            capacity: u64,
            #[codama(default_value = data("capacity"))]
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
                identifier: "initialize".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("capacity", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode {
                            default_value: Box::new(Some(
                                InjectedValueNode::new("maxCapacity").into()
                            )),
                            ..StructFieldTypeNode::new("max_capacity", IntegerTypeNode::le(U64))
                        },
                    ])
                    .into()
                )),
                provides: vec![ProvidedNode::new(
                    "maxCapacity",
                    DataValueNode::new(path("capacity")),
                )],
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
                identifier: "initialize".into(),
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
                identifier: "initialize".into(),
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
                identifier: "myInstruction".into(),
                discriminators: vec![
                    SizeDiscriminatorNode::new(100).into(),
                    FieldDiscriminatorNode::new(path("discriminator"), 0).into(),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_field_attributes_only() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field("age", integer(u8)))]
        #[codama(field("name", string(utf8)))]
        struct MyInstruction;
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                identifier: "myInstruction".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                        StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    ])
                    .into()
                )),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_prepended_field_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field("discriminator", integer(u8), value = 0))]
        #[codama(field("name", string(utf8)))]
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
                identifier: "myInstruction".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode {
                            default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            ..StructFieldTypeNode::new("discriminator", IntegerTypeNode::le(U8))
                        },
                        StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                        StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                    ])
                    .into()
                )),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_appended_field_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field(after, "name", string(utf8)))]
        #[codama(field(after, "is_member", boolean))]
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
                identifier: "myInstruction".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                        StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                        StructFieldTypeNode::new("is_member", BooleanTypeNode::default()),
                    ])
                    .into()
                )),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn with_prepended_and_appended_field_attributes() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field(after, "first_name", string(utf8)))]
        #[codama(field("year_of_birth", integer(u8)))]
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
                identifier: "myInstruction".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("year_of_birth", IntegerTypeNode::le(U8)),
                        StructFieldTypeNode::new("is_member", BooleanTypeNode::default()),
                        StructFieldTypeNode::new("age", IntegerTypeNode::le(U8)),
                        StructFieldTypeNode::new("first_name", StringTypeNode::utf8()),
                        StructFieldTypeNode::new("last_name", StringTypeNode::utf8()),
                    ])
                    .into()
                )),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}
