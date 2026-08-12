use codama_errors::CodamaResult;
use codama_korok_visitors::{
    ApplyDisplayVisitor, ApplyTypeOverridesVisitor, IdentifyFieldTypesVisitor, KorokVisitable,
};
use codama_koroks::{FieldKorok, KorokTrait};
use codama_nodes::{
    AmountNumberDisplayNode, AmountTypeNode, ArrayTypeNode, BooleanTypeNode, ConstantValueNode,
    DateTimeNumberDisplayNode, DateTimeTypeNode, DefinedTypeLinkNode, FixedSizeTypeNode,
    HiddenPrefixTypeNode, HiddenSuffixTypeNode, MapTypeNode, NestedTypeNode, NumberDisplayNode,
    NumberFormat::{U32, U64, U8},
    NumberTypeNode, NumberValueNode, OptionTypeNode, PostOffsetTypeNode, PreOffsetTypeNode,
    RemainderOptionTypeNode, SentinelTypeNode, SetTypeNode, SizePrefixTypeNode, SolAmountTypeNode,
    StringTypeNode, StringValueNode, StructFieldTypeNode, TupleTypeNode, TypeNode,
    ZeroableOptionTypeNode,
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
                NumberTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            NumberValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..NumberTypeNode::le(U64)
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
        #[codama(type = number(u64, display = date_time))]
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
                NumberTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            NumberValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..NumberTypeNode::le(U64)
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
        #[codama(display(date_time))]
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
                NumberTypeNode {
                    display: Box::new(Some(NumberDisplayNode::Amount(
                        AmountNumberDisplayNode::new(
                            NumberValueNode::new(9u64),
                            StringValueNode::new("SOL"),
                        ),
                    ))),
                    ..NumberTypeNode::le(U64)
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
        #[codama(display(date_time(ticks_per_second = 1_000)))]
        values: Vec<Option<u64>>
    };
    let mut korok = FieldKorok::parse(&ast)?;
    let prefix = NumberTypeNode::le(U32);
    let expected_prefix = prefix.clone();
    let display = NumberDisplayNode::DateTime(DateTimeNumberDisplayNode::new(Some(1_000)));
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "values",
            FixedSizeTypeNode::new(
                SizePrefixTypeNode::new(
                    OptionTypeNode {
                        fixed: None,
                        item: Box::new(ArrayTypeNode::fixed(NumberTypeNode::le(U64), 2).into()),
                        prefix: NumberTypeNode::le(U8).into(),
                    },
                    prefix,
                ),
                42,
            ),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "values",
                FixedSizeTypeNode::new(
                    SizePrefixTypeNode::new(
                        OptionTypeNode {
                            fixed: None,
                            item: Box::new(
                                ArrayTypeNode::fixed(
                                    NumberTypeNode {
                                        display: Box::new(Some(display)),
                                        ..NumberTypeNode::le(U64)
                                    },
                                    2,
                                )
                                .into(),
                            ),
                            prefix: NumberTypeNode::le(U8).into(),
                        },
                        expected_prefix,
                    ),
                    42,
                ),
            )
            .into()
        )
    );
    Ok(())
}

#[test]
fn it_traverses_all_nested_type_modifiers() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        value: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;
    let hidden_prefix = ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(1u8));
    let hidden_suffix = ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(2u8));
    let sentinel = ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(255u8));
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            PreOffsetTypeNode::relative(
                PostOffsetTypeNode::padded(
                    HiddenPrefixTypeNode::new(
                        HiddenSuffixTypeNode::new(
                            SentinelTypeNode::new(
                                RemainderOptionTypeNode::new(NumberTypeNode::le(U64)),
                                sentinel.clone(),
                            ),
                            vec![hidden_suffix.clone()],
                        ),
                        vec![hidden_prefix.clone()],
                    ),
                    8,
                ),
                4,
            ),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                PreOffsetTypeNode::relative(
                    PostOffsetTypeNode::padded(
                        HiddenPrefixTypeNode::new(
                            HiddenSuffixTypeNode::new(
                                SentinelTypeNode::new(
                                    RemainderOptionTypeNode::new(NumberTypeNode {
                                        display: Box::new(Some(NumberDisplayNode::Amount(
                                            AmountNumberDisplayNode::default(),
                                        ))),
                                        ..NumberTypeNode::le(U64)
                                    }),
                                    sentinel,
                                ),
                                vec![hidden_suffix],
                            ),
                            vec![hidden_prefix],
                        ),
                        8,
                    ),
                    4,
                ),
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_traverses_alternative_option_encodings() -> CodamaResult<()> {
    let remainder_ast: syn::Field = syn::parse_quote! {
        #[codama(display(date_time))]
        value: u64
    };
    let mut remainder_korok = FieldKorok::parse(&remainder_ast)?;
    remainder_korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            RemainderOptionTypeNode::new(NumberTypeNode::le(U64)),
        )
        .into(),
    ));

    remainder_korok.accept(&mut ApplyDisplayVisitor::new())?;

    assert_eq!(
        remainder_korok.node,
        Some(
            StructFieldTypeNode::new(
                "value",
                RemainderOptionTypeNode::new(NumberTypeNode {
                    display: Box::new(Some(NumberDisplayNode::DateTime(
                        DateTimeNumberDisplayNode::default(),
                    ))),
                    ..NumberTypeNode::le(U64)
                }),
            )
            .into(),
        )
    );

    let zeroable_ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        value: u64
    };
    let mut zeroable_korok = FieldKorok::parse(&zeroable_ast)?;
    let zero_value = ConstantValueNode::new(NumberTypeNode::le(U64), NumberValueNode::new(0u64));
    zeroable_korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            ZeroableOptionTypeNode::custom(NumberTypeNode::le(U64), zero_value.clone()),
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
                    NumberTypeNode {
                        display: Box::new(Some(NumberDisplayNode::Amount(
                            AmountNumberDisplayNode::default(),
                        ))),
                        ..NumberTypeNode::le(U64)
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
fn it_traverses_wrappers_from_type_overrides() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(type = pre_offset(number(u64), 4, relative))]
        #[codama(display(amount))]
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
                PreOffsetTypeNode::relative(
                    NumberTypeNode {
                        display: Box::new(Some(NumberDisplayNode::Amount(
                            AmountNumberDisplayNode::default(),
                        ))),
                        ..NumberTypeNode::le(U64)
                    },
                    4,
                ),
            )
            .into(),
        )
    );
    Ok(())
}

#[test]
fn it_reports_errors_at_the_logical_leaf() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        value: u64
    };
    let wrapped_types: Vec<TypeNode> = vec![
        PreOffsetTypeNode::relative(StringTypeNode::utf8(), 4).into(),
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
        #[codama(display(amount))]
        amounts: [u64; 2]
    };
    let vector_ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
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
        let TypeNode::Number(number) = *array.item else {
            panic!("Expected a number type node");
        };
        assert_eq!(
            *number.display,
            Some(NumberDisplayNode::Amount(AmountNumberDisplayNode::default()))
        );
    }
    Ok(())
}

#[test]
fn it_does_not_traverse_composite_type_nodes() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        value: u64
    };
    let unsupported: Vec<(TypeNode, &str)> = vec![
        (BooleanTypeNode::default().into(), "booleanTypeNode"),
        (
            SetTypeNode::fixed(NumberTypeNode::le(U64), 2).into(),
            "setTypeNode",
        ),
        (
            MapTypeNode::fixed(NumberTypeNode::le(U64), NumberTypeNode::le(U64), 2).into(),
            "mapTypeNode",
        ),
        (
            TupleTypeNode::new(vec![NumberTypeNode::le(U64).into()]).into(),
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
        #[codama(display(amount))]
        value: u64
    };
    let unsupported: Vec<(TypeNode, &str)> = vec![
        (
            AmountTypeNode::new(NumberTypeNode::le(U64), 9, Some("SOL".to_string())).into(),
            "amountTypeNode",
        ),
        (
            DateTimeTypeNode::new(NumberTypeNode::le(U64)).into(),
            "dateTimeTypeNode",
        ),
        (
            SolAmountTypeNode::new(NumberTypeNode::le(U64)).into(),
            "solAmountTypeNode",
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
fn it_does_not_update_wrapper_metadata() -> CodamaResult<()> {
    let ast: syn::Field = syn::parse_quote! {
        #[codama(display(amount))]
        value: u64
    };
    let mut korok = FieldKorok::parse(&ast)?;
    let prefix = NumberTypeNode::le(U32);
    korok.set_node(Some(
        StructFieldTypeNode::new(
            "value",
            SizePrefixTypeNode::new(NumberTypeNode::le(U64), prefix.clone()),
        )
        .into(),
    ));

    korok.accept(&mut ApplyDisplayVisitor::new())?;

    let Some(codama_nodes::Node::Type(codama_nodes::RegisteredTypeNode::StructField(field))) =
        korok.node
    else {
        panic!("Expected a struct field type node");
    };
    let TypeNode::SizePrefix(size_prefix) = *field.r#type else {
        panic!("Expected a size prefix type node");
    };
    assert_eq!(*size_prefix.prefix, NestedTypeNode::Value(prefix));
    let TypeNode::Number(number) = *size_prefix.r#type else {
        panic!("Expected a number type node");
    };
    assert_eq!(
        *number.display,
        Some(NumberDisplayNode::Amount(AmountNumberDisplayNode::default()))
    );
    Ok(())
}
