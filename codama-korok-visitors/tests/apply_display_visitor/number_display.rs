use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyDisplayVisitor, ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable,
};
use codama_koroks::{FieldKorok, KorokTrait};
use codama_nodes::{
    AmountNumberDisplayNode, ArrayTypeNode, BooleanTypeNode, ConstantValueNode, DateTimeTypeNode,
    DefinedTypeLinkNode, DurationTypeNode, FixedSizeTransformNode, HasTransforms,
    HiddenPrefixTransformNode, HiddenSuffixTransformNode,
    IntegerFormat::{U32, U64, U8},
    IntegerTypeNode, IntegerValueNode, MapTypeNode, NumberDisplayNode, OptionTypeNode,
    PostOffsetTransformNode, PreOffsetTransformNode, RemainderOptionTypeNode,
    SentinelTransformNode, SetTypeNode, SizePrefixTransformNode, StringTypeNode, StringValueNode,
    StructFieldTypeNode, TupleTypeNode, TypeNode, ZeroableOptionTypeNode,
};

#[test]
fn it_applies_number_displays_without_creating_field_displays() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        amount: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "amount",
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                },
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_replaces_existing_number_displays() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = integer(u64, display = amount(decimals = 6, unit = "USDC")))]
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        amount: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "amount",
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                },
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_applies_multiple_number_displays_in_source_order() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 6, unit = "USDC")))]
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        amount: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "amount",
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                },
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_traverses_supported_number_wrappers() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        values: Vec<Option<u64>>
    };
    let mut korok = FieldKorok::parse(&ast)?;
    let display = NumberDisplayNode::Amount(AmountNumberDisplayNode::new(
        IntegerValueNode::new(9u64),
        StringValueNode::new("SOL"),
    ));
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "values",
            OptionTypeNode::new(ArrayTypeNode::fixed(IntegerTypeNode::le(U64), 2))
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                .add_transform(FixedSizeTransformNode::new(42)),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "values",
                OptionTypeNode::new(ArrayTypeNode::fixed(
                    IntegerTypeNode {
                        display: Box::new(Some(display)),
                        ..IntegerTypeNode::le(U64)
                    },
                    2,
                ))
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
                .add_transform(FixedSizeTransformNode::new(42)),
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_traverses_nodes_with_transforms() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;
    let hidden_prefix = ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(1u8));
    let hidden_suffix = ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(2u8));
    let sentinel = ConstantValueNode::new(IntegerTypeNode::le(U8), IntegerValueNode::new(255u8));
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            RemainderOptionTypeNode::new(IntegerTypeNode::le(U64))
                .add_transform(SentinelTransformNode::new(sentinel.clone()))
                .add_transform(HiddenSuffixTransformNode::new(vec![hidden_suffix.clone()]))
                .add_transform(HiddenPrefixTransformNode::new(vec![hidden_prefix.clone()]))
                .add_transform(PostOffsetTransformNode::padded(8))
                .add_transform(PreOffsetTransformNode::relative(4)),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                RemainderOptionTypeNode::new(IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                })
                .add_transform(SentinelTransformNode::new(sentinel))
                .add_transform(HiddenSuffixTransformNode::new(vec![hidden_suffix]))
                .add_transform(HiddenPrefixTransformNode::new(vec![hidden_prefix]))
                .add_transform(PostOffsetTransformNode::padded(8))
                .add_transform(PreOffsetTransformNode::relative(4)),
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_traverses_alternative_option_encodings() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let display = NumberDisplayNode::Amount(AmountNumberDisplayNode::new(
        IntegerValueNode::new(9u64),
        StringValueNode::new("SOL"),
    ));

    let mut remainder_korok = FieldKorok::parse(&ast)?;
    remainder_korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            RemainderOptionTypeNode::new(IntegerTypeNode::le(U64)),
        )
        .into(),
    ));

    remainder_korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        remainder_korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                RemainderOptionTypeNode::new(IntegerTypeNode {
                    display: Box::new(Some(display.clone())),
                    ..IntegerTypeNode::le(U64)
                }),
            )
            .into(),
        )
    );

    let mut zeroable_korok = FieldKorok::parse(&ast)?;
    let zero_value = ConstantValueNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(0u64));
    zeroable_korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            ZeroableOptionTypeNode::custom(IntegerTypeNode::le(U64), zero_value.clone()),
        )
        .into(),
    ));

    zeroable_korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        zeroable_korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                ZeroableOptionTypeNode::custom(
                    IntegerTypeNode {
                        display: Box::new(Some(display)),
                        ..IntegerTypeNode::le(U64)
                    },
                    zero_value,
                ),
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_keeps_transforms_from_type_overrides() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = pre_offset(integer(u64), 4, relative))]
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;

    korok.accept(&mut ApplyTypeOverridesVisitor::new())?;
    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                }
                .add_transform(PreOffsetTransformNode::relative(4)),
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_reports_errors_at_the_logical_leaf() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let wrapped_types: Vec<TypeNode> = vec![
        StringTypeNode::utf8()
            .add_transform(PreOffsetTransformNode::relative(4))
            .into(),
        RemainderOptionTypeNode::new(StringTypeNode::utf8()).into(),
    ];

    for wrapped_type in wrapped_types {
        let mut korok = FieldKorok::parse(&ast)?;
        korok.set_node(Some(StructFieldTypeNode::new("value", wrapped_type).into()));

        let error = korok.accept(&mut ApplyDisplayVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `stringTypeNode`"
        );
    }
    Ok(())
}

#[test]
fn it_traverses_rust_arrays_and_vectors() -> CodamaResult<()> {
    let array_ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        amounts: [u64; 2]
    };
    let vector_ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        amounts: Vec<u64>
    };

    for ast in [&array_ast, &vector_ast] {
        let mut korok = FieldKorok::parse(ast)?;
        korok.accept(&mut IdentifyFieldTypesVisitor::new())?;
        korok.accept(&mut ApplyDisplayVisitor::new())?;

        let Some(codama_nodes::Node::Type(codama_nodes::RegisteredTypeNode::StructField(field))) =
            korok.node
        else {
            panic!("Expected a struct field type node");
        };
        let TypeNode::Array(array) = *field.r#type else {
            panic!("Expected an array type node");
        };
        let TypeNode::Integer(integer) = *array.item else {
            panic!("Expected an integer type node");
        };
        assert_eq!(
            *integer.display,
            Some(NumberDisplayNode::Amount(AmountNumberDisplayNode::new(
                IntegerValueNode::new(9u64),
                StringValueNode::new("SOL"),
            )))
        );
    }
    Ok(())
}

#[test]
fn it_does_not_traverse_composite_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let unsupported: Vec<(TypeNode, &str)> = vec![
        (BooleanTypeNode::default().into(), "booleanTypeNode"),
        (
            SetTypeNode::fixed(IntegerTypeNode::le(U64), 2).into(),
            "setTypeNode",
        ),
        (
            MapTypeNode::fixed(IntegerTypeNode::le(U64), IntegerTypeNode::le(U64), 2).into(),
            "mapTypeNode",
        ),
        (
            TupleTypeNode::new(vec![IntegerTypeNode::le(U64).into()]).into(),
            "tupleTypeNode",
        ),
        (
            DefinedTypeLinkNode::new("external_number").into(),
            "definedTypeLinkNode",
        ),
    ];

    for (type_node, kind) in unsupported {
        let mut korok = FieldKorok::parse(&ast)?;
        korok.set_node(Some(StructFieldTypeNode::new("value", type_node).into()));

        let error = korok.accept(&mut ApplyDisplayVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!(
                "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `{kind}`"
            )
        );
    }
    Ok(())
}

#[test]
fn it_does_not_traverse_semantic_number_types() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let unsupported: Vec<(TypeNode, &str)> = vec![
        (
            DateTimeTypeNode::new(IntegerTypeNode::le(U64)).into(),
            "dateTimeTypeNode",
        ),
        (
            DurationTypeNode::new(IntegerTypeNode::le(U64)).into(),
            "durationTypeNode",
        ),
    ];

    for (type_node, kind) in unsupported {
        let mut korok = FieldKorok::parse(&ast)?;
        korok.set_node(Some(StructFieldTypeNode::new("value", type_node).into()));

        let error = korok.accept(&mut ApplyDisplayVisitor::new()).unwrap_err();
        assert_eq!(
            error.to_string(),
            format!(
                "Cannot apply attribute `#[codama(display)]` as a number display on a node of kind `{kind}`"
            )
        );
    }
    Ok(())
}

#[test]
fn it_does_not_update_transform_metadata() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount(decimals = 9, unit = "SOL")))]
        value: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            IntegerTypeNode::le(U64)
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32))),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                IntegerTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            IntegerValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..IntegerTypeNode::le(U64)
                }
                .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32))),
            )
            .into(),
        )
    );
    Ok(())
}
