use crate::helpers::{get_data_fields, path};
use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyDisplayVisitor, IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor,
};
use codama_koroks::{EnumKorok, StructKorok};
use codama_nodes::{
    AmountNumberDisplayNode, DisplaySkip, FieldDiscriminatorNode, InstructionDisplayNode,
    InstructionNode, IntegerFormat::U8, IntegerTypeNode, IntegerValueNode, NumberDisplayNode,
    ProgramNode, StringValueNode, StructFieldDisplayNode, StructFieldTypeNode, StructTypeNode,
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
                identifier: "transfer".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![StructFieldTypeNode {
                        display: Some(StructFieldDisplayNode::new("Amount")),
                        r#type: Box::new(
                            IntegerTypeNode {
                                display: Box::new(Some(NumberDisplayNode::Amount(
                                    AmountNumberDisplayNode::new(
                                        IntegerValueNode::new(9u64),
                                        StringValueNode::new("SOL"),
                                    ),
                                ))),
                                ..IntegerTypeNode::le(codama_nodes::U64)
                            }
                            .into()
                        ),
                        ..StructFieldTypeNode::new("amount", IntegerTypeNode::le(codama_nodes::U64))
                    }])
                    .into()
                )),
                display: Some(InstructionDisplayNode {
                    intent: Some("Transfer SOL".into()),
                    interpolated_intent: Some("Transfer ${data.amount} SOL".into()),
                    plugins: vec![],
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
                        identifier: "withdraw".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    default_value_strategy: Some(
                                        codama_nodes::DefaultValueStrategy::Omitted,
                                    ),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(0u64).into()
                                    )),
                                    display: skipped.clone(),
                                    ..StructFieldTypeNode::new(
                                        "discriminator",
                                        IntegerTypeNode::le(U8),
                                    )
                                },
                                StructFieldTypeNode::new(
                                    "amount",
                                    IntegerTypeNode::le(codama_nodes::U64),
                                ),
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into(),
                        ],
                        display: Some(InstructionDisplayNode {
                            intent: Some("Withdraw stake".into()),
                            interpolated_intent: Some("Withdraw ${data.amount}".into()),
                            plugins: vec![],
                        }),
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: "deposit".into(),
                        data: Box::new(
                            Some(
                                StructTypeNode::new(vec![StructFieldTypeNode {
                                    default_value_strategy: Some(
                                        codama_nodes::DefaultValueStrategy::Omitted,
                                    ),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(1u64).into()
                                    )),
                                    display: skipped,
                                    ..StructFieldTypeNode::new(
                                        "discriminator",
                                        IntegerTypeNode::le(U8),
                                    )
                                }])
                                .into()
                            )
                        ),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into(),
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
fn it_preserves_tuple_field_displays() -> CodamaResult<()> {
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
        get_data_fields(&program.instructions[0])[1].display,
        Some(StructFieldDisplayNode {
            label: Some("Amount".into()),
            skip: Some(DisplaySkip::WhenInjected),
            flatten: Some(true),
            flatten_prefix: None,
            value_labels: vec![],
            plugins: vec![],
        })
    );
    Ok(())
}
