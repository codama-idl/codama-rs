use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyDisplayVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AmountNumberDisplayNode, DisplaySkip, FieldDiscriminatorNode, InstructionArgumentNode,
    InstructionDisplayNode, InstructionNode, NumberDisplayNode, NumberFormat::U8, NumberTypeNode,
    NumberValueNode, ProgramNode, StringValueNode, StructFieldDisplayNode,
};

#[test]
fn it_applies_displays_to_instruction_structs() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstruction)]
        #[codama(display(intent = "Transfer tokens"))]
        #[codama(display(
            intent = "Transfer SOL",
            interpolated_intent = "Transfer ${data.amount} SOL"
        ))]
        struct Transfer {
            #[codama(display(
                label = "Amount",
                amount(decimals = 9, unit = "SOL")
            ))]
            amount: u64,
        }
    };
    let mut korok = StructKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            InstructionNode {
                name: "transfer".into(),
                arguments: vec![InstructionArgumentNode {
                    display: Some(StructFieldDisplayNode::new("Amount")),
                    r#type: Box::new(
                        NumberTypeNode {
                            display: Box::new(Some(NumberDisplayNode::Amount(
                                AmountNumberDisplayNode::new(
                                    NumberValueNode::new(9u64),
                                    StringValueNode::new("SOL"),
                                ),
                            ))),
                            ..NumberTypeNode::le(codama_nodes::U64)
                        }
                        .into()
                    ),
                    ..InstructionArgumentNode::new("amount", NumberTypeNode::le(codama_nodes::U64))
                }],
                display: Some(InstructionDisplayNode {
                    intent: Some("Transfer SOL".to_string()),
                    interpolated_intent: Some("Transfer ${data.amount} SOL".to_string()),
                }),
                ..InstructionNode::default()
            }
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_applies_displays_to_instruction_enum_variants() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum Instructions {
            #[codama(display(
                intent = "Withdraw stake",
                interpolated_intent = "Withdraw ${data.amount}"
            ))]
            Withdraw { amount: u64 },
            Deposit,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let skipped = Some(StructFieldDisplayNode::skipped(DisplaySkip::Always));
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        name: "withdraw".into(),
                        arguments: vec![
                            InstructionArgumentNode {
                                default_value_strategy: Some(
                                    codama_nodes::DefaultValueStrategy::Omitted,
                                ),
                                default_value: Box::new(Some(NumberValueNode::new(0u64).into())),
                                display: skipped.clone(),
                                ..InstructionArgumentNode::new(
                                    "discriminator",
                                    NumberTypeNode::le(U8),
                                )
                            },
                            InstructionArgumentNode::new(
                                "amount",
                                NumberTypeNode::le(codama_nodes::U64),
                            ),
                        ],
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into(),
                        ],
                        display: Some(InstructionDisplayNode {
                            intent: Some("Withdraw stake".to_string()),
                            interpolated_intent: Some("Withdraw ${data.amount}".to_string()),
                        }),
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        name: "deposit".into(),
                        arguments: vec![InstructionArgumentNode {
                            default_value_strategy: Some(
                                codama_nodes::DefaultValueStrategy::Omitted,
                            ),
                            default_value: Box::new(Some(NumberValueNode::new(1u64).into())),
                            display: skipped,
                            ..InstructionArgumentNode::new("discriminator", NumberTypeNode::le(U8),)
                        }],
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into(),
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
fn it_preserves_tuple_argument_displays() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum Instructions {
            Withdraw(
                #[codama(name = "amount")]
                #[codama(display(label = "First", skip = when_injected))]
                #[codama(display(label = "Amount", flatten))]
                u64,
            ),
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;

    let Some(codama_nodes::Node::Program(program)) = korok.node else {
        panic!("Expected a program node");
    };
    assert_eq!(
        program.instructions[0].arguments[1].display,
        Some(StructFieldDisplayNode {
            label: Some("Amount".to_string()),
            skip: Some(DisplaySkip::WhenInjected),
            flatten: Some(true),
            flatten_prefix: None,
        })
    );
    Ok(())
}
