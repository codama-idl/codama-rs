use crate::helpers::path;
use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetInstructionsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefaultValueStrategy, DisplaySkip, Docs, FieldDiscriminatorNode, InstructionNode,
    IntegerFormat::{U64, U8},
    IntegerTypeNode, IntegerValueNode, ProgramNode, StructFieldDisplayNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn skip_variant_in_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            Initialize {
                amount: u64,
            },
            #[codama(skip)]
            EmitEvent {},
            Update {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: "initialize".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: "discriminator".into(),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(0u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: "update".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![
                                StructFieldTypeNode {
                                    identifier: "discriminator".into(),
                                    default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                    docs: Docs::default(),
                                    r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                    default_value: Box::new(Some(
                                        IntegerValueNode::new(2u8).into()
                                    )),
                                    display: Some(StructFieldDisplayNode::skipped(
                                        DisplaySkip::Always
                                    )),
                                    plugins: vec![],
                                },
                                StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64))
                            ])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        #[repr(C, u8)]
        enum MyProgramInstructions {
            Initialize {} = 0,
            Update {} = 1,
            #[codama(skip)]
            #[codama(account(name = "event_authority", signer))]
            EmitEvent {} = 228,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: "initialize".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: "update".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(1u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            First,
            #[codama(skip)]
            Second,
            Third,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetInstructionsVisitor::new())?;

    // Third should have discriminator value 2 (not 1), because
    // Second still occupies slot 1 even though it's skipped.
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![
                    InstructionNode {
                        identifier: "first".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(0u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
                    },
                    InstructionNode {
                        identifier: "third".into(),
                        data: Box::new(Some(
                            StructTypeNode::new(vec![StructFieldTypeNode {
                                identifier: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: Box::new(IntegerTypeNode::le(U8).into()),
                                default_value: Box::new(Some(IntegerValueNode::new(2u8).into())),
                                display: Some(StructFieldDisplayNode::skipped(DisplaySkip::Always)),
                                plugins: vec![],
                            }])
                            .into()
                        )),
                        discriminators: vec![
                            FieldDiscriminatorNode::new(path("discriminator"), 0).into()
                        ],
                        ..InstructionNode::default()
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
        #[derive(CodamaInstructions)]
        enum MyProgramInstructions {
            #[codama(skip)]
            First,
            #[codama(skip)]
            Second,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetInstructionsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                instructions: vec![],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
