use crate::helpers::get_data_fields;
use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AccountBumpValueNode, DefaultValueStrategy, InjectedValueNode, InstructionAccountNode,
    InstructionNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, Node, PayerValueNode, ProvidedNode, PublicKeyTypeNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn it_injects_contextual_default_values() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(field("owner", public_key, default_value = payer))]
        #[codama(field(after, "bump", integer(u8), value = account_bump("vault")))]
        struct Initialize {
            #[codama(account(writable))]
            vault: AccountMeta,
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
                identifier: "initialize".into(),
                accounts: vec![InstructionAccountNode::new("vault", true, false)],
                data: Box::new(Some(
                    StructTypeNode::new(vec![
                        StructFieldTypeNode {
                            default_value: Box::new(Some(InjectedValueNode::new("owner").into())),
                            ..StructFieldTypeNode::new("owner", PublicKeyTypeNode::new())
                        },
                        StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
                        StructFieldTypeNode {
                            default_value: Box::new(Some(InjectedValueNode::new("bump").into())),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            ..StructFieldTypeNode::new("bump", IntegerTypeNode::le(U8))
                        },
                    ])
                    .into()
                )),
                provides: vec![
                    ProvidedNode::new("owner", PayerValueNode::new()),
                    ProvidedNode::new("bump", AccountBumpValueNode::new("vault")),
                ],
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_injects_contextual_default_values_in_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum Instructions {
            #[codama(field("owner", public_key, default_value = payer))]
            Initialize { amount: u64 },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    let Some(Node::Program(program)) = &korok.node else {
        panic!("Expected a program node");
    };
    let instruction = &program.instructions[0];
    assert_eq!(
        get_data_fields(instruction)[1..],
        [
            StructFieldTypeNode {
                default_value: Box::new(Some(InjectedValueNode::new("owner").into())),
                ..StructFieldTypeNode::new("owner", PublicKeyTypeNode::new())
            },
            StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64)),
        ]
    );
    assert_eq!(
        instruction.provides,
        vec![ProvidedNode::new("owner", PayerValueNode::new())]
    );
    Ok(())
}
