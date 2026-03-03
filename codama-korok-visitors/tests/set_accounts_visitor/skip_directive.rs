use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetAccountsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::{
    AccountNode, DefaultValueStrategy, Docs, FieldDiscriminatorNode,
    NumberFormat::{U64, U8},
    NumberTypeNode, NumberValueNode, ProgramNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn skip_variant_in_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            Mint {
                supply: u64,
            },
            #[codama(skip)]
            InternalState {},
            Token {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

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
                            StructFieldTypeNode::new("supply", NumberTypeNode::le(U64)),
                        ])
                        .into(),
                        pda: None,
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
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
                                default_value: Some(NumberValueNode::new(2u8).into()),
                            },
                            StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                        ])
                        .into(),
                        pda: None,
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
fn skip_variant_with_explicit_discriminator() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        #[repr(C, u8)]
        enum MyProgramAccounts {
            Mint {} = 0,
            Token {} = 1,
            #[codama(skip)]
            InternalState {} = 228,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

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
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    AccountNode {
                        name: "token".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(1u8).into()),
                        }])
                        .into(),
                        pda: None,
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
fn skip_preserves_sibling_discriminator_counting() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            First,
            #[codama(skip)]
            Second,
            Third,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetAccountsVisitor::new())?;

    // Third should have discriminator value 2 (not 1), because
    // Second still occupies slot 1 even though it's skipped.
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![
                    AccountNode {
                        name: "first".into(),
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
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    AccountNode {
                        name: "third".into(),
                        size: None,
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(2u8).into()),
                        }])
                        .into(),
                        pda: None,
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
fn skip_all_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaAccounts)]
        enum MyProgramAccounts {
            #[codama(skip)]
            First,
            #[codama(skip)]
            Second,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetAccountsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                accounts: vec![],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
