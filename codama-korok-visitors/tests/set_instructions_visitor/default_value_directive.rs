use crate::helpers::get_data_fields;
use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
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
fn it_takes_no_data_default_from_account_fields() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field(after, "authority", public_key))]
        struct SetAuthority {
            #[codama(account(signer))]
            #[codama(default_value = payer)]
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

#[test]
fn it_provides_one_value_per_identifier() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        struct Transfer {
            #[codama(name = "owner")]
            #[codama(default_value = payer)]
            source_owner: Pubkey,
            #[codama(name = "owner")]
            #[codama(default_value = payer)]
            destination_owner: Pubkey,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    assert_eq!(korok.node, None);
    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    let owner = StructFieldTypeNode {
        default_value: Box::new(Some(InjectedValueNode::new("owner").into())),
        ..StructFieldTypeNode::new("owner", PublicKeyTypeNode::new())
    };
    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                identifier: "transfer".into(),
                data: Box::new(Some(StructTypeNode::new(vec![owner.clone(), owner]).into())),
                provides: vec![ProvidedNode::new("owner", PayerValueNode::new())],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}
