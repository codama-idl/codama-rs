use crate::helpers::get_data_fields;
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor,
};
use codama_koroks::{ItemKorok, KorokTrait, StructKorok};
use codama_nodes::{
    AccountBumpValueNode, AccountValueNode, DefaultValueStrategy, InjectedValueNode,
    InstructionAccountNode, InstructionNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, Node, PayerValueNode, PdaLinkNode, PdaSeedValueNode, PdaValueNode,
    ProvidedNode, PublicKeyTypeNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_injects_contextual_default_values() -> CodamaResult<()> {
    for (item, expected_field, expected_provided) in [
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                struct Initialize {
                    #[codama(default_value = payer)]
                    authority: Pubkey,
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("authority").into())),
                ..StructFieldTypeNode::new("authority", PublicKeyTypeNode::new())
            },
            ProvidedNode::new("authority", PayerValueNode::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                struct Initialize {
                    #[codama(value = payer)]
                    authority: Pubkey,
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("authority").into())),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                ..StructFieldTypeNode::new("authority", PublicKeyTypeNode::new())
            },
            ProvidedNode::new("authority", PayerValueNode::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                struct Initialize {
                    #[codama(account(writable))]
                    vault: AccountMeta,
                    #[codama(default_value = account_bump("vault"))]
                    bump: u8,
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("bump").into())),
                ..StructFieldTypeNode::new("bump", IntegerTypeNode::le(U8))
            },
            ProvidedNode::new("bump", AccountBumpValueNode::new("vault")),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstruction)]
                struct Initialize {
                    #[codama(default_value = pda("config", [account("auth")]))]
                    config: Pubkey,
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("config").into())),
                ..StructFieldTypeNode::new("config", PublicKeyTypeNode::new())
            },
            ProvidedNode::new(
                "config",
                PdaValueNode::new(
                    PdaLinkNode::new("config"),
                    vec![PdaSeedValueNode::new("auth", AccountValueNode::new("auth"))],
                ),
            ),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    V(#[codama(default_value = payer)] Pubkey),
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("arg0").into())),
                ..StructFieldTypeNode::new("arg0", PublicKeyTypeNode::new())
            },
            ProvidedNode::new("arg0", PayerValueNode::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    V(
                        #[codama(account(name = "auth", signer))]
                        #[codama(default_value = payer)]
                        Pubkey,
                    ),
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("arg0").into())),
                ..StructFieldTypeNode::new("arg0", PublicKeyTypeNode::new())
            },
            ProvidedNode::new("arg0", PayerValueNode::new()),
        ),
    ] {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        korok.accept(&mut SetInstructionsVisitor::new())?;
        let instruction = match korok.node() {
            Some(Node::Instruction(instruction)) => instruction,
            Some(Node::Program(program)) => &program.instructions[0],
            _ => panic!("Expected an instruction node"),
        };
        assert_eq!(get_data_fields(instruction).last(), Some(&expected_field));
        assert_eq!(instruction.provides, vec![expected_provided]);
    }
    Ok(())
}

#[test]
fn it_matches_default_values_by_identifier() -> CodamaResult<()> {
    for (item, expected_fields, expected_provides) in [
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    #[codama(type = variant("transfer", struct(field("amount", integer(u64)))))]
                    Transfer {
                        #[codama(default_value = payer)]
                        owner: Pubkey,
                    },
                }
            },
            vec![StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64))],
            vec![],
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    #[codama(type = variant("transfer", struct(
                        field("owner", public_key),
                        field("amount", integer(u64)),
                    )))]
                    Transfer {
                        amount: u64,
                        #[codama(default_value = payer)]
                        owner: Pubkey,
                    },
                }
            },
            vec![
                StructFieldTypeNode {
                    default_value: Box::new(Some(InjectedValueNode::new("owner").into())),
                    ..StructFieldTypeNode::new("owner", PublicKeyTypeNode::new())
                },
                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
            ],
            vec![ProvidedNode::new("owner", PayerValueNode::new())],
        ),
    ] {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
        korok.accept(&mut SetInstructionsVisitor::new())?;
        let Some(Node::Program(program)) = korok.node() else {
            panic!("Expected a program node");
        };
        let instruction = &program.instructions[0];
        assert_eq!(
            &get_data_fields(instruction)[1..],
            expected_fields.as_slice()
        );
        assert_eq!(instruction.provides, expected_provides);
    }
    Ok(())
}

#[test]
fn it_rejects_default_values_on_account_fields() -> CodamaResult<()> {
    for item in [
        syn::parse_quote! {
            #[derive(CodamaInstruction)]
            #[codama(field(after, "authority", public_key))]
            struct SetAuthority {
                #[codama(account(signer))]
                #[codama(default_value = payer)]
                authority: AccountMeta,
                amount: u64,
            }
        },
        syn::parse_quote! {
            #[derive(CodamaInstruction)]
            struct Deposit {
                #[codama(account(writable))]
                #[codama(value = 42)]
                vault: AccountMeta,
            }
        },
    ] {
        let mut korok = StructKorok::parse(&item)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        let error = korok
            .accept(&mut SetInstructionsVisitor::new())
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "A default value has no effect on an account field, use `account(..., default_value = ...)`"
        );
    }
    Ok(())
}

#[test]
fn it_separates_account_and_data_fields_with_one_name() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field(after, "authority", public_key))]
        struct SetAuthority {
            #[codama(account(signer))]
            authority: AccountMeta,
            amount: u64,
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
                identifier: "setAuthority".into(),
                accounts: vec![InstructionAccountNode::new("authority", false, true)],
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode::new("authority", PublicKeyTypeNode::new()),
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
