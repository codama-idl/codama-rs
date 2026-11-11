//! The validation items of `get_validation_items_visitor`, ported from the tests of `@codama/validators`.

use codama_nodes::{
    AccountLinkNode, AccountNode, DateTimeTypeNode, DefinedTypeLinkNode, DefinedTypeNode,
    DurationTypeNode, EnumTypeNode, EnumVariantTypeNode, ErrorNode, FixedPointTypeNode, Identifier,
    InstructionAccountNode, InstructionLifecycle, InstructionNode, InstructionStatusNode,
    IntegerTypeNode, NodeRef, PluginNode, ProgramNode, PublicKeyTypeNode, RootNode,
    StructFieldTypeNode, StructTypeNode, Text, TextNode, TupleTypeNode, I64, U64,
};
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;

/// The level, message and path of each item of `node`.
fn items<'a>(node: impl Into<NodeRef<'a>>) -> Vec<(LogLevel, String, String)> {
    visit(&get_validation_items_visitor(), node)
        .into_iter()
        .map(|item| (item.level, item.message, item.path.to_string()))
        .collect()
}

fn error(message: &str, path: &str) -> (LogLevel, String, String) {
    (LogLevel::Error, message.to_string(), path.to_string())
}

fn program(identifier: &'static str) -> ProgramNode {
    ProgramNode::new(identifier, "1111")
}

fn instruction(identifier: &'static str) -> InstructionNode {
    InstructionNode {
        identifier: identifier.into(),
        ..InstructionNode::default()
    }
}

fn instruction_with(
    identifier: &'static str,
    sub_instructions: Vec<InstructionNode>,
) -> InstructionNode {
    InstructionNode {
        sub_instructions,
        ..instruction(identifier)
    }
}

fn field(identifier: &str) -> StructFieldTypeNode {
    StructFieldTypeNode::new(
        identifier.parse::<Identifier>().unwrap(),
        PublicKeyTypeNode::new(),
    )
}

#[test]
fn reports_no_cause_for_diagnostics_that_do_not_stem_from_errors() {
    let tuple = TupleTypeNode::new(vec![]);
    let items = visit(&get_validation_items_visitor(), &tuple);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].level, LogLevel::Warn);
    assert_eq!(items[0].message, "Tuple has no items.");
    assert!(items[0].cause.is_none());
}

#[test]
fn reports_instruction_accounts_without_identifiers_against_the_account() {
    let node = InstructionNode {
        accounts: vec![InstructionAccountNode::new(
            Identifier::default(),
            false,
            false,
        )],
        ..instruction("transfer")
    };
    assert_eq!(
        items(&node),
        [error(
            "Instruction account has no identifier.",
            "[instructionNode]transfer.[instructionAccountNode]"
        )]
    );
}

#[test]
fn validates_program_nodes() {
    let node = ProgramNode {
        identifier: Identifier::default(),
        public_key: String::new(),
        version: String::new(),
        ..ProgramNode::default()
    };
    assert_eq!(
        items(&node),
        [
            error("Program has no identifier.", "[programNode]"),
            error("Program has no public key.", "[programNode]"),
            (
                LogLevel::Warn,
                "Program has no version.".to_string(),
                "[programNode]".to_string()
            ),
        ]
    );
}

#[test]
fn validates_nested_nodes() {
    let node = TupleTypeNode::new(vec![
        TupleTypeNode::new(vec![]).into(),
        StructTypeNode::new(vec![field("owner"), field("owner")]).into(),
    ]);
    assert_eq!(
        items(&node),
        [
            (
                LogLevel::Warn,
                "Tuple has no items.".to_string(),
                "[tupleTypeNode].[tupleTypeNode]".to_string()
            ),
            error(
                "Struct field identifier \"owner\" is not unique.",
                "[tupleTypeNode].[structTypeNode].[structFieldTypeNode]owner"
            ),
        ]
    );
}

#[test]
fn validates_a_defined_type_link_nested_within_a_struct_field() {
    let node = ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new(
                "foo",
                StructTypeNode::new(vec![StructFieldTypeNode::new(
                    "bar",
                    DefinedTypeLinkNode::new("baz"),
                )]),
            ),
            DefinedTypeNode::new("baz", IntegerTypeNode::le(U64)),
        ],
        ..program("test")
    };
    assert_eq!(items(&node), []);
}

#[test]
fn reports_a_nested_defined_type_link_that_points_at_a_missing_type() {
    let node = ProgramNode {
        defined_types: vec![DefinedTypeNode::new(
            "foo",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "bar",
                DefinedTypeLinkNode::new("missing"),
            )]),
        )],
        ..program("test")
    };
    assert_eq!(
        items(&node),
        [error(
            "Pointing to a missing defined type named \"missing\"",
            "[programNode]test.[definedTypeNode]foo.[structTypeNode].[structFieldTypeNode]bar.[definedTypeLinkNode]missing"
        )]
    );
}

#[test]
fn reports_identifiers_colliding_once_converted_to_camel_case() {
    for (first, second) in [
        ("fooBar", "foo_bar"),
        ("foo1", "foo_1"),
        ("getURL", "get_url"),
        ("HTTPServer", "httpServer"),
        ("MAX_SUPPLY", "maxSupply"),
        ("_foo", "foo"),
        ("foo__bar", "foo_bar"),
        ("foo1Bar", "foo1_bar"),
    ] {
        let node = StructTypeNode::new(vec![field(first), field(second)]);
        assert_eq!(
            items(&node),
            [error(
                &format!("Struct field identifier \"{second}\" collides with \"{first}\" once converted to camelCase."),
                &format!("[structTypeNode].[structFieldTypeNode]{second}")
            )],
            "{first} and {second}"
        );
    }
}

#[test]
fn allows_identifiers_with_distinct_camel_case_forms() {
    for (first, second) in [
        ("foo_dart", "food_art"),
        ("group__sub_group__name", "group_subgroup_name"),
        ("foo1bar", "foo1_bar"),
        ("HTTPServer", "hTTPServer"),
    ] {
        let node = StructTypeNode::new(vec![field(first), field(second)]);
        assert_eq!(items(&node), [], "{first} and {second}");
    }
}

#[test]
fn reports_identifier_collisions_within_the_collections_of_a_program() {
    let node = ProgramNode {
        accounts: vec![
            AccountNode::new("token", StructTypeNode::new(vec![])),
            AccountNode::new("Token", StructTypeNode::new(vec![])),
        ],
        ..program("test")
    };
    assert_eq!(
        items(&node),
        [error(
            "Account identifier \"Token\" collides with \"token\" once converted to camelCase in program \"test\".",
            "[programNode]test.[accountNode]Token"
        )]
    );
}

#[test]
fn reports_identifier_collisions_between_the_programs_of_a_root() {
    let node = RootNode {
        additional_programs: vec![ProgramNode::new("splToken", "2222")],
        ..RootNode::new(program("spl_token"))
    };
    assert_eq!(
        items(&node),
        [error(
            "Program identifier \"splToken\" collides with \"spl_token\" once converted to camelCase.",
            "[programNode]splToken"
        )]
    );
}

#[test]
fn reports_duplicate_enum_variants_and_enums_without_variants() {
    let node = EnumTypeNode::new(vec![
        EnumVariantTypeNode::new("quit"),
        EnumVariantTypeNode::new("quit"),
    ]);
    assert_eq!(
        items(&node),
        [error(
            "Enum variant identifier \"quit\" is not unique.",
            "[enumTypeNode].[enumVariantTypeNode]quit"
        )]
    );
    assert_eq!(
        items(&EnumTypeNode::new(vec![])),
        [(
            LogLevel::Warn,
            "Enum has no variants.".to_string(),
            "[enumTypeNode]".to_string()
        )]
    );
}

#[test]
fn reports_duplicate_instruction_accounts() {
    let node = InstructionNode {
        accounts: vec![
            InstructionAccountNode::new("owner", false, true),
            InstructionAccountNode::new("owner", false, true),
        ],
        ..instruction("transfer")
    };
    assert_eq!(
        items(&node),
        [error(
            "Instruction account identifier \"owner\" is not unique in instruction \"transfer\".",
            "[instructionNode]transfer.[instructionAccountNode]owner"
        )]
    );
}

const ACROSS: &str = "across the instructions and sub-instructions of program \"test\".";

fn program_with(instructions: Vec<InstructionNode>) -> ProgramNode {
    ProgramNode {
        instructions,
        ..program("test")
    }
}

#[test]
fn reports_sub_instructions_sharing_an_identifier_under_different_parents() {
    let node = program_with(vec![
        instruction_with("firstParent", vec![instruction("shared")]),
        instruction_with("secondParent", vec![instruction("shared")]),
    ]);
    assert_eq!(
        items(&node),
        [error(
            &format!("Instruction identifier \"shared\" is not unique {ACROSS}"),
            "[programNode]test.[instructionNode]secondParent.[instructionNode]shared"
        )]
    );
}

#[test]
fn reports_sub_instructions_sharing_an_identifier_with_a_top_level_instruction() {
    let node = program_with(vec![
        instruction("shared"),
        instruction_with("parent", vec![instruction("shared")]),
    ]);
    assert_eq!(
        items(&node),
        [error(
            &format!("Instruction identifier \"shared\" is not unique {ACROSS}"),
            "[programNode]test.[instructionNode]parent.[instructionNode]shared"
        )]
    );
}

#[test]
fn reports_top_level_instructions_sharing_an_identifier_with_a_previous_sub_instruction() {
    let node = program_with(vec![
        instruction_with("parent", vec![instruction("shared")]),
        instruction("shared"),
    ]);
    assert_eq!(
        items(&node),
        [error(
            &format!("Instruction identifier \"shared\" is not unique {ACROSS}"),
            "[programNode]test.[instructionNode]shared"
        )]
    );
}

#[test]
fn reports_deeply_nested_sub_instructions_sharing_an_identifier() {
    let node = program_with(vec![
        instruction("shared"),
        instruction_with(
            "parent",
            vec![instruction_with("child", vec![instruction("shared")])],
        ),
    ]);
    assert_eq!(
        items(&node),
        [error(
            &format!("Instruction identifier \"shared\" is not unique {ACROSS}"),
            "[programNode]test.[instructionNode]parent.[instructionNode]child.[instructionNode]shared"
        )]
    );
}

#[test]
fn does_not_report_sub_instructions_with_distinct_identifiers() {
    let node = program_with(vec![
        instruction_with(
            "parent",
            vec![instruction_with("child", vec![instruction("grandchild")])],
        ),
        instruction_with("otherParent", vec![instruction("otherChild")]),
    ]);
    assert_eq!(items(&node), []);
}

#[test]
fn does_not_report_sub_instructions_sharing_an_identifier_across_programs() {
    let node = RootNode {
        additional_programs: vec![ProgramNode {
            instructions: vec![instruction_with("parent", vec![instruction("shared")])],
            ..ProgramNode::new("second", "2222")
        }],
        ..RootNode::new(ProgramNode {
            instructions: vec![instruction("shared")],
            ..program("first")
        })
    };
    assert_eq!(items(&node), []);
}

#[test]
fn does_not_report_sibling_instructions_sharing_an_identifier_twice() {
    let node = program_with(vec![
        instruction("topLevel"),
        instruction("topLevel"),
        instruction_with("parent", vec![instruction("child"), instruction("child")]),
    ]);
    assert_eq!(
        items(&node),
        [
            error(
                "Instruction identifier \"topLevel\" is not unique in program \"test\".",
                "[programNode]test.[instructionNode]topLevel"
            ),
            error(
                "Sub-instruction identifier \"child\" is not unique in instruction \"parent\".",
                "[programNode]test.[instructionNode]parent.[instructionNode]child"
            ),
        ]
    );
}

#[test]
fn reports_nested_duplicates_that_are_not_siblings_of_an_earlier_duplicate() {
    let node = program_with(vec![
        instruction("shared"),
        instruction("shared"),
        instruction_with("parent", vec![instruction("shared")]),
    ]);
    assert_eq!(
        items(&node),
        [
            error(
                "Instruction identifier \"shared\" is not unique in program \"test\".",
                "[programNode]test.[instructionNode]shared"
            ),
            error(
                &format!("Instruction identifier \"shared\" is not unique {ACROSS}"),
                "[programNode]test.[instructionNode]parent.[instructionNode]shared"
            ),
        ]
    );
}

#[test]
fn reports_sub_instructions_colliding_with_a_top_level_instruction_once_converted_to_camel_case() {
    let node = program_with(vec![
        instruction("transferTokens"),
        instruction_with("parent", vec![instruction("transfer_tokens")]),
    ]);
    assert_eq!(
        items(&node),
        [error(
            &format!("Instruction identifier \"transfer_tokens\" collides with \"transferTokens\" once converted to camelCase {ACROSS}"),
            "[programNode]test.[instructionNode]parent.[instructionNode]transfer_tokens"
        )]
    );
}

#[test]
fn does_not_report_sibling_instructions_colliding_once_converted_to_camel_case_twice() {
    let node = program_with(vec![instruction_with(
        "parent",
        vec![
            instruction("transferTokens"),
            instruction("transfer_tokens"),
        ],
    )]);
    assert_eq!(
        items(&node),
        [error(
            "Sub-instruction identifier \"transfer_tokens\" collides with \"transferTokens\" once converted to camelCase in instruction \"parent\".",
            "[programNode]test.[instructionNode]parent.[instructionNode]transfer_tokens"
        )]
    );
}

#[test]
fn reports_text_nodes_without_plugins() {
    let status = |plugins| InstructionStatusNode {
        message: Some(Text::Node(TextNode {
            content: "Use the new instruction.".to_string(),
            plugins,
        })),
        ..InstructionStatusNode::new(InstructionLifecycle::Deprecated)
    };
    assert_eq!(
        items(&status(vec![])),
        [(
            LogLevel::Info,
            "Text node has no plugins; use a plain string instead.".to_string(),
            "[instructionStatusNode].[textNode]".to_string()
        )]
    );
    let translated = vec![PluginNode::new(
        "i18n.es".parse::<codama_nodes::Namespace>().unwrap(),
    )];
    assert_eq!(items(&status(translated)), []);
}

#[test]
fn reports_errors_without_identifiers_or_messages() {
    let node = ErrorNode::new(Identifier::default(), 42, "");
    assert_eq!(
        items(&node),
        [
            error("Error has no identifier.", "[errorNode]"),
            (
                LogLevel::Warn,
                "Error has no message.".to_string(),
                "[errorNode]".to_string()
            ),
        ]
    );
}

#[test]
fn reports_links_outside_a_program_as_missing() {
    let node = DefinedTypeNode::new(
        "loop",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "next",
            DefinedTypeLinkNode::new("loop"),
        )]),
    );
    assert_eq!(
        items(&node),
        [error(
            "Pointing to a missing defined type named \"loop\"",
            "[definedTypeNode]loop.[structTypeNode].[structFieldTypeNode]next.[definedTypeLinkNode]loop"
        )]
    );
}

#[test]
fn reports_date_times_and_durations_without_ticks_per_second() {
    let date_time = DateTimeTypeNode {
        ticks_per_second: Some(0),
        ..DateTimeTypeNode::new(IntegerTypeNode::le(I64))
    };
    let items_of_date_time = visit(&get_validation_items_visitor(), &date_time);
    assert_eq!(items_of_date_time.len(), 1);
    let message =
        "Expected a positive integer number of ticks per second for [dateTimeTypeNode], got [0].";
    assert_eq!(items_of_date_time[0].message, message);
    assert_eq!(
        items_of_date_time[0].cause.as_ref().unwrap().to_string(),
        message
    );

    let duration = DurationTypeNode {
        ticks_per_second: Some(0),
        ..DurationTypeNode::new(IntegerTypeNode::le(U64))
    };
    assert_eq!(
        items(&duration),
        [error(
            "Expected a positive integer number of ticks per second for [durationTypeNode], got [0].",
            "[durationTypeNode]"
        )]
    );
}

#[test]
fn accepts_date_times_and_durations_with_positive_ticks_per_second() {
    let date_time = DateTimeTypeNode {
        ticks_per_second: Some(1000),
        ..DateTimeTypeNode::new(IntegerTypeNode::le(I64))
    };
    assert_eq!(items(&date_time), []);
    assert_eq!(items(&DurationTypeNode::new(IntegerTypeNode::le(U64))), []);
}

#[test]
fn reports_fixed_points_whose_scale_is_not_positive() {
    assert_eq!(
        items(&FixedPointTypeNode::new(IntegerTypeNode::le(U64), 0)),
        [error(
            "Fixed-point scale must be a positive integer, got 0.",
            "[fixedPointTypeNode]"
        )]
    );
    assert_eq!(
        items(&FixedPointTypeNode::new(IntegerTypeNode::le(U64), 6)),
        []
    );
}

#[test]
fn reports_account_links_pointing_to_missing_accounts() {
    let account = InstructionAccountNode {
        account_link: Some(AccountLinkNode::new("missing")),
        ..InstructionAccountNode::new("mint", false, false)
    };
    let root = RootNode::new(ProgramNode {
        instructions: vec![InstructionNode {
            accounts: vec![account],
            ..instruction("mintTo")
        }],
        ..program("myProgram")
    });
    assert_eq!(
        items(&root),
        [error(
            "Pointing to a missing account named \"missing\"",
            "[programNode]myProgram.[instructionNode]mintTo.[instructionAccountNode]mint.[accountLinkNode]missing"
        )]
    );
}

#[test]
fn records_the_links_of_each_tree_it_visits() {
    let with_config = ProgramNode {
        defined_types: vec![
            DefinedTypeNode::new("config", StructTypeNode::new(vec![])),
            DefinedTypeNode::new("wrapper", DefinedTypeLinkNode::new("config")),
        ],
        ..program("test")
    };
    let without_config = ProgramNode {
        defined_types: vec![DefinedTypeNode::new(
            "wrapper",
            DefinedTypeLinkNode::new("config"),
        )],
        ..program("test")
    };
    let visitor = get_validation_items_visitor();
    assert!(visit(&visitor, &with_config).is_empty());
    assert_eq!(visit(&visitor, &without_config).len(), 1);
}
