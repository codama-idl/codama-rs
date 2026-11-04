use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor,
};
use codama_koroks::{ItemKorok, KorokTrait, StructKorok};
use codama_nodes::test_utils::{get_data_fields, id};
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
                default_value: Box::new(Some(InjectedValueNode::new(id("authority")).into())),
                ..StructFieldTypeNode::new(id("authority"), PublicKeyTypeNode::new())
            },
            ProvidedNode::new(id("authority"), PayerValueNode::new()),
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
                default_value: Box::new(Some(InjectedValueNode::new(id("authority")).into())),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                ..StructFieldTypeNode::new(id("authority"), PublicKeyTypeNode::new())
            },
            ProvidedNode::new(id("authority"), PayerValueNode::new()),
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
                default_value: Box::new(Some(InjectedValueNode::new(id("bump")).into())),
                ..StructFieldTypeNode::new(id("bump"), IntegerTypeNode::le(U8))
            },
            ProvidedNode::new(id("bump"), AccountBumpValueNode::new(id("vault"))),
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
                default_value: Box::new(Some(InjectedValueNode::new(id("config")).into())),
                ..StructFieldTypeNode::new(id("config"), PublicKeyTypeNode::new())
            },
            ProvidedNode::new(
                id("config"),
                PdaValueNode::new(
                    PdaLinkNode::new(id("config")),
                    vec![PdaSeedValueNode::new(
                        id("auth"),
                        AccountValueNode::new(id("auth")),
                    )],
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
                default_value: Box::new(Some(InjectedValueNode::new(id("arg0")).into())),
                ..StructFieldTypeNode::new(id("arg0"), PublicKeyTypeNode::new())
            },
            ProvidedNode::new(id("arg0"), PayerValueNode::new()),
        ),
        (
            syn::parse_quote! {
                #[derive(CodamaInstructions)]
                enum Instructions {
                    V(
                        #[codama(account(identifier = "auth", signer))]
                        #[codama(default_value = payer)]
                        Pubkey,
                    ),
                }
            },
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new(id("arg0")).into())),
                ..StructFieldTypeNode::new(id("arg0"), PublicKeyTypeNode::new())
            },
            ProvidedNode::new(id("arg0"), PayerValueNode::new()),
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
fn it_injects_contextual_default_values_of_renamed_fields() -> CodamaResult<()> {
    let expected_field = StructFieldTypeNode {
        default_value: Box::new(Some(InjectedValueNode::new(id("authority")).into())),
        ..StructFieldTypeNode::new(id("authority"), PublicKeyTypeNode::new())
    };
    let expected_provided = ProvidedNode::new(id("authority"), PayerValueNode::new());
    for item in [
        syn::parse_quote! {
            #[derive(CodamaInstruction)]
            struct Initialize {
                #[codama(type = field("authority", public_key))]
                #[codama(default_value = payer)]
                owner: Pubkey,
            }
        },
        syn::parse_quote! {
            #[derive(CodamaInstructions)]
            enum Instructions {
                Initialize {
                    #[codama(type = field("authority", public_key))]
                    #[codama(default_value = payer)]
                    owner: Pubkey,
                },
            }
        },
    ] {
        let mut korok = ItemKorok::parse(&item, &[], &mut 0)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
        korok.accept(&mut SetInstructionsVisitor::new())?;
        let instruction = match korok.node() {
            Some(Node::Instruction(instruction)) => instruction,
            Some(Node::Program(program)) => &program.instructions[0],
            _ => panic!("Expected an instruction node"),
        };
        assert_eq!(get_data_fields(instruction).last(), Some(&expected_field));
        assert_eq!(instruction.provides, vec![expected_provided.clone()]);
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
            vec![StructFieldTypeNode::new(
                id("amount"),
                IntegerTypeNode::le(U64),
            )],
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
                    default_value: Box::new(Some(InjectedValueNode::new(id("owner")).into())),
                    ..StructFieldTypeNode::new(id("owner"), PublicKeyTypeNode::new())
                },
                StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64)),
            ],
            vec![ProvidedNode::new(id("owner"), PayerValueNode::new())],
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
                identifier: id("set_authority"),
                accounts: vec![InstructionAccountNode::new(id("authority"), false, true)],
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode::new(id("amount"), IntegerTypeNode::le(U64)),
                        StructFieldTypeNode::new(id("authority"), PublicKeyTypeNode::new()),
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
