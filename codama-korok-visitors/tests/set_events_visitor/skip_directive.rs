use codama_errors::CodamaResult;
use codama_korok_visitors::{IdentifyFieldTypesVisitor, KorokVisitable, SetEventsVisitor};
use codama_koroks::EnumKorok;
use codama_nodes::{
    DefaultValueStrategy, Docs, EventNode, FieldDiscriminatorNode,
    NumberFormat::{U64, U8},
    NumberTypeNode, NumberValueNode, ProgramNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn skip_variant_in_enum() -> CodamaResult<()> {
    let item: syn::Item = syn::parse_quote! {
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            Transfer {
                amount: u64,
            },
            #[codama(skip)]
            InternalEvent {},
            Burn {
                amount: u64,
            },
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "transfer".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![
                            StructFieldTypeNode {
                                name: "discriminator".into(),
                                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                                docs: Docs::default(),
                                r#type: NumberTypeNode::le(U8).into(),
                                default_value: Some(NumberValueNode::new(0u8).into()),
                            },
                            StructFieldTypeNode::new("amount", NumberTypeNode::le(U64)),
                        ])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "burn".into(),
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
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            First,
            #[codama(skip)]
            Second,
            Third,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetEventsVisitor::new())?;

    // Third should have discriminator value 2 (not 1), because
    // Second still occupies slot 1 even though it's skipped.
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![
                    EventNode {
                        name: "first".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(0u8).into()),
                        }])
                        .into(),
                        discriminators: vec![
                            FieldDiscriminatorNode::new("discriminator", 0).into()
                        ],
                    },
                    EventNode {
                        name: "third".into(),
                        docs: Docs::default(),
                        data: StructTypeNode::new(vec![StructFieldTypeNode {
                            name: "discriminator".into(),
                            default_value_strategy: Some(DefaultValueStrategy::Omitted),
                            docs: Docs::default(),
                            r#type: NumberTypeNode::le(U8).into(),
                            default_value: Some(NumberValueNode::new(2u8).into()),
                        }])
                        .into(),
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
        #[derive(CodamaEvents)]
        enum MyProgramEvents {
            #[codama(skip)]
            First,
            #[codama(skip)]
            Second,
        }
    };
    let mut korok = EnumKorok::parse(&item)?;

    korok.accept(&mut SetEventsVisitor::new())?;
    assert_eq!(
        korok.node,
        Some(
            ProgramNode {
                events: vec![],
                ..ProgramNode::default()
            }
            .into()
        )
    );
    Ok(())
}
