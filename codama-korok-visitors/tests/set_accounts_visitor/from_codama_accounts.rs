use codama_errors::CodamaResult;
use codama_korok_visitors::{KorokVisitable, SetAccountsVisitor, SetBorshTypesVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountNode, BooleanTypeNode, DefaultValueStrategy, Docs, FieldDiscriminatorNode,
    NumberFormat::{U32, U64, U8},
    NumberTypeNode, NumberValueNode, OptionTypeNode, ProgramNode, PublicKeyTypeNode,
    StructFieldTypeNode, StructTypeNode,
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
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
    korok.accept(&mut SetBorshTypesVisitor::new())?;
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
fn with_custom_enum_size() -> CodamaResult<()> {
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
