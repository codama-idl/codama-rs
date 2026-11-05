use codama_nodes::test_utils::id;
use codama_nodes::{
    AccountNode, ArrayTypeNode, BooleanTypeNode, ConstantNode, ConstantPdaSeedNode,
    ConstantValueNode, DefinedTypeNode, EnumTypeNode, EnumVariantTypeNode, ErrorNode, EventNode,
    FixedCountNode, HasTransforms, HiddenPrefixTransformNode, Identifier, InstructionAccountNode,
    InstructionNode, InstructionRemainingAccountsNode, IntegerTypeNode, IntegerValueNode, NodePath,
    NodeVisitor, PdaNode, PdaSeedValueNode, PdaValueNode, ProgramNode, ProvidedNode, RootNode,
    StructFieldTypeNode, StructFieldValueNode, StructTypeNode, StructValueNode, TypeNode,
    ValidateIdentifierCollisionVisitor, VariablePdaSeedNode, U8,
};

fn collision_errors(root: &RootNode) -> Vec<String> {
    let mut visitor = ValidateIdentifierCollisionVisitor::new();
    visitor.visit_root(root, &NodePath::default());
    visitor.errors
}

fn program(identifier: &str) -> ProgramNode {
    ProgramNode::new(id(identifier), "1111")
}

fn integer() -> IntegerTypeNode {
    IntegerTypeNode::le(U8)
}

fn fields(first: &str, second: &str) -> StructTypeNode {
    StructTypeNode::new(vec![
        StructFieldTypeNode::new(id(first), integer()),
        StructFieldTypeNode::new(id(second), integer()),
    ])
}

fn instruction(identifier: &str) -> InstructionNode {
    InstructionNode {
        identifier: id(identifier),
        ..InstructionNode::default()
    }
}

fn parent(identifier: &str, sub_instructions: Vec<InstructionNode>) -> InstructionNode {
    InstructionNode {
        sub_instructions,
        ..instruction(identifier)
    }
}

fn errors_in_program(program: ProgramNode) -> Vec<String> {
    collision_errors(&RootNode::new(program))
}

fn errors_in_type(r#type: impl Into<TypeNode>) -> Vec<String> {
    errors_in_program(program("test").add_defined_type(DefinedTypeNode::new(id("config"), r#type)))
}

fn not_unique_message(label: &str, identifier: &str, path: &str) -> String {
    format!("{label} identifier \"{identifier}\" is not unique in {path}.")
}

fn collides_with_message(label: &str, identifier: &str, existing: &str, path: &str) -> String {
    format!("{label} identifier \"{identifier}\" collides with \"{existing}\" in {path}.")
}

fn not_unique_across_ixs_message(identifier: &str, path: &str) -> String {
    format!(
        "Instruction identifier \"{identifier}\" in {path} is not unique across the instructions and sub-instructions of program \"test\"."
    )
}

fn collides_across_ixs_message(identifier: &str, existing: &str, path: &str) -> String {
    format!(
        "Instruction identifier \"{identifier}\" in {path} collides with \"{existing}\" across the instructions and sub-instructions of program \"test\"."
    )
}

#[test]
fn programs_of_the_root() {
    let root = RootNode::new(program("spl_token"))
        .add_program(program("splToken"))
        .add_program(program("spl_token"));
    assert_eq!(
        collision_errors(&root),
        [
            "Program identifier \"splToken\" collides with \"spl_token\".",
            "Program identifier \"spl_token\" is not unique.",
        ]
    );
}

#[test]
fn sibling_sets_of_a_program() {
    type AddNode = fn(ProgramNode, &str) -> ProgramNode;
    let sets: [(&str, AddNode); 7] = [
        ("Account", |program, identifier| {
            program.add_account(AccountNode::new(
                id(identifier),
                StructTypeNode::new(vec![]),
            ))
        }),
        ("Instruction", |program, identifier| {
            program.add_instruction(instruction(identifier))
        }),
        ("Defined type", |program, identifier| {
            program.add_defined_type(DefinedTypeNode::new(id(identifier), integer()))
        }),
        ("PDA", |program, identifier| {
            program.add_pda(PdaNode::new(id(identifier), vec![]))
        }),
        ("Event", |program, identifier| {
            program.add_event(EventNode::new(id(identifier), StructTypeNode::new(vec![])))
        }),
        ("Error", |program, identifier| {
            program.add_error(ErrorNode::new(id(identifier), 0, "message"))
        }),
        ("Constant", |program, identifier| {
            let value = IntegerValueNode::new(1u8);
            program.add_constant(ConstantNode::new(id(identifier), integer(), value))
        }),
    ];
    for (label, add) in sets {
        assert_eq!(
            errors_in_program(add(add(program("test"), "token"), "token")),
            [not_unique_message(label, "token", "program \"test\"")]
        );
        assert_eq!(
            errors_in_program(add(add(program("test"), "token"), "Token")),
            [collides_with_message(
                label,
                "Token",
                "token",
                "program \"test\""
            )]
        );
    }
}

#[test]
fn sibling_sets_of_an_instruction() {
    type SetNodes = fn(&[&str]) -> InstructionNode;
    let sets: [(&str, SetNodes); 4] = [
        ("Instruction account", |identifiers| InstructionNode {
            accounts: identifiers
                .iter()
                .map(|identifier| InstructionAccountNode::new(id(identifier), false, false))
                .collect(),
            ..instruction("transfer")
        }),
        ("Instruction remaining accounts", |identifiers| {
            InstructionNode {
                remaining_accounts: identifiers
                    .iter()
                    .map(|identifier| InstructionRemainingAccountsNode {
                        identifier: id(identifier),
                        ..InstructionRemainingAccountsNode::default()
                    })
                    .collect(),
                ..instruction("transfer")
            }
        }),
        ("Provided value", |identifiers| InstructionNode {
            provides: identifiers
                .iter()
                .map(|identifier| ProvidedNode::new(id(identifier), IntegerValueNode::new(1u8)))
                .collect(),
            ..instruction("transfer")
        }),
        ("Sub-instruction", |identifiers| {
            parent(
                "transfer",
                identifiers.iter().map(|i| instruction(i)).collect(),
            )
        }),
    ];
    let path = "program \"test\" > instruction \"transfer\"";
    for (label, set) in sets {
        assert_eq!(
            errors_in_program(program("test").add_instruction(set(&["owner", "owner"]))),
            [not_unique_message(label, "owner", path)]
        );
        assert_eq!(
            errors_in_program(program("test").add_instruction(set(&["ownerKey", "owner_key"]))),
            [collides_with_message(label, "owner_key", "ownerKey", path)]
        );
    }
}

#[test]
fn variable_seeds_of_a_pda() {
    let pda = |first: &str, second: &str| {
        PdaNode::new(
            id("vault"),
            vec![
                VariablePdaSeedNode::new(id(first), integer()).into(),
                ConstantPdaSeedNode::new(integer(), IntegerValueNode::new(1u8)).into(),
                VariablePdaSeedNode::new(id(second), integer()).into(),
            ],
        )
    };
    let inline = |pda: PdaNode| InstructionNode {
        accounts: vec![InstructionAccountNode {
            default_value: Box::new(Some(PdaValueNode::new(pda, vec![]).into())),
            ..InstructionAccountNode::new(id("vault"), false, false)
        }],
        ..instruction("transfer")
    };
    let path = "program \"test\" > pda \"vault\"";
    let inline_path =
        "program \"test\" > instruction \"transfer\" > instructionAccount \"vault\" > defaultValue > pda \"vault\"";
    let cases = [
        (
            program("test").add_pda(pda("owner", "owner")),
            not_unique_message("PDA seed", "owner", path),
        ),
        (
            program("test").add_pda(pda("owner", "Owner")),
            collides_with_message("PDA seed", "Owner", "owner", path),
        ),
        (
            program("test").add_instruction(inline(pda("owner", "owner"))),
            not_unique_message("PDA seed", "owner", inline_path),
        ),
    ];
    for (program, expected) in cases {
        assert_eq!(errors_in_program(program), [expected]);
    }
}

#[test]
fn fields_and_variants_of_a_type() {
    let variants = |first: &str, second: &str| {
        EnumTypeNode::new(vec![
            EnumVariantTypeNode::new(id(first)),
            EnumVariantTypeNode::new(id(second)),
        ])
    };
    let path = "program \"test\" > definedType \"config\" > type";
    let cases: [(TypeNode, String); 4] = [
        (
            fields("owner", "owner").into(),
            not_unique_message("Struct field", "owner", path),
        ),
        (
            fields("fooBar", "foo_bar").into(),
            collides_with_message("Struct field", "foo_bar", "fooBar", path),
        ),
        (
            variants("quit", "quit").into(),
            not_unique_message("Enum variant", "quit", path),
        ),
        (
            variants("getURL", "get_url").into(),
            collides_with_message("Enum variant", "get_url", "getURL", path),
        ),
    ];
    for (r#type, expected) in cases {
        assert_eq!(errors_in_type(r#type), [expected]);
    }
}

#[test]
fn spec_casing_collision_pairs() {
    let path = "program \"test\" > definedType \"config\" > type";
    let collides_with_message_pairs = [
        ("fooBar", "foo_bar"),
        ("foo1", "foo_1"),
        ("getURL", "get_url"),
        ("HTTPServer", "httpServer"),
        ("MAX_SUPPLY", "maxSupply"),
        ("_foo", "foo"),
        ("foo__bar", "foo_bar"),
        ("foo1Bar", "foo1_bar"),
    ];
    for (first, second) in collides_with_message_pairs {
        assert_eq!(
            errors_in_type(fields(first, second)),
            [collides_with_message("Struct field", second, first, path)]
        );
    }
    let distinct_pairs = [
        ("foo_dart", "food_art"),
        ("group__sub_group__name", "group_subgroup_name"),
        ("foo1bar", "foo1_bar"),
        ("HTTPServer", "hTTPServer"),
    ];
    for (first, second) in distinct_pairs {
        assert_eq!(errors_in_type(fields(first, second)), Vec::<String>::new());
    }
}

#[test]
fn struct_types_in_every_position() {
    let data = || fields("owner", "owner");
    let in_type = |r#type: TypeNode| {
        program("test").add_defined_type(DefinedTypeNode::new(id("config"), r#type))
    };
    let cases: [(ProgramNode, &str); 11] = [
        (
            program("test").add_account(AccountNode::new(id("vault"), data())),
            "account \"vault\" > data",
        ),
        (
            program("test").add_event(EventNode::new(id("deposited"), data())),
            "event \"deposited\" > data",
        ),
        (
            program("test").add_instruction(InstructionNode {
                data: Box::new(Some(data().into())),
                ..instruction("transfer")
            }),
            "instruction \"transfer\" > data",
        ),
        (
            program("test").add_instruction(InstructionNode {
                return_data: Box::new(Some(data().into())),
                ..instruction("transfer")
            }),
            "instruction \"transfer\" > returnData",
        ),
        (
            program("test").add_instruction(InstructionNode {
                provides: vec![ProvidedNode::new(id("config"), TypeNode::from(data()))],
                ..instruction("transfer")
            }),
            "instruction \"transfer\" > provided \"config\" > node",
        ),
        (
            program("test").add_constant(ConstantNode::new(
                id("config"),
                data(),
                StructValueNode::new(vec![]),
            )),
            "constant \"config\" > type",
        ),
        (
            program("test").add_pda(PdaNode::new(
                id("vault"),
                vec![VariablePdaSeedNode::new(id("seed"), data()).into()],
            )),
            "pda \"vault\" > variablePdaSeed \"seed\" > type",
        ),
        (
            in_type(
                StructTypeNode::new(vec![StructFieldTypeNode::new(id("inner"), data())]).into(),
            ),
            "definedType \"config\" > type > structFieldType \"inner\" > type",
        ),
        (
            in_type(ArrayTypeNode::new(data(), FixedCountNode::new(2)).into()),
            "definedType \"config\" > type > item",
        ),
        (
            in_type(
                EnumTypeNode::new(vec![EnumVariantTypeNode::with_data(id("move"), data())]).into(),
            ),
            "definedType \"config\" > type > enumVariantType \"move\" > data",
        ),
        (
            in_type(
                BooleanTypeNode::default()
                    .add_transform(HiddenPrefixTransformNode::new(vec![
                        ConstantValueNode::new(data(), StructValueNode::new(vec![])),
                    ]))
                    .into(),
            ),
            "definedType \"config\" > type > transforms[0] > prefix[0] > type",
        ),
    ];
    for (program, path) in cases {
        let path = format!("program \"test\" > {path}");
        assert_eq!(
            errors_in_program(program),
            [not_unique_message("Struct field", "owner", &path)]
        );
    }
}

#[test]
fn struct_values_and_pda_seed_values_are_not_checked() {
    let struct_value = StructValueNode::new(vec![
        StructFieldValueNode::new(id("owner"), IntegerValueNode::new(1u8)),
        StructFieldValueNode::new(id("owner"), IntegerValueNode::new(2u8)),
    ]);
    let pda_value = PdaValueNode::new(
        PdaNode::new(id("vault"), vec![]),
        vec![
            PdaSeedValueNode::new(id("owner"), IntegerValueNode::new(1u8)),
            PdaSeedValueNode::new(id("owner"), IntegerValueNode::new(2u8)),
        ],
    );
    let program = program("test")
        .add_constant(ConstantNode::new(id("config"), integer(), struct_value))
        .add_instruction(InstructionNode {
            accounts: vec![InstructionAccountNode {
                default_value: Box::new(Some(pda_value.into())),
                ..InstructionAccountNode::new(id("vault"), false, false)
            }],
            ..instruction("transfer")
        });
    assert_eq!(errors_in_program(program), Vec::<String>::new());
}

#[test]
fn empty_identifiers_are_skipped() {
    let r#type = StructTypeNode::new(vec![
        StructFieldTypeNode::new(Identifier::default(), integer()),
        StructFieldTypeNode::new(Identifier::default(), integer()),
    ]);
    assert_eq!(errors_in_type(r#type), Vec::<String>::new());
}

#[test]
fn instructions_across_sub_instructions() {
    let one_program = |program: ProgramNode| RootNode::new(program);
    let cases: [(RootNode, Vec<String>); 10] = [
        (
            one_program(
                program("test")
                    .add_instruction(parent("firstParent", vec![instruction("shared")]))
                    .add_instruction(parent("secondParent", vec![instruction("shared")])),
            ),
            vec![not_unique_across_ixs_message(
                "shared",
                "program \"test\" > instruction \"secondParent\"",
            )],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(instruction("shared"))
                    .add_instruction(parent("parent", vec![instruction("shared")])),
            ),
            vec![not_unique_across_ixs_message(
                "shared",
                "program \"test\" > instruction \"parent\"",
            )],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(parent("parent", vec![instruction("shared")]))
                    .add_instruction(instruction("shared")),
            ),
            vec![not_unique_across_ixs_message("shared", "program \"test\"")],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(instruction("shared"))
                    .add_instruction(parent(
                        "parent",
                        vec![parent("child", vec![instruction("shared")])],
                    )),
            ),
            vec![not_unique_across_ixs_message(
                "shared",
                "program \"test\" > instruction \"parent\" > instruction \"child\"",
            )],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(parent(
                        "parent",
                        vec![parent("child", vec![instruction("grandchild")])],
                    ))
                    .add_instruction(parent("otherParent", vec![instruction("otherChild")])),
            ),
            vec![],
        ),
        (
            RootNode::new(program("first").add_instruction(instruction("shared"))).add_program(
                program("second").add_instruction(parent("parent", vec![instruction("shared")])),
            ),
            vec![],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(instruction("topLevel"))
                    .add_instruction(instruction("topLevel"))
                    .add_instruction(parent(
                        "parent",
                        vec![instruction("child"), instruction("child")],
                    )),
            ),
            vec![
                not_unique_message("Instruction", "topLevel", "program \"test\""),
                not_unique_message(
                    "Sub-instruction",
                    "child",
                    "program \"test\" > instruction \"parent\"",
                ),
            ],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(instruction("shared"))
                    .add_instruction(instruction("shared"))
                    .add_instruction(parent("parent", vec![instruction("shared")])),
            ),
            vec![
                not_unique_message("Instruction", "shared", "program \"test\""),
                not_unique_across_ixs_message(
                    "shared",
                    "program \"test\" > instruction \"parent\"",
                ),
            ],
        ),
        (
            one_program(
                program("test")
                    .add_instruction(instruction("transferTokens"))
                    .add_instruction(parent("parent", vec![instruction("transfer_tokens")])),
            ),
            vec![collides_across_ixs_message(
                "transfer_tokens",
                "transferTokens",
                "program \"test\" > instruction \"parent\"",
            )],
        ),
        (
            one_program(program("test").add_instruction(parent(
                "parent",
                vec![
                    instruction("transferTokens"),
                    instruction("transfer_tokens"),
                ],
            ))),
            vec![collides_with_message(
                "Sub-instruction",
                "transfer_tokens",
                "transferTokens",
                "program \"test\" > instruction \"parent\"",
            )],
        ),
    ];
    for (root, expected) in cases {
        assert_eq!(collision_errors(&root), expected);
    }
}

#[test]
fn programs_without_an_identifier() {
    let fix = "Set one with `codama_program!(identifier = \"...\")` or `#[codama(program(identifier = \"...\", address = \"...\"))]`.";
    let cases = [
        (
            RootNode::new(ProgramNode::default()),
            vec![format!("The program has no identifier. {fix}")],
        ),
        (
            RootNode::new(ProgramNode::new(Identifier::default(), "1111")),
            vec![format!(
                "The program with public key \"1111\" has no identifier. {fix}"
            )],
        ),
        (
            RootNode::new(program("token"))
                .add_program(ProgramNode::new(Identifier::default(), "2222"))
                .add_program(ProgramNode::new(Identifier::default(), "3333")),
            vec![
                format!("The program with public key \"2222\" has no identifier. {fix}"),
                format!("The program with public key \"3333\" has no identifier. {fix}"),
            ],
        ),
    ];
    for (root, expected) in cases {
        assert_eq!(collision_errors(&root), expected);
    }
}
