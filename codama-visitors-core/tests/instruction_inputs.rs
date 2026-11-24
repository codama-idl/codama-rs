//! The tests of `getResolvedInstructionInputsVisitor` and `getInstructionDataFields` in
//! `@codama/visitors-core`.

use codama_nodes::{
    AccountBumpValueNode, AccountValueNode, ConditionalValueNode, DataValueNode,
    DefinedTypeLinkNode, DefinedTypeNode, InjectedValueNode, InstructionAccountNode,
    InstructionInputValueNode, InstructionNode, IntegerTypeNode, IntegerValueNode, IsSigner, Node,
    NodeRef, PdaLinkNode, PdaSeedValueNode, PdaValueNode, ProgramLinkNode, ProgramNode,
    ProvidedNode, PublicKeyTypeNode, PublicKeyValueNode, RootNode, StructFieldTypeNode,
    StructTypeNode, TypeNode, ValueNode, U16, U64, U8,
};
use codama_visitors_core::{
    get_instruction_data_fields, get_linkables_from_node, get_resolved_instruction_inputs,
    InstructionDependency, LinkableDictionary, NodePath, ResolveInstructionInputsOptions,
    ResolvedInstructionAccount, ResolvedInstructionDataField, ResolvedInstructionInput,
};

fn account(identifier: &'static str, is_signer: IsSigner) -> InstructionAccountNode {
    InstructionAccountNode::new(identifier, false, is_signer)
}

fn defaulting(
    identifier: &'static str,
    is_signer: IsSigner,
    value: impl Into<InstructionInputValueNode>,
) -> InstructionAccountNode {
    InstructionAccountNode {
        default_value: Box::new(Some(value.into())),
        ..account(identifier, is_signer)
    }
}

fn field(identifier: &'static str, r#type: impl Into<TypeNode>) -> StructFieldTypeNode {
    StructFieldTypeNode::new(identifier, r#type)
}

fn field_defaulting(
    identifier: &'static str,
    r#type: impl Into<TypeNode>,
    value: impl Into<ValueNode>,
) -> StructFieldTypeNode {
    StructFieldTypeNode {
        default_value: Box::new(Some(value.into())),
        ..field(identifier, r#type)
    }
}

fn instruction(identifier: &'static str) -> InstructionNode {
    InstructionNode {
        identifier: identifier.into(),
        ..InstructionNode::default()
    }
}

fn provide(identifier: &'static str, value: impl Into<Node>) -> ProvidedNode {
    ProvidedNode::new(identifier, value)
}

fn depends_on_account(identifier: &'static str) -> InstructionDependency {
    InstructionDependency::Account(AccountValueNode::new(identifier))
}

fn resolve<'a>(
    path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> Vec<ResolvedInstructionInput<'a>> {
    get_resolved_instruction_inputs(path, linkables, Default::default()).unwrap()
}

/// The inputs of an instruction on its own, outside of any program.
fn resolve_alone(instruction: &InstructionNode) -> Vec<ResolvedInstructionInput<'_>> {
    resolve(
        &NodePath::from(vec![NodeRef::from(instruction)]),
        &LinkableDictionary::default(),
    )
}

fn as_account<'r, 'a>(
    input: &'r ResolvedInstructionInput<'a>,
) -> &'r ResolvedInstructionAccount<'a> {
    match input {
        ResolvedInstructionInput::Account(account) => account,
        input => panic!("expected an account, got {input:?}"),
    }
}

fn field_at<'r, 'a>(
    inputs: &'r [ResolvedInstructionInput<'a>],
    path: &str,
) -> &'r ResolvedInstructionDataField<'a> {
    inputs
        .iter()
        .find_map(|input| match input {
            ResolvedInstructionInput::DataField(field) if field.path.as_str() == path => {
                Some(field)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no data field at {path}"))
}

fn identifiers(inputs: &[ResolvedInstructionInput]) -> Vec<String> {
    inputs
        .iter()
        .map(|input| input.node().identifier().unwrap().to_string())
        .collect()
}

#[test]
fn returns_all_instruction_accounts_in_order_of_resolution() {
    let node = InstructionNode {
        accounts: vec![
            defaulting("owner", IsSigner::True, AccountValueNode::new("authority")),
            account("authority", IsSigner::True),
        ],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(
        inputs,
        [
            ResolvedInstructionInput::Account(ResolvedInstructionAccount {
                node: &node.accounts[1],
                depends_on: vec![],
                is_pda: false,
                resolved_default_value: None,
                resolved_is_optional: false,
                resolved_is_signer: IsSigner::True,
            }),
            ResolvedInstructionInput::Account(ResolvedInstructionAccount {
                node: &node.accounts[0],
                depends_on: vec![depends_on_account("authority")],
                is_pda: false,
                resolved_default_value: Some(AccountValueNode::new("authority").into()),
                resolved_is_optional: false,
                resolved_is_signer: IsSigner::True,
            }),
        ]
    );
}

#[test]
fn sets_the_resolved_signer_to_either_when_signers_differ() {
    for (owner, authority) in [
        (IsSigner::False, IsSigner::True),
        (IsSigner::True, IsSigner::False),
    ] {
        let node = InstructionNode {
            accounts: vec![
                defaulting("owner", owner, AccountValueNode::new("authority")),
                account("authority", authority),
            ],
            ..instruction("myInstruction")
        };
        let inputs = resolve_alone(&node);
        assert_eq!(as_account(&inputs[1]).resolved_is_signer, IsSigner::Either);
    }
}

#[test]
fn carries_the_resolved_signer_and_optional_flags_along_a_chain_of_defaults() {
    // `a` and `b` are signers, but `b` defaults to `c`, which is not, so `b` may not sign and nor may `a`.
    let node = InstructionNode {
        accounts: vec![
            defaulting("a", IsSigner::True, AccountValueNode::new("b")),
            defaulting("b", IsSigner::True, AccountValueNode::new("c")),
            account("c", IsSigner::False),
        ],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["c", "b", "a"]);
    assert_eq!(as_account(&inputs[1]).resolved_is_signer, IsSigner::Either);
    assert_eq!(as_account(&inputs[2]).resolved_is_signer, IsSigner::Either);

    // `b` is optional, but it defaults to the payer, which is always there, so `a` is not optional.
    let node = InstructionNode {
        accounts: vec![
            defaulting("a", IsSigner::False, AccountValueNode::new("b")),
            InstructionAccountNode {
                is_optional: Some(true),
                ..defaulting("b", IsSigner::False, codama_nodes::PayerValueNode::new())
            },
        ],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["b", "a"]);
    assert!(!as_account(&inputs[0]).resolved_is_optional);
    assert!(!as_account(&inputs[1]).resolved_is_optional);
}

#[test]
fn includes_instruction_data_fields_with_resolvable_default_values() {
    let node = InstructionNode {
        accounts: vec![account("owner", IsSigner::True)],
        data: Box::new(Some(
            StructTypeNode::new(vec![
                field_defaulting(
                    "ownerArg",
                    PublicKeyTypeNode::new(),
                    InjectedValueNode::new("owner"),
                ),
                field("argWithoutDefaults", IntegerTypeNode::le(U8)),
            ])
            .into(),
        )),
        provides: vec![provide("owner", AccountValueNode::new("owner"))],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["owner", "ownerArg"]);
    let owner_arg = field_at(&inputs, "ownerArg");
    assert_eq!(owner_arg.depends_on, [depends_on_account("owner")]);
    assert_eq!(
        owner_arg.resolved_default_value,
        Some(AccountValueNode::new("owner").into())
    );
}

#[test]
fn returns_an_empty_list_for_empty_instructions() {
    assert_eq!(resolve_alone(&instruction("myInstruction")), []);
}

#[test]
fn resolves_the_seeds_and_the_program_id_of_a_pda_value_first() {
    let pda = PdaValueNode::new(
        PdaLinkNode::new("counter"),
        vec![PdaSeedValueNode::new(
            "authority",
            AccountValueNode::new("payer"),
        )],
    );
    let node = InstructionNode {
        accounts: vec![
            defaulting("counter", IsSigner::False, pda.clone()),
            account("payer", IsSigner::True),
        ],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["payer", "counter"]);
    let counter = as_account(&inputs[1]);
    assert_eq!(counter.depends_on, [depends_on_account("payer")]);
    assert_eq!(counter.resolved_default_value, Some(pda.into()));
    assert_eq!(counter.resolved_is_signer, IsSigner::False);

    let pda = PdaValueNode::new_with_program_id(
        PdaLinkNode::new("counter"),
        vec![],
        AccountValueNode::new("counterProgram"),
    );
    let node = InstructionNode {
        accounts: vec![
            defaulting("counter", IsSigner::False, pda),
            account("counterProgram", IsSigner::False),
        ],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["counterProgram", "counter"]);
    assert_eq!(
        as_account(&inputs[1]).depends_on,
        [depends_on_account("counterProgram")]
    );

    // A program id read from a data field is a dependency too, so the field resolves first.
    let pda = PdaValueNode::new_with_program_id(
        PdaLinkNode::new("counter"),
        vec![],
        DataValueNode::new("program"),
    );
    let node = InstructionNode {
        accounts: vec![defaulting("counter", IsSigner::False, pda)],
        data: Box::new(Some(
            StructTypeNode::new(vec![field("program", PublicKeyTypeNode::new())]).into(),
        )),
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["program", "counter"]);
    assert_eq!(
        as_account(&inputs[1]).depends_on,
        [InstructionDependency::Data(DataValueNode::new("program"))]
    );
}

fn bump_instruction(provides: Vec<ProvidedNode>) -> InstructionNode {
    InstructionNode {
        accounts: vec![InstructionAccountNode::new("counter", true, false)],
        data: Box::new(Some(
            StructTypeNode::new(vec![field_defaulting(
                "bump",
                IntegerTypeNode::le(U8),
                InjectedValueNode::new("counterBump"),
            )])
            .into(),
        )),
        provides,
        ..instruction("myInstruction")
    }
}

#[test]
fn resolves_an_injected_data_field_default_and_marks_its_account_as_a_pda() {
    let node = bump_instruction(vec![provide(
        "counterBump",
        AccountBumpValueNode::new("counter"),
    )]);
    let inputs = resolve_alone(&node);
    let bump = field_at(&inputs, "bump");
    assert_eq!(bump.depends_on, [depends_on_account("counter")]);
    assert_eq!(
        bump.resolved_default_value,
        Some(AccountBumpValueNode::new("counter").into())
    );
    assert!(as_account(&inputs[0]).is_pda);
}

#[test]
fn fails_when_a_provided_value_is_not_an_instruction_input_value() {
    let node = bump_instruction(vec![provide("counterBump", PublicKeyTypeNode::new())]);
    let path = NodePath::from(vec![NodeRef::from(&node)]);
    let error =
        get_resolved_instruction_inputs(&path, &LinkableDictionary::default(), Default::default())
            .unwrap_err();
    assert!(error
        .to_string()
        .starts_with("The value provided at [providedNode]counterBump.[publicKeyTypeNode] is not one of [accountBumpValueNode, "));
}

#[test]
fn resolves_injections_nested_within_a_default_value() {
    let injected = PdaValueNode::new(
        PdaLinkNode::new("counter"),
        vec![PdaSeedValueNode::new(
            "authority",
            InjectedValueNode::new("counterAuthority"),
        )],
    );
    let node = InstructionNode {
        accounts: vec![
            InstructionAccountNode {
                default_value: Box::new(Some(injected.into())),
                ..InstructionAccountNode::new("counter", true, false)
            },
            account("payer", IsSigner::True),
        ],
        provides: vec![provide("counterAuthority", AccountValueNode::new("payer"))],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    assert_eq!(identifiers(&inputs), ["payer", "counter"]);
    let counter = as_account(&inputs[1]);
    assert_eq!(counter.depends_on, [depends_on_account("payer")]);
    assert_eq!(
        counter.resolved_default_value,
        Some(
            PdaValueNode::new(
                PdaLinkNode::new("counter"),
                vec![PdaSeedValueNode::new(
                    "authority",
                    AccountValueNode::new("payer")
                )],
            )
            .into()
        )
    );
}

#[test]
fn drops_a_default_value_with_an_unresolvable_nested_injection() {
    let node = InstructionNode {
        accounts: vec![InstructionAccountNode {
            default_value: Box::new(Some(
                PdaValueNode::new(
                    PdaLinkNode::new("counter"),
                    vec![PdaSeedValueNode::new(
                        "authority",
                        InjectedValueNode::new("counterAuthority"),
                    )],
                )
                .into(),
            )),
            ..InstructionAccountNode::new("counter", true, false)
        }],
        ..instruction("myInstruction")
    };
    let inputs = resolve_alone(&node);
    let counter = as_account(&inputs[0]);
    assert_eq!(counter.resolved_default_value, None);
    assert_eq!(counter.depends_on, []);
}

#[test]
fn resolves_injections_against_the_parent_instructions_of_its_path() {
    let child = bump_instruction(vec![]);
    let parent = InstructionNode {
        sub_instructions: vec![child],
        provides: vec![provide("counterBump", AccountBumpValueNode::new("counter"))],
        ..instruction("parent")
    };
    let path = NodePath::from(vec![
        NodeRef::from(&parent),
        NodeRef::from(&parent.sub_instructions[0]),
    ]);
    let inputs = resolve(&path, &LinkableDictionary::default());
    assert_eq!(
        field_at(&inputs, "bump").resolved_default_value,
        Some(AccountBumpValueNode::new("counter").into())
    );
}

#[test]
fn lets_an_instruction_shadow_a_key_provided_by_its_parent() {
    let child = InstructionNode {
        accounts: vec![
            account("inner", IsSigner::False),
            account("outer", IsSigner::False),
        ],
        data: Box::new(Some(
            StructTypeNode::new(vec![field_defaulting(
                "target",
                PublicKeyTypeNode::new(),
                InjectedValueNode::new("target"),
            )])
            .into(),
        )),
        provides: vec![provide("target", AccountValueNode::new("inner"))],
        ..instruction("child")
    };
    let parent = InstructionNode {
        sub_instructions: vec![child],
        provides: vec![provide("target", AccountValueNode::new("outer"))],
        ..instruction("parent")
    };
    let path = NodePath::from(vec![
        NodeRef::from(&parent),
        NodeRef::from(&parent.sub_instructions[0]),
    ]);
    let inputs = resolve(&path, &LinkableDictionary::default());
    assert_eq!(
        field_at(&inputs, "target").depends_on,
        [depends_on_account("inner")]
    );
}

#[test]
fn collects_data_fields_through_a_defined_type_link() {
    let program = ProgramNode {
        defined_types: vec![DefinedTypeNode::new(
            "counterArgs",
            StructTypeNode::new(vec![field_defaulting(
                "bump",
                IntegerTypeNode::le(U8),
                InjectedValueNode::new("counterBump"),
            )]),
        )],
        instructions: vec![InstructionNode {
            accounts: vec![InstructionAccountNode::new("counter", true, false)],
            data: Box::new(Some(DefinedTypeLinkNode::new("counterArgs").into())),
            provides: vec![provide("counterBump", AccountBumpValueNode::new("counter"))],
            ..instruction("increment")
        }],
        ..ProgramNode::new("counterProgram", "1111")
    };
    let linkables = get_linkables_from_node(&program);
    let path = NodePath::from(vec![
        NodeRef::from(&program),
        NodeRef::from(&program.instructions[0]),
    ]);
    let inputs = resolve(&path, &linkables);
    assert_eq!(
        field_at(&inputs, "bump").resolved_default_value,
        Some(AccountBumpValueNode::new("counter").into())
    );
    assert!(as_account(&inputs[0]).is_pda);
}

#[test]
fn falls_back_to_the_injected_value_fallback_when_no_provider_is_found() {
    let fallback = PublicKeyValueNode::new("11111111111111111111111111111111");
    let node = InstructionNode {
        data: Box::new(Some(
            StructTypeNode::new(vec![field_defaulting(
                "signer",
                PublicKeyTypeNode::new(),
                InjectedValueNode::with_fallback("missing", fallback.clone()),
            )])
            .into(),
        )),
        ..instruction("myInstruction")
    };
    let path = NodePath::from(vec![NodeRef::from(&node)]);
    let options = ResolveInstructionInputsOptions {
        include_data_value_nodes: true,
    };
    let inputs =
        get_resolved_instruction_inputs(&path, &LinkableDictionary::default(), options).unwrap();
    let signer = field_at(&inputs, "signer");
    assert_eq!(signer.depends_on, []);
    assert_eq!(signer.resolved_default_value, Some(fallback.into()));

    // Without the option, a plain value has nothing to resolve.
    assert_eq!(resolve_alone(&node), []);
}

fn resolve_error<'a>(
    path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> codama_errors::CodamaError {
    get_resolved_instruction_inputs(path, linkables, Default::default()).unwrap_err()
}

#[test]
fn reports_invalid_dependencies_by_the_kind_and_name_of_what_they_refer_to() {
    let node = InstructionNode {
        accounts: vec![defaulting(
            "owner",
            IsSigner::False,
            AccountValueNode::new("missing"),
        )],
        ..instruction("myInstruction")
    };
    let error = resolve_error(
        &NodePath::from(vec![NodeRef::from(&node)]),
        &LinkableDictionary::default(),
    );
    let codama_errors::CodamaError::InvalidInstructionDefaultValueDependency {
        dependency_kind,
        dependency_name,
        dependent,
        path,
    } = &error
    else {
        panic!("expected an invalid dependency, got {error}");
    };
    assert_eq!(
        (
            dependency_kind.as_str(),
            dependency_name.as_str(),
            dependent.as_str(),
            path.as_str()
        ),
        (
            "accountValueNode",
            "missing",
            "owner",
            "[instructionNode]myInstruction"
        )
    );
    assert_eq!(
        error.to_string(),
        "Dependency [missing] of kind [accountValueNode] is not a valid dependency of [owner] in instruction [instructionNode]myInstruction."
    );
}

/// A program whose instruction data links to a `config` type, whose `fee` field injects a key that the
/// instruction provides.
fn config_program(provides: Vec<ProvidedNode>, accounts: Vec<InstructionAccountNode>) -> RootNode {
    RootNode::new(ProgramNode {
        defined_types: vec![DefinedTypeNode::new(
            "config",
            StructTypeNode::new(vec![field_defaulting(
                "fee",
                PublicKeyTypeNode::new(),
                InjectedValueNode::new("fee"),
            )]),
        )],
        instructions: vec![InstructionNode {
            accounts,
            data: Box::new(Some(
                StructTypeNode::new(vec![field("config", DefinedTypeLinkNode::new("config"))])
                    .into(),
            )),
            provides,
            ..instruction("myInstruction")
        }],
        ..ProgramNode::new("myProgram", "1111")
    })
}

fn instruction_path(root: &RootNode) -> NodePath<'_> {
    NodePath::from(vec![
        NodeRef::from(root),
        NodeRef::from(&root.program),
        NodeRef::from(&root.program.instructions[0]),
    ])
}

#[test]
fn reports_invalid_dependencies_of_data_fields_within_linked_types_with_the_path_of_their_instruction(
) {
    let root = config_program(
        vec![provide("fee", AccountValueNode::new("missing"))],
        vec![],
    );
    let error = resolve_error(&instruction_path(&root), &get_linkables_from_node(&root));
    assert_eq!(
        error.to_string(),
        "Dependency [missing] of kind [accountValueNode] is not a valid dependency of [config.fee] in instruction [programNode]myProgram.[instructionNode]myInstruction."
    );
}

#[test]
fn reports_cycles_between_accounts_with_the_paths_of_the_accounts() {
    let node = InstructionNode {
        accounts: vec![
            defaulting("a", IsSigner::False, AccountValueNode::new("b")),
            defaulting("b", IsSigner::False, AccountValueNode::new("a")),
        ],
        ..instruction("myInstruction")
    };
    let error = resolve_error(
        &NodePath::from(vec![NodeRef::from(&node)]),
        &LinkableDictionary::default(),
    );
    assert_eq!(
        error.to_string(),
        "Circular dependency detected when resolving the default values of instruction [instructionNode]myInstruction. Got the following dependency cycle [a -> b -> a]."
    );
    let codama_errors::CodamaError::CyclicDependencyDetectedWhenResolvingInstructionDefaultValues {
        cycle,
        ..
    } = error
    else {
        panic!("expected a cycle");
    };
    assert_eq!(
        cycle,
        [
            "[instructionNode]myInstruction.[instructionAccountNode]a",
            "[instructionNode]myInstruction.[instructionAccountNode]b",
            "[instructionNode]myInstruction.[instructionAccountNode]a",
        ]
    );
}

#[test]
fn reports_cycles_through_data_fields_of_linked_types_with_the_paths_of_the_fields() {
    let authority = InstructionAccountNode {
        default_value: Box::new(Some(
            ConditionalValueNode {
                condition: Box::new(
                    DataValueNode::new("config.fee".parse::<codama_nodes::Path>().unwrap()).into(),
                ),
                value: Box::new(None),
                if_true: Box::new(Some(
                    PublicKeyValueNode::new("11111111111111111111111111111111").into(),
                )),
                if_false: Box::new(None),
                plugins: vec![],
            }
            .into(),
        )),
        ..account("authority", IsSigner::False)
    };
    let root = config_program(
        vec![provide("fee", AccountValueNode::new("authority"))],
        vec![authority],
    );
    let error = resolve_error(&instruction_path(&root), &get_linkables_from_node(&root));
    let codama_errors::CodamaError::CyclicDependencyDetectedWhenResolvingInstructionDefaultValues {
        formatted_cycle,
        cycle,
        ..
    } = error
    else {
        panic!("expected a cycle, got {error}");
    };
    assert_eq!(formatted_cycle, "authority -> config.fee -> authority");
    assert_eq!(
        cycle,
        [
            "[programNode]myProgram.[instructionNode]myInstruction.[instructionAccountNode]authority",
            "[programNode]myProgram.[definedTypeNode]config.[structTypeNode].[structFieldTypeNode]fee",
            "[programNode]myProgram.[instructionNode]myInstruction.[instructionAccountNode]authority",
        ]
    );
}

#[test]
fn fails_when_a_pda_value_takes_an_optional_account_as_a_seed() {
    let node = InstructionNode {
        accounts: vec![
            defaulting(
                "counter",
                IsSigner::False,
                PdaValueNode::new(
                    PdaLinkNode::new("counter"),
                    vec![PdaSeedValueNode::new(
                        "authority",
                        AccountValueNode::new("authority"),
                    )],
                ),
            ),
            InstructionAccountNode {
                is_optional: Some(true),
                ..account("authority", IsSigner::False)
            },
        ],
        ..instruction("myInstruction")
    };
    let error = resolve_error(
        &NodePath::from(vec![NodeRef::from(&node)]),
        &LinkableDictionary::default(),
    );
    assert_eq!(
        error.to_string(),
        "Cannot use optional account [authority] as PDA seed [instructionNode]myInstruction.[instructionAccountNode]counter.[pdaValueNode].[pdaSeedValueNode]authority."
    );
}

#[test]
fn keeps_the_account_in_the_path_of_an_optional_seed_when_another_seed_is_injected() {
    // The PDA value is the account's own, so its path starts at the account even though resolving its
    // injected seed rebuilds it. Only a PDA value injected as a whole starts at the PDA value.
    let node = InstructionNode {
        accounts: vec![
            defaulting(
                "counter",
                IsSigner::False,
                PdaValueNode::new(
                    PdaLinkNode::new("counter"),
                    vec![
                        PdaSeedValueNode::new("authority", AccountValueNode::new("authority")),
                        PdaSeedValueNode::new("bump", InjectedValueNode::new("bump")),
                    ],
                ),
            ),
            InstructionAccountNode {
                is_optional: Some(true),
                ..account("authority", IsSigner::False)
            },
        ],
        provides: vec![provide("bump", IntegerValueNode::new(255u8))],
        ..instruction("myInstruction")
    };
    let error = resolve_error(
        &NodePath::from(vec![NodeRef::from(&node)]),
        &LinkableDictionary::default(),
    );
    assert_eq!(
        error.to_string(),
        "Cannot use optional account [authority] as PDA seed [instructionNode]myInstruction.[instructionAccountNode]counter.[pdaValueNode].[pdaSeedValueNode]authority."
    );
}

// --- getInstructionDataFields ---

fn data_paths<'a>(path: &NodePath<'a>, linkables: &LinkableDictionary<'a>) -> Vec<String> {
    get_instruction_data_fields(path, linkables)
        .iter()
        .map(|field| field.path.to_string())
        .collect()
}

fn program_with(instruction: InstructionNode, defined_types: Vec<DefinedTypeNode>) -> ProgramNode {
    ProgramNode {
        defined_types,
        instructions: vec![instruction],
        ..ProgramNode::new("myProgram", "1111")
    }
}

fn program_instruction_path(program: &ProgramNode) -> NodePath<'_> {
    NodePath::from(vec![
        NodeRef::from(program),
        NodeRef::from(&program.instructions[0]),
    ])
}

fn with_data(data: impl Into<TypeNode>) -> InstructionNode {
    InstructionNode {
        data: Box::new(Some(data.into())),
        ..instruction("myInstruction")
    }
}

#[test]
fn lists_nested_data_fields_with_their_paths_following_links() {
    let program = program_with(
        with_data(StructTypeNode::new(vec![
            field("amount", IntegerTypeNode::le(U64)),
            field(
                "config",
                StructTypeNode::new(vec![field("fee", IntegerTypeNode::le(U16))]),
            ),
            field("extra", DefinedTypeLinkNode::new("extra")),
        ])),
        vec![DefinedTypeNode::new(
            "extra",
            StructTypeNode::new(vec![field("flag", IntegerTypeNode::le(U8))]),
        )],
    );
    let linkables = get_linkables_from_node(&program);
    assert_eq!(
        data_paths(&program_instruction_path(&program), &linkables),
        ["amount", "config", "config.fee", "extra", "extra.flag"]
    );
}

#[test]
fn does_not_follow_cycles_but_follows_a_defined_type_once_per_branch() {
    let program = program_with(
        with_data(DefinedTypeLinkNode::new("node")),
        vec![DefinedTypeNode::new(
            "node",
            StructTypeNode::new(vec![field("next", DefinedTypeLinkNode::new("node"))]),
        )],
    );
    let linkables = get_linkables_from_node(&program);
    assert_eq!(
        data_paths(&program_instruction_path(&program), &linkables),
        ["next"]
    );

    let program = program_with(
        with_data(StructTypeNode::new(vec![
            field("a", DefinedTypeLinkNode::new("config")),
            field("b", DefinedTypeLinkNode::new("config")),
        ])),
        vec![DefinedTypeNode::new(
            "config",
            StructTypeNode::new(vec![field("fee", IntegerTypeNode::le(U16))]),
        )],
    );
    let linkables = get_linkables_from_node(&program);
    assert_eq!(
        data_paths(&program_instruction_path(&program), &linkables),
        ["a", "a.fee", "b", "b.fee"]
    );
}

#[test]
fn follows_same_named_defined_types_from_different_programs() {
    let root = RootNode {
        additional_programs: vec![ProgramNode {
            defined_types: vec![DefinedTypeNode::new(
                "args",
                StructTypeNode::new(vec![field("amount", IntegerTypeNode::le(U64))]),
            )],
            ..ProgramNode::new("programB", "2222")
        }],
        ..RootNode::new(ProgramNode {
            defined_types: vec![DefinedTypeNode::new(
                "args",
                StructTypeNode::new(vec![field(
                    "remote",
                    DefinedTypeLinkNode::new_from_program("args", ProgramLinkNode::new("programB")),
                )]),
            )],
            instructions: vec![with_data(DefinedTypeLinkNode::new("args"))],
            ..ProgramNode::new("programA", "1111")
        })
    };
    let linkables = get_linkables_from_node(&root);
    assert_eq!(
        data_paths(&instruction_path(&root), &linkables),
        ["remote", "remote.amount"]
    );
}

#[test]
fn returns_the_node_path_of_each_data_field_through_linked_defined_types() {
    let root = RootNode::new(program_with(
        with_data(StructTypeNode::new(vec![
            field(
                "config",
                StructTypeNode::new(vec![field("fee", IntegerTypeNode::le(U16))]),
            ),
            field("extra", DefinedTypeLinkNode::new("extra")),
        ])),
        vec![DefinedTypeNode::new(
            "extra",
            StructTypeNode::new(vec![field("flag", IntegerTypeNode::le(U8))]),
        )],
    ));
    let linkables = get_linkables_from_node(&root);
    let field_paths: Vec<String> =
        get_instruction_data_fields(&instruction_path(&root), &linkables)
            .iter()
            .map(|field| field.field_path.to_string())
            .collect();
    let data = "[programNode]myProgram.[instructionNode]myInstruction.[structTypeNode]";
    assert_eq!(
        field_paths,
        [
            format!("{data}.[structFieldTypeNode]config"),
            format!("{data}.[structFieldTypeNode]config.[structTypeNode].[structFieldTypeNode]fee"),
            format!("{data}.[structFieldTypeNode]extra"),
            "[programNode]myProgram.[definedTypeNode]extra.[structTypeNode].[structFieldTypeNode]flag".to_string(),
        ]
    );
}
