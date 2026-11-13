//! The validation items that resolve paths and contextual values, ported from the tests of
//! `@codama/validators`.

use codama_nodes::{
    AccountBumpValueNode, AccountDataValueNode, AccountLinkNode, AccountNode, AccountValueNode,
    ConditionalValueNode, ConstantValueNode, DataValueNode, DefinedTypeLinkNode, DefinedTypeNode,
    FieldDiscriminatorNode, InjectedValueNode, InstructionAccountNode, InstructionDisplayNode,
    InstructionInputValueNode, InstructionNode, IntegerTypeNode, IntegerValueNode, NodeRef,
    NoneValueNode, OptionTypeNode, PayerValueNode, ProgramNode, ProvidedNode, PublicKeyTypeNode,
    RootNode, SomeValueNode, StringValueNode, StructFieldDisplayNode, StructFieldTypeNode,
    StructTypeNode, Text, TextNode, TupleTypeNode, TypeNode, ValueLabelDisplayNode, ValueNode, I64,
    U64, U8,
};
use codama_validators::{get_validation_items_visitor, LogLevel};
use codama_visitors_core::visit;

/// The message, path and whether each item of `node` has a cause.
fn items<'a>(node: impl Into<NodeRef<'a>>) -> Vec<(String, String, bool)> {
    visit(&get_validation_items_visitor(), node)
        .into_iter()
        .inspect(|item| assert_eq!(item.level, LogLevel::Error, "{item}"))
        .map(|item| (item.message, item.path.to_string(), item.cause.is_some()))
        .collect()
}

fn error(message: &str, path: &str) -> (String, String, bool) {
    (message.to_string(), path.to_string(), false)
}

fn caused(message: &str, path: &str) -> (String, String, bool) {
    (message.to_string(), path.to_string(), true)
}

fn cannot_resolve(segment: &str, path_string: &str, path: &str) -> String {
    format!("Cannot resolve segment [{segment}] of path [{path_string}] from {path}.")
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

fn with_data(identifier: &'static str, data: impl Into<TypeNode>) -> InstructionNode {
    InstructionNode {
        data: Box::new(Some(data.into())),
        ..instruction(identifier)
    }
}

fn amount_struct() -> StructTypeNode {
    StructTypeNode::new(vec![StructFieldTypeNode::new(
        "amount",
        IntegerTypeNode::le(U64),
    )])
}

fn linked_public_key(account: &'static str) -> PublicKeyTypeNode {
    PublicKeyTypeNode {
        account_link: Some(AccountLinkNode::new(account)),
        ..PublicKeyTypeNode::new()
    }
}

fn mint_account() -> AccountNode {
    AccountNode::new(
        "mint",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "decimals",
            IntegerTypeNode::le(U8),
        )]),
    )
}

fn linked_mint() -> InstructionAccountNode {
    InstructionAccountNode {
        account_link: Some(AccountLinkNode::new("mint")),
        ..InstructionAccountNode::new("mint", false, false)
    }
}

fn provide(identifier: &'static str, value: impl Into<codama_nodes::Node>) -> ProvidedNode {
    ProvidedNode::new(identifier, value)
}

// --- Value labels ---

fn field_with_labels(
    identifier: &'static str,
    r#type: impl Into<TypeNode>,
    labels: Vec<ValueLabelDisplayNode>,
) -> StructFieldTypeNode {
    StructFieldTypeNode {
        display: Some(StructFieldDisplayNode {
            value_labels: labels,
            ..StructFieldDisplayNode::default()
        }),
        ..StructFieldTypeNode::new(identifier, r#type)
    }
}

fn label(value: impl Into<ValueNode>) -> ValueLabelDisplayNode {
    ValueLabelDisplayNode::new(value, "Label")
}

const LABEL_PATH: &str =
    "[structFieldTypeNode]amount.[structFieldDisplayNode].[valueLabelDisplayNode]";

#[test]
fn accepts_value_labels_matching_the_type_of_their_field() {
    let field = field_with_labels(
        "expiry",
        OptionTypeNode::new(IntegerTypeNode::le(I64)),
        vec![
            label(NoneValueNode::new()),
            label(SomeValueNode::new(IntegerValueNode::new(0))),
        ],
    );
    assert_eq!(items(&field), []);
}

#[test]
fn reports_value_labels_not_matching_the_type_of_their_field() {
    let field = field_with_labels(
        "amount",
        IntegerTypeNode::le(U64),
        vec![label(StringValueNode::new("max"))],
    );
    assert_eq!(
        items(&field),
        [error(
            "Value label does not match the type of field \"amount\".",
            LABEL_PATH
        )]
    );
}

#[test]
fn reports_value_labels_with_injected_or_constant_values() {
    let field = field_with_labels(
        "amount",
        IntegerTypeNode::le(U64),
        vec![label(InjectedValueNode::new("max"))],
    );
    let node = with_data("withdraw", StructTypeNode::new(vec![field]));
    assert_eq!(
        items(&node),
        [error(
            "Value labels must be concrete values, got an injected value.",
            &format!("[instructionNode]withdraw.[structTypeNode].{LABEL_PATH}")
        )]
    );

    let constant = ConstantValueNode::new(IntegerTypeNode::le(U64), IntegerValueNode::new(1));
    let field = field_with_labels("amount", IntegerTypeNode::le(U64), vec![label(constant)]);
    assert_eq!(
        items(&field),
        [error(
            "Value labels must be concrete values, got a constant value.",
            LABEL_PATH
        )]
    );
}

#[test]
fn checks_value_labels_against_the_defined_types_their_field_links_to() {
    let field = field_with_labels(
        "amount",
        DefinedTypeLinkNode::new("maybeAmount"),
        vec![label(IntegerValueNode::new(1))],
    );
    let root = RootNode::new(ProgramNode {
        accounts: vec![AccountNode::new("vault", StructTypeNode::new(vec![field]))],
        defined_types: vec![DefinedTypeNode::new(
            "maybeAmount",
            OptionTypeNode::new(IntegerTypeNode::le(U64)),
        )],
        ..program("myProgram")
    });
    assert_eq!(
        items(&root),
        [error(
            "Value label does not match the type of field \"amount\".",
            &format!("[programNode]myProgram.[accountNode]vault.[structTypeNode].{LABEL_PATH}")
        )]
    );
}

// --- Field discriminators ---

fn vault(data: impl Into<TypeNode>, path: &'static str) -> AccountNode {
    AccountNode {
        discriminators: vec![FieldDiscriminatorNode::new(
            path.parse::<codama_nodes::Path>().unwrap(),
            0,
        )
        .into()],
        ..AccountNode::new("vault", data)
    }
}

#[test]
fn accepts_field_discriminators_pointing_to_struct_fields() {
    let data = StructTypeNode::new(vec![StructFieldTypeNode {
        default_value: Box::new(Some(IntegerValueNode::new(1).into())),
        ..StructFieldTypeNode::new("kind", IntegerTypeNode::le(U8))
    }]);
    assert_eq!(items(&vault(data, "kind")), []);
}

#[test]
fn reports_field_discriminators_whose_path_cannot_be_resolved() {
    let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "kind",
        IntegerTypeNode::le(U8),
    )]);
    assert_eq!(
        items(&vault(data, "missing")),
        [caused(
            &cannot_resolve("missing", "missing", "[accountNode]vault.[structTypeNode]"),
            "[accountNode]vault.[fieldDiscriminatorNode]"
        )]
    );
}

#[test]
fn reports_field_discriminators_that_do_not_point_to_struct_fields() {
    let data = TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()]);
    assert_eq!(
        items(&vault(data, "[0]")),
        [error(
            "Field discriminator path \"[0]\" must point to a struct field.",
            "[accountNode]vault.[fieldDiscriminatorNode]"
        )]
    );
}

#[test]
fn reports_field_discriminators_of_instructions_without_data() {
    let node = InstructionNode {
        discriminators: vec![FieldDiscriminatorNode::new("kind", 0).into()],
        ..instruction("close")
    };
    assert_eq!(
        items(&node),
        [error(
            "Field discriminator path \"kind\" cannot be resolved as there is no data.",
            "[instructionNode]close.[fieldDiscriminatorNode]"
        )]
    );
}

fn token_with_discriminator(path: &'static str) -> RootNode {
    let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
        "mint",
        linked_public_key("mint"),
    )]);
    let token = AccountNode {
        discriminators: vec![FieldDiscriminatorNode::new(
            path.parse::<codama_nodes::Path>().unwrap(),
            0,
        )
        .into()],
        ..AccountNode::new("token", data)
    };
    RootNode::new(ProgramNode {
        accounts: vec![token, mint_account()],
        ..program("myProgram")
    })
}

#[test]
fn reports_field_discriminators_crossing_account_links() {
    assert_eq!(
        items(&token_with_discriminator("mint.data.decimals")),
        [caused(
            "Field discriminator path \"mint.data.decimals\" must not cross an account link.",
            "[programNode]myProgram.[accountNode]token.[fieldDiscriminatorNode]"
        )]
    );
}

#[test]
fn reports_field_discriminator_paths_applying_other_segments_to_linked_public_keys_as_unresolvable()
{
    assert_eq!(
        items(&token_with_discriminator("mint.decimals")),
        [caused(
            &cannot_resolve(
                "decimals",
                "mint.decimals",
                "[programNode]myProgram.[accountNode]token.[structTypeNode].[structFieldTypeNode]mint.[publicKeyTypeNode]"
            ),
            "[programNode]myProgram.[accountNode]token.[fieldDiscriminatorNode]"
        )]
    );
}

// --- Contextual values ---

#[test]
fn accepts_data_values_whose_path_resolves_within_the_instruction_data() {
    let node = InstructionNode {
        provides: vec![provide("x", DataValueNode::new("amount"))],
        ..with_data("transfer", amount_struct())
    };
    assert_eq!(items(&node), []);
}

#[test]
fn reports_data_values_whose_path_cannot_be_resolved() {
    let node = InstructionNode {
        provides: vec![provide("x", DataValueNode::new("fee"))],
        ..with_data("transfer", amount_struct())
    };
    assert_eq!(
        items(&node),
        [caused(
            &cannot_resolve("fee", "fee", "[instructionNode]transfer.[structTypeNode]"),
            "[instructionNode]transfer.[providedNode]x.[dataValueNode]"
        )]
    );
}

#[test]
fn reports_data_value_defaults_whose_path_cannot_be_resolved_once() {
    for name in ["missing", "authority"] {
        let default_value = ConditionalValueNode {
            condition: Box::new(
                DataValueNode::new(name.parse::<codama_nodes::Path>().unwrap()).into(),
            ),
            value: Box::new(None),
            if_true: Box::new(Some(PayerValueNode::new().into())),
            if_false: Box::new(None),
            plugins: vec![],
        };
        let payer = InstructionAccountNode {
            default_value: Box::new(Some(default_value.into())),
            ..InstructionAccountNode::new("payer", true, true)
        };
        let node = InstructionNode {
            accounts: vec![
                payer,
                InstructionAccountNode::new("authority", false, false),
            ],
            ..with_data("transfer", amount_struct())
        };
        assert_eq!(
            items(&node),
            [caused(
                &cannot_resolve(name, name, "[instructionNode]transfer.[structTypeNode]"),
                "[instructionNode]transfer.[instructionAccountNode]payer.[conditionalValueNode].[dataValueNode]"
            )],
            "{name}"
        );
    }
}

fn account_defaulting_to(
    identifier: &'static str,
    value: impl Into<InstructionInputValueNode>,
) -> InstructionAccountNode {
    InstructionAccountNode {
        default_value: Box::new(Some(value.into())),
        ..InstructionAccountNode::new(identifier, false, false)
    }
}

#[test]
fn reports_account_values_of_unknown_accounts_once() {
    let account_data = AccountDataValueNode {
        path: Some("owner".parse().unwrap()),
        ..AccountDataValueNode::new("mint")
    };
    let cases: [(InstructionNode, &str); 4] = [
        (
            InstructionNode {
                accounts: vec![account_defaulting_to("owner", account_data)],
                ..instruction("transfer")
            },
            "[instructionNode]transfer.[instructionAccountNode]owner.[accountDataValueNode]",
        ),
        (
            InstructionNode {
                provides: vec![provide("owner", AccountValueNode::new("mint"))],
                ..instruction("transfer")
            },
            "[instructionNode]transfer.[providedNode]owner.[accountValueNode]mint",
        ),
        (
            InstructionNode {
                accounts: vec![account_defaulting_to(
                    "seed",
                    AccountBumpValueNode::new("mint"),
                )],
                ..instruction("transfer")
            },
            "[instructionNode]transfer.[instructionAccountNode]seed.[accountBumpValueNode]mint",
        ),
        (
            InstructionNode {
                accounts: vec![account_defaulting_to(
                    "owner",
                    AccountValueNode::new("mint"),
                )],
                ..with_data(
                    "transfer",
                    StructTypeNode::new(vec![StructFieldTypeNode::new(
                        "mint",
                        PublicKeyTypeNode::new(),
                    )]),
                )
            },
            "[instructionNode]transfer.[instructionAccountNode]owner.[accountValueNode]mint",
        ),
    ];
    for (node, path) in cases {
        assert_eq!(
            items(&node),
            [error(
                "Account \"mint\" is not an account of instruction \"transfer\".",
                path
            )]
        );
    }
}

#[test]
fn reports_data_values_of_instructions_without_data() {
    let node = InstructionNode {
        provides: vec![provide("x", DataValueNode::new("amount"))],
        ..instruction("close")
    };
    assert_eq!(
        items(&node),
        [error(
            "Data path \"amount\" cannot be resolved as instruction \"close\" has no data.",
            "[instructionNode]close.[providedNode]x.[dataValueNode]"
        )]
    );
}

#[test]
fn accepts_data_values_crossing_account_links() {
    let instruction = InstructionNode {
        provides: vec![provide(
            "decimals",
            DataValueNode::new("mint.data.decimals".parse::<codama_nodes::Path>().unwrap()),
        )],
        ..with_data(
            "transfer",
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                "mint",
                linked_public_key("mint"),
            )]),
        )
    };
    let root = RootNode::new(ProgramNode {
        accounts: vec![mint_account()],
        instructions: vec![instruction],
        ..program("myProgram")
    });
    assert_eq!(items(&root), []);
}

fn mint_program(provides: Vec<ProvidedNode>) -> RootNode {
    RootNode::new(ProgramNode {
        accounts: vec![mint_account()],
        instructions: vec![InstructionNode {
            accounts: vec![linked_mint()],
            provides,
            ..instruction("transfer")
        }],
        ..program("myProgram")
    })
}

fn account_data(account: &'static str, path: Option<&'static str>) -> AccountDataValueNode {
    AccountDataValueNode {
        path: path.map(|path| path.parse().unwrap()),
        ..AccountDataValueNode::new(account)
    }
}

#[test]
fn accepts_account_data_values_whose_path_resolves_within_the_linked_account_data() {
    let root = mint_program(vec![
        provide("decimals", account_data("mint", Some("decimals"))),
        provide("state", account_data("mint", None)),
    ]);
    assert_eq!(items(&root), []);
}

#[test]
fn reports_account_data_values_whose_path_cannot_be_resolved() {
    let root = mint_program(vec![provide(
        "supply",
        account_data("mint", Some("supply")),
    )]);
    assert_eq!(
        items(&root),
        [caused(
            &cannot_resolve("supply", "supply", "[programNode]myProgram.[accountNode]mint.[structTypeNode]"),
            "[programNode]myProgram.[instructionNode]transfer.[providedNode]supply.[accountDataValueNode]"
        )]
    );
}

#[test]
fn reports_account_data_values_of_unknown_accounts_or_without_an_account_link() {
    let node = InstructionNode {
        provides: vec![provide("decimals", account_data("mint", Some("decimals")))],
        ..instruction("transfer")
    };
    let path = "[instructionNode]transfer.[providedNode]decimals.[accountDataValueNode]";
    assert_eq!(
        items(&node),
        [error(
            "Account \"mint\" is not an account of instruction \"transfer\".",
            path
        )]
    );
    let node = InstructionNode {
        accounts: vec![InstructionAccountNode::new("mint", false, false)],
        ..node
    };
    assert_eq!(
        items(&node),
        [error(
            "Account \"mint\" has no account link, so its data cannot be read.",
            path
        )]
    );
}

// --- Interpolated intents ---

#[test]
fn accepts_interpolated_intents_whose_placeholders_resolve() {
    let root = RootNode::new(ProgramNode {
        accounts: vec![mint_account()],
        instructions: vec![InstructionNode {
            accounts: vec![linked_mint()],
            display: Some(InstructionDisplayNode::interpolated(
                "Mint ${data.amount} of ${accounts.mint} (${accounts.mint.data.decimals} decimals)",
            )),
            ..with_data("mintTo", amount_struct())
        }],
        ..program("myProgram")
    });
    assert_eq!(items(&root), []);
}

#[test]
fn reports_interpolated_intents_with_invalid_placeholders() {
    let node = InstructionNode {
        display: Some(InstructionDisplayNode::interpolated(
            "Burn ${accounts.mint.lamports}",
        )),
        ..instruction("burn")
    };
    assert_eq!(
        items(&node),
        [caused(
            "Invalid placeholder [${accounts.mint.lamports}] in interpolated intent [Burn ${accounts.mint.lamports}]. Placeholders are `${data.<path>}`, `${accounts.<account>}` or `${accounts.<account>.data[.<path>]}`.",
            "[instructionNode]burn.[instructionDisplayNode]"
        )]
    );
}

#[test]
fn reports_interpolated_intents_referring_to_unknown_accounts_or_unresolvable_paths() {
    let intent = Text::Node(TextNode {
        content: "Close ${accounts.vault}".to_string(),
        plugins: vec![codama_nodes::PluginNode::new(
            "i18n.es".parse::<codama_nodes::Namespace>().unwrap(),
        )],
    });
    let node = InstructionNode {
        display: Some(InstructionDisplayNode::interpolated(intent)),
        ..instruction("close")
    };
    assert_eq!(
        items(&node),
        [error(
            "Account \"vault\" is not an account of instruction \"close\".",
            "[instructionNode]close.[instructionDisplayNode]"
        )]
    );

    let node = InstructionNode {
        display: Some(InstructionDisplayNode::interpolated("Transfer ${data.fee}")),
        ..with_data("transfer", amount_struct())
    };
    assert_eq!(
        items(&node),
        [caused(
            &cannot_resolve("fee", "fee", "[instructionNode]transfer.[structTypeNode]"),
            "[instructionNode]transfer.[instructionDisplayNode]"
        )]
    );
}
