//! The tests of `resolveTypePath` and `isValueNodeOfType` in `@codama/visitors-core`.

use codama_nodes::{
    AccountLinkNode, AccountNode, ArrayTypeNode, ArrayValueNode, BooleanTypeNode, BooleanValueNode,
    BytesTypeNode, BytesValueNode, ConstantValueNode, DateTimeTypeNode, DefinedTypeLinkNode,
    DefinedTypeNode, DurationTypeNode, EnumTypeNode, EnumValueNode, EnumVariantTypeNode,
    FixedPointTypeNode, FloatTypeNode, FloatValueNode, InjectedValueNode, InstructionNode,
    IntegerTypeNode, IntegerValueNode, MapEntryValueNode, MapTypeNode, MapValueNode, NodeRef,
    NoneValueNode, OptionTypeNode, Path, ProgramLinkNode, ProgramNode, PublicKeyTypeNode,
    PublicKeyValueNode, RemainderOptionTypeNode, RootNode, SetTypeNode, SetValueNode,
    SomeValueNode, StringTypeNode, StringValueNode, StructFieldTypeNode, StructFieldValueNode,
    StructTypeNode, StructValueNode, TupleTypeNode, TupleValueNode, TypeNode, ValueNode,
    ZeroableOptionTypeNode, F64, I64, U16, U32, U64, U8,
};
use codama_visitors_core::{
    get_linkables_from_node, is_value_node_of_type, resolve_type_path, LinkableDictionary,
    NodePath, ResolveTypePathError, ResolveTypePathOptions,
};

fn path(value: &str) -> Path {
    value.parse().unwrap()
}

fn node_path<'a>(nodes: impl IntoIterator<Item = NodeRef<'a>>) -> NodePath<'a> {
    nodes.into_iter().collect()
}

const FOLLOW: ResolveTypePathOptions = ResolveTypePathOptions {
    follow_account_links: true,
};

fn resolve<'a>(
    source: &NodePath<'a>,
    value: &str,
    linkables: &LinkableDictionary<'a>,
    options: ResolveTypePathOptions,
) -> Result<String, String> {
    resolve_type_path(source, &path(value), linkables, options)
        .map(|resolved| resolved.to_string())
        .map_err(|error| error.to_string())
}

fn data_of(instruction: &InstructionNode) -> &TypeNode {
    instruction.data.as_ref().as_ref().unwrap()
}

#[test]
fn resolves_struct_fields() {
    let root = RootNode::new(ProgramNode {
        instructions: vec![InstructionNode {
            identifier: "transfer".into(),
            data: Box::new(Some(
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "config",
                    StructTypeNode::new(vec![StructFieldTypeNode::new(
                        "amount",
                        IntegerTypeNode::le(U64),
                    )]),
                )])
                .into(),
            )),
            ..InstructionNode::default()
        }],
        ..ProgramNode::new("myProgram", "1111")
    });
    let instruction = &root.program.instructions[0];
    let source = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(instruction),
        NodeRef::from(data_of(instruction)),
    ]);
    let linkables = get_linkables_from_node(&root);
    let prefix = "[programNode]myProgram.[instructionNode]transfer.[structTypeNode]";
    assert_eq!(
        resolve(&source, "config", &linkables, Default::default()),
        Ok(format!("{prefix}.[structFieldTypeNode]config"))
    );
    assert_eq!(
        resolve(&source, "config.amount", &linkables, Default::default()),
        Ok(format!(
            "{prefix}.[structFieldTypeNode]config.[structTypeNode].[structFieldTypeNode]amount"
        ))
    );
}

#[test]
fn resolves_tuple_array_and_set_items() {
    let tuple = TupleTypeNode::new(vec![
        IntegerTypeNode::le(U8).into(),
        PublicKeyTypeNode::new().into(),
    ]);
    let array = ArrayTypeNode::prefixed(IntegerTypeNode::le(U16), IntegerTypeNode::le(U32));
    let set = SetTypeNode::fixed(StringTypeNode::utf8(), 2);
    let linkables = LinkableDictionary::default();
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&tuple)]),
            "[1]",
            &linkables,
            Default::default()
        ),
        Ok("[tupleTypeNode].[publicKeyTypeNode]".to_string())
    );
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&array)]),
            "[5]",
            &linkables,
            Default::default()
        ),
        Ok("[arrayTypeNode].[integerTypeNode]".to_string())
    );
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&set)]),
            "[0]",
            &linkables,
            Default::default()
        ),
        Ok("[setTypeNode].[stringTypeNode]".to_string())
    );
}

#[test]
fn follows_links_to_other_programs_using_the_path_of_their_definitions() {
    let program_b = ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new(
                "config",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "fees",
                    ArrayTypeNode::fixed(DefinedTypeLinkNode::new("fee"), 2),
                )]),
            ),
            DefinedTypeNode::new("fee", IntegerTypeNode::le(U16)),
        ],
        ..ProgramNode::new("programB", "2222")
    };
    let root = RootNode {
        additional_programs: vec![program_b],
        ..RootNode::new(ProgramNode {
            instructions: vec![InstructionNode {
                identifier: "transfer".into(),
                data: Box::new(Some(
                    StructTypeNode::new(vec![StructFieldTypeNode::new(
                        "config",
                        DefinedTypeLinkNode::new_from_program(
                            "config",
                            ProgramLinkNode::new("programB"),
                        ),
                    )])
                    .into(),
                )),
                ..InstructionNode::default()
            }],
            ..ProgramNode::new("programA", "1111")
        })
    };
    let instruction = &root.program.instructions[0];
    let source = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(instruction),
        NodeRef::from(data_of(instruction)),
    ]);
    assert_eq!(
        resolve(&source, "config.fees[0]", &get_linkables_from_node(&root), Default::default()),
        Ok("[programNode]programB.[definedTypeNode]config.[structTypeNode].[structFieldTypeNode]fees.[arrayTypeNode].[definedTypeLinkNode]fee".to_string())
    );
}

#[test]
fn returns_the_last_node_as_addressed_without_following_it() {
    let link = DefinedTypeLinkNode::new("config");
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("items", TupleTypeNode::new(vec![link.clone().into()])),
        StructFieldTypeNode::new("config", link),
    ]);
    let linkables = LinkableDictionary::default();
    let source = node_path([NodeRef::from(&data)]);
    assert_eq!(
        resolve(&source, "config", &linkables, Default::default()),
        Ok("[structTypeNode].[structFieldTypeNode]config".to_string())
    );
    assert_eq!(
        resolve(&source, "items[0]", &linkables, Default::default()),
        Ok("[structTypeNode].[structFieldTypeNode]items.[tupleTypeNode].[definedTypeLinkNode]config".to_string())
    );
}

#[test]
fn resolves_paths_from_struct_fields_and_enum_variants() {
    let r#struct = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "amount",
        IntegerTypeNode::le(U64),
    )]);
    let field = StructFieldTypeNode::new("args", r#struct.clone());
    let variant = EnumVariantTypeNode::with_data("transfer", r#struct);
    let linkables = LinkableDictionary::default();
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&field)]),
            "amount",
            &linkables,
            Default::default()
        ),
        Ok("[structFieldTypeNode]args.[structTypeNode].[structFieldTypeNode]amount".to_string())
    );
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&variant)]),
            "amount",
            &linkables,
            Default::default()
        ),
        Ok(
            "[enumVariantTypeNode]transfer.[structTypeNode].[structFieldTypeNode]amount"
                .to_string()
        )
    );
}

fn cannot_resolve(segment: &str, path_string: &str, path: &str) -> Result<String, String> {
    Err(format!(
        "Cannot resolve segment [{segment}] of path [{path_string}] from {path}."
    ))
}

#[test]
fn fails_when_a_field_does_not_exist_or_a_segment_does_not_apply() {
    let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "amount",
        IntegerTypeNode::le(U64),
    )]);
    let tuple = TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()]);
    let variant = EnumVariantTypeNode::new("empty");
    let linkables = LinkableDictionary::default();
    let data_path = node_path([NodeRef::from(&data)]);
    let tuple_path = node_path([NodeRef::from(&tuple)]);
    assert_eq!(
        resolve(&data_path, "fee", &linkables, Default::default()),
        cannot_resolve("fee", "fee", "[structTypeNode]")
    );
    assert_eq!(
        resolve(&data_path, "amount[0]", &linkables, Default::default()),
        cannot_resolve(
            "[0]",
            "amount[0]",
            "[structTypeNode].[structFieldTypeNode]amount.[integerTypeNode]"
        )
    );
    assert_eq!(
        resolve(&tuple_path, "first", &linkables, Default::default()),
        cannot_resolve("first", "first", "[tupleTypeNode]")
    );
    assert_eq!(
        resolve(&tuple_path, "[1]", &linkables, Default::default()),
        cannot_resolve("[1]", "[1]", "[tupleTypeNode]")
    );
    assert_eq!(
        resolve(
            &node_path([NodeRef::from(&variant)]),
            "amount",
            &linkables,
            Default::default()
        ),
        cannot_resolve("amount", "amount", "[enumVariantTypeNode]empty")
    );
}

#[test]
fn fails_when_a_link_cannot_be_resolved() {
    let root = RootNode::new(ProgramNode::new("myProgram", "1111"));
    let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "config",
        DefinedTypeLinkNode::new("missing"),
    )]);
    let source = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(&data),
    ]);
    let linkables = get_linkables_from_node(&root);
    let error = resolve_type_path(&source, &path("config.fee"), &linkables, Default::default())
        .unwrap_err();
    assert!(matches!(
        error,
        ResolveTypePathError::LinkedNodeNotFound { .. }
    ));
    assert_eq!(
        error.to_string(),
        "Could not find the node linked from [programNode]myProgram.[structTypeNode].[structFieldTypeNode]config.[definedTypeLinkNode]missing."
    );
}

#[test]
fn fails_on_alias_cycles_rather_than_looping() {
    let root = RootNode::new(ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new("a", DefinedTypeLinkNode::new("b")),
            DefinedTypeNode::new("b", DefinedTypeLinkNode::new("a")),
        ],
        ..ProgramNode::new("myProgram", "1111")
    });
    let type_a = &root.program.defined_types[0];
    let source = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(type_a),
        NodeRef::from(&*type_a.r#type),
    ]);
    assert_eq!(
        resolve(
            &source,
            "x",
            &get_linkables_from_node(&root),
            Default::default()
        ),
        cannot_resolve(
            "x",
            "x",
            "[programNode]myProgram.[definedTypeNode]a.[definedTypeLinkNode]b"
        )
    );
}

/// A token account whose `mint` field links to the mint account.
fn token_program() -> RootNode {
    RootNode::new(ProgramNode {
        accounts: vec![
            AccountNode::new(
                "token",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "mint",
                    PublicKeyTypeNode {
                        account_link: Some(AccountLinkNode::new("mint")),
                        ..PublicKeyTypeNode::new()
                    },
                )]),
            ),
            AccountNode::new(
                "mint",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "decimals",
                    IntegerTypeNode::le(U8),
                )]),
            ),
        ],
        ..ProgramNode::new("myProgram", "1111")
    })
}

fn account_source<'a>(root: &'a RootNode, index: usize) -> NodePath<'a> {
    let account = &root.program.accounts[index];
    node_path([
        NodeRef::from(root),
        NodeRef::from(&root.program),
        NodeRef::from(account),
        NodeRef::from(&*account.data),
    ])
}

const MINT_FIELD: &str =
    "[programNode]myProgram.[accountNode]token.[structTypeNode].[structFieldTypeNode]mint.[publicKeyTypeNode]";

#[test]
fn follows_account_links_with_a_data_segment() {
    let root = token_program();
    let linkables = get_linkables_from_node(&root);
    let source = account_source(&root, 0);
    assert_eq!(
        resolve(&source, "mint.data.decimals", &linkables, FOLLOW),
        Ok("[programNode]myProgram.[accountNode]mint.[structTypeNode].[structFieldTypeNode]decimals".to_string())
    );
    assert_eq!(
        resolve(&source, "mint.data", &linkables, FOLLOW),
        Ok("[programNode]myProgram.[accountNode]mint.[structTypeNode]".to_string())
    );
    assert_eq!(
        resolve(&source, "mint", &linkables, FOLLOW),
        Ok(
            "[programNode]myProgram.[accountNode]token.[structTypeNode].[structFieldTypeNode]mint"
                .to_string()
        )
    );
}

#[test]
fn does_not_follow_account_links_with_other_segments_or_by_default() {
    let root = token_program();
    let linkables = get_linkables_from_node(&root);
    let source = account_source(&root, 0);
    for segment in ["decimals", "lamports"] {
        let value = format!("mint.{segment}");
        assert_eq!(
            resolve(&source, &value, &linkables, FOLLOW),
            cannot_resolve(segment, &value, MINT_FIELD)
        );
    }
    assert_eq!(
        resolve(
            &source,
            "mint.data.decimals",
            &linkables,
            Default::default()
        ),
        cannot_resolve("data", "mint.data.decimals", MINT_FIELD)
    );
    assert_eq!(
        resolve(&source, "mint[0]", &linkables, FOLLOW),
        cannot_resolve("[0]", "mint[0]", MINT_FIELD)
    );
}

#[test]
fn resolves_a_data_segment_to_data_that_is_not_a_struct() {
    let root = RootNode::new(ProgramNode {
        accounts: vec![AccountNode::new("counter", IntegerTypeNode::le(U64))],
        ..ProgramNode::new("counters", "6666")
    });
    let holder = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "counter",
        PublicKeyTypeNode {
            account_link: Some(AccountLinkNode::new("counter")),
            ..PublicKeyTypeNode::new()
        },
    )]);
    let source = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(&holder),
    ]);
    assert_eq!(
        resolve(
            &source,
            "counter.data",
            &get_linkables_from_node(&root),
            FOLLOW
        ),
        Ok("[programNode]counters.[accountNode]counter.[integerTypeNode]".to_string())
    );
}

#[test]
fn follows_account_links_back_to_the_same_account_across_segments() {
    let root = RootNode::new(ProgramNode {
        accounts: vec![
            AccountNode::new(
                "node",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("value", IntegerTypeNode::le(U8)),
                    StructFieldTypeNode::new(
                        "next",
                        PublicKeyTypeNode {
                            account_link: Some(AccountLinkNode::new("node")),
                            ..PublicKeyTypeNode::new()
                        },
                    ),
                ]),
            ),
            AccountNode::new(
                "self",
                PublicKeyTypeNode {
                    account_link: Some(AccountLinkNode::new("self")),
                    ..PublicKeyTypeNode::new()
                },
            ),
        ],
        ..ProgramNode::new("list", "2222")
    });
    let linkables = get_linkables_from_node(&root);
    assert_eq!(
        resolve(
            &account_source(&root, 0),
            "next.data.next.data.value",
            &linkables,
            FOLLOW
        ),
        Ok(
            "[programNode]list.[accountNode]node.[structTypeNode].[structFieldTypeNode]value"
                .to_string()
        )
    );
    assert_eq!(
        resolve(&account_source(&root, 1), "data.data", &linkables, FOLLOW),
        Ok("[programNode]list.[accountNode]self.[publicKeyTypeNode]".to_string())
    );
}

#[test]
fn fails_when_a_linked_account_cannot_be_found() {
    let root = RootNode::new(ProgramNode {
        accounts: vec![AccountNode::new(
            "holder",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "missing",
                PublicKeyTypeNode {
                    account_link: Some(AccountLinkNode::new("missing")),
                    ..PublicKeyTypeNode::new()
                },
            )]),
        )],
        ..ProgramNode::new("holder", "4444")
    });
    assert_eq!(
        resolve(&account_source(&root, 0), "missing.data", &get_linkables_from_node(&root), FOLLOW),
        Err("Could not find the node linked from [programNode]holder.[accountNode]holder.[structTypeNode].[structFieldTypeNode]missing.[publicKeyTypeNode].[accountLinkNode]missing.".to_string())
    );
}

// --- isValueNodeOfType ---

fn is_of_type(value: impl Into<ValueNode>, r#type: impl Into<TypeNode>) -> bool {
    let r#type: TypeNode = r#type.into();
    is_value_node_of_type(
        &value.into(),
        &node_path([NodeRef::from(&r#type)]),
        &LinkableDictionary::default(),
    )
    .unwrap()
}

fn integer(value: i64) -> IntegerValueNode {
    IntegerValueNode::new(value)
}

#[test]
fn accepts_values_of_their_type() {
    assert!(is_of_type(
        BooleanValueNode::new(true),
        BooleanTypeNode::default()
    ));
    assert!(is_of_type(
        BytesValueNode::base16("0102"),
        BytesTypeNode::new()
    ));
    assert!(is_of_type(
        integer(0),
        DateTimeTypeNode::new(IntegerTypeNode::le(I64))
    ));
    assert!(is_of_type(
        integer(60),
        DurationTypeNode::new(IntegerTypeNode::le(U64))
    ));
    assert!(is_of_type(
        integer(12345),
        FixedPointTypeNode::new(IntegerTypeNode::le(U64), 2)
    ));
    assert!(is_of_type(FloatValueNode::new(1.5), FloatTypeNode::le(F64)));
    assert!(is_of_type(integer(1), FloatTypeNode::le(F64)));
    assert!(is_of_type(integer(1), IntegerTypeNode::le(U64)));
    assert!(is_of_type(
        PublicKeyValueNode::new("11111111111111111111111111111111"),
        PublicKeyTypeNode::new()
    ));
    assert!(is_of_type(
        StringValueNode::new("hello"),
        StringTypeNode::utf8()
    ));
}

#[test]
fn rejects_values_of_other_types() {
    assert!(!is_of_type(
        StringValueNode::new("All"),
        IntegerTypeNode::le(U64)
    ));
    assert!(!is_of_type(
        FloatValueNode::new(1.5),
        IntegerTypeNode::le(U64)
    ));
    assert!(!is_of_type(integer(1), BooleanTypeNode::default()));
    assert!(!is_of_type(integer(1), StringTypeNode::utf8()));
    assert!(!is_of_type(
        StringValueNode::new("11111111111111111111111111111111"),
        PublicKeyTypeNode::new()
    ));
    assert!(!is_of_type(NoneValueNode::new(), IntegerTypeNode::le(U64)));
}

#[test]
fn rejects_injected_and_constant_values_which_are_not_concrete() {
    assert!(!is_of_type(
        InjectedValueNode::new("max"),
        IntegerTypeNode::le(U64)
    ));
    assert!(!is_of_type(
        ConstantValueNode::new(IntegerTypeNode::le(U64), integer(1)),
        IntegerTypeNode::le(U64)
    ));
}

#[test]
fn accepts_none_and_some_values_of_the_item_type_for_options() {
    let types: [TypeNode; 3] = [
        OptionTypeNode::new(IntegerTypeNode::le(U64)).into(),
        ZeroableOptionTypeNode::new(IntegerTypeNode::le(U64)).into(),
        RemainderOptionTypeNode::new(IntegerTypeNode::le(U64)).into(),
    ];
    for r#type in types {
        assert!(is_of_type(NoneValueNode::new(), r#type.clone()));
        assert!(is_of_type(SomeValueNode::new(integer(1)), r#type.clone()));
        assert!(!is_of_type(
            SomeValueNode::new(StringValueNode::new("1")),
            r#type
        ));
    }
}

#[test]
fn checks_the_items_of_arrays_sets_tuples_and_maps() {
    let array = ArrayTypeNode::fixed(IntegerTypeNode::le(U64), 2);
    assert!(is_of_type(
        ArrayValueNode::new(vec![integer(1).into(), integer(2).into()]),
        array.clone()
    ));
    assert!(!is_of_type(
        ArrayValueNode::new(vec![StringValueNode::new("1").into()]),
        array.clone()
    ));
    assert!(!is_of_type(
        SetValueNode::new(vec![integer(1).into()]),
        array
    ));

    let set = SetTypeNode::fixed(IntegerTypeNode::le(U64), 1);
    assert!(is_of_type(
        SetValueNode::new(vec![integer(1).into()]),
        set.clone()
    ));
    assert!(!is_of_type(
        ArrayValueNode::new(vec![integer(1).into()]),
        set
    ));

    let tuple = TupleTypeNode::new(vec![
        IntegerTypeNode::le(U64).into(),
        StringTypeNode::utf8().into(),
    ]);
    let value = |items: Vec<ValueNode>| TupleValueNode::new(items);
    assert!(is_of_type(
        value(vec![integer(1).into(), StringValueNode::new("a").into()]),
        tuple.clone()
    ));
    assert!(!is_of_type(
        value(vec![StringValueNode::new("a").into(), integer(1).into()]),
        tuple.clone()
    ));
    assert!(!is_of_type(value(vec![integer(1).into()]), tuple));

    let map = MapTypeNode::fixed(StringTypeNode::utf8(), IntegerTypeNode::le(U64), 1);
    let entry = |key: ValueNode, value: ValueNode| MapEntryValueNode::new(key, value);
    assert!(is_of_type(
        MapValueNode::new(vec![entry(
            StringValueNode::new("a").into(),
            integer(1).into()
        )]),
        map.clone()
    ));
    assert!(!is_of_type(
        MapValueNode::new(vec![entry(integer(1).into(), integer(1).into())]),
        map
    ));
}

#[test]
fn checks_the_fields_of_structs() {
    let amount = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U64));
    let r#type = StructTypeNode::new(vec![amount.clone()]);
    let field =
        |identifier: &'static str, value: ValueNode| StructFieldValueNode::new(identifier, value);
    assert!(is_of_type(
        StructValueNode::new(vec![field("amount", integer(1).into())]),
        r#type.clone()
    ));
    assert!(!is_of_type(
        StructValueNode::new(vec![field("amount", StringValueNode::new("1").into())]),
        r#type.clone()
    ));
    assert!(!is_of_type(
        StructValueNode::new(vec![
            field("amount", integer(1).into()),
            field("extra", integer(1).into())
        ]),
        r#type
    ));

    let with_fee = |default_value: Option<ValueNode>| {
        StructTypeNode::new(vec![
            amount.clone(),
            StructFieldTypeNode {
                default_value: Box::new(default_value),
                ..StructFieldTypeNode::new("fee", IntegerTypeNode::le(U64))
            },
        ])
    };
    let only_amount = StructValueNode::new(vec![field("amount", integer(1).into())]);
    assert!(!is_of_type(only_amount.clone(), with_fee(None)));
    assert!(is_of_type(only_amount, with_fee(Some(integer(0).into()))));
}

#[test]
fn checks_the_variants_of_enums() {
    let r#type = EnumTypeNode::new(vec![
        EnumVariantTypeNode::new("market"),
        EnumVariantTypeNode::with_data("limit", IntegerTypeNode::le(U64)),
    ]);
    let link = DefinedTypeLinkNode::new("order");
    let value = |variant: &'static str, data: Option<ValueNode>| match data {
        Some(data) => EnumValueNode::with_value(link.clone(), variant, data),
        None => EnumValueNode::new(link.clone(), variant),
    };
    assert!(is_of_type(value("market", None), r#type.clone()));
    assert!(is_of_type(
        value("limit", Some(integer(5).into())),
        r#type.clone()
    ));
    assert!(!is_of_type(value("stop", None), r#type.clone()));
    assert!(!is_of_type(value("limit", None), r#type.clone()));
    assert!(!is_of_type(
        value("limit", Some(StringValueNode::new("5").into())),
        r#type.clone()
    ));
    assert!(!is_of_type(
        value("market", Some(integer(5).into())),
        r#type
    ));
}

#[test]
fn follows_links_to_defined_types_and_rejects_alias_cycles() {
    let root = RootNode::new(ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new("amount", OptionTypeNode::new(IntegerTypeNode::le(U64))),
            DefinedTypeNode::new("a", DefinedTypeLinkNode::new("b")),
            DefinedTypeNode::new("b", DefinedTypeLinkNode::new("a")),
        ],
        ..ProgramNode::new("myProgram", "1111")
    });
    let linkables = get_linkables_from_node(&root);
    let amount = DefinedTypeLinkNode::new("amount");
    let a = DefinedTypeLinkNode::new("a");
    let type_path = |link| {
        node_path([
            NodeRef::from(&root),
            NodeRef::from(&root.program),
            NodeRef::from(link),
        ])
    };
    assert!(is_value_node_of_type(
        &NoneValueNode::new().into(),
        &type_path(&amount),
        &linkables
    )
    .unwrap());
    assert!(!is_value_node_of_type(&integer(1).into(), &type_path(&amount), &linkables).unwrap());
    assert!(!is_value_node_of_type(&integer(1).into(), &type_path(&a), &linkables).unwrap());
}

#[test]
fn fails_when_a_link_to_a_defined_type_cannot_be_resolved() {
    let root = RootNode::new(ProgramNode::new("myProgram", "1111"));
    let link = DefinedTypeLinkNode::new("missing");
    let type_path = node_path([
        NodeRef::from(&root),
        NodeRef::from(&root.program),
        NodeRef::from(&link),
    ]);
    let error = is_value_node_of_type(
        &integer(1).into(),
        &type_path,
        &LinkableDictionary::default(),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Could not find the node linked from [programNode]myProgram.[definedTypeLinkNode]missing."
    );
}
