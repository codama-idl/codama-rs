use codama_nodes::test_utils::{id, path};
use codama_nodes::{
    AccountBumpValueNode, AccountDataValueNode, AccountLinkNode, AccountNode, AccountValueNode,
    ConditionalValueNode, DataValueNode, DefinedTypeLinkNode, DefinedTypeNode, DiscriminatorNode,
    EventNode, FieldDiscriminatorNode, InstructionAccountNode, InstructionByteDeltaNode,
    InstructionNode, InstructionRemainingAccountsNode, IntegerTypeNode, Linkables, NodePath,
    NodeVisitor, PdaLinkNode, PdaSeedValueNode, PdaValueNode, ProgramLinkNode, ProgramNode,
    ProvidedNode, PublicKeyTypeNode, RootNode, SizeDiscriminatorNode, StructFieldTypeNode,
    StructTypeNode, TupleTypeNode, TypeNode, ValidatePathsVisitor, U64, U8,
};

fn path_errors(root: &RootNode) -> Vec<String> {
    let linkables = Linkables::new(root);
    let mut visitor = ValidatePathsVisitor::new(&linkables);
    visitor.visit_root(root, &NodePath::default());
    visitor.errors
}

fn program(identifier: &str) -> ProgramNode {
    ProgramNode::new(id(identifier), "1111")
}

fn field(identifier: &str, r#type: impl Into<TypeNode>) -> StructFieldTypeNode {
    StructFieldTypeNode::new(id(identifier), r#type)
}

fn amount_data() -> StructTypeNode {
    StructTypeNode::new(vec![field("amount", IntegerTypeNode::le(U64))])
}

fn mint_account() -> AccountNode {
    let data = StructTypeNode::new(vec![field("decimals", IntegerTypeNode::le(U8))]);
    AccountNode::new(id("mint"), data)
}

fn public_key_with_account_link(link: AccountLinkNode) -> PublicKeyTypeNode {
    PublicKeyTypeNode {
        account_link: Some(link),
        ..PublicKeyTypeNode::new()
    }
}

fn instruction_account(identifier: &str) -> InstructionAccountNode {
    InstructionAccountNode::new(id(identifier), false, false)
}

fn instruction_account_with_account_link(
    identifier: &str,
    link: AccountLinkNode,
) -> InstructionAccountNode {
    InstructionAccountNode {
        account_link: Some(link),
        ..instruction_account(identifier)
    }
}

fn instruction(identifier: &str) -> InstructionNode {
    InstructionNode {
        identifier: id(identifier),
        ..InstructionNode::default()
    }
}

fn instruction_with_data(identifier: &str, data: impl Into<TypeNode>) -> InstructionNode {
    InstructionNode {
        data: Box::new(Some(data.into())),
        ..instruction(identifier)
    }
}

fn account_with_field_discriminators(
    identifier: &str,
    data: impl Into<TypeNode>,
    field_paths: &[&str],
) -> AccountNode {
    AccountNode {
        discriminators: field_discriminators(field_paths),
        ..AccountNode::new(id(identifier), data)
    }
}

fn field_discriminators(field_paths: &[&str]) -> Vec<DiscriminatorNode> {
    field_paths
        .iter()
        .map(|field_path| FieldDiscriminatorNode::new(path(field_path), 0).into())
        .collect()
}

fn unresolvable_segment_message(segment: &str, type_path: &str, path: &str) -> String {
    format!("Cannot resolve segment \"{segment}\" of path \"{type_path}\" in {path}.")
}

fn unknown_account_message(account: &str, instruction: &str, path: &str) -> String {
    format!("Account \"{account}\" is not an account of instruction \"{instruction}\" in {path}.")
}

fn no_account_link_message(account: &str, path: &str) -> String {
    format!("Account \"{account}\" has no account link, so its data cannot be read in {path}.")
}

fn no_instruction_data_message(data_path: &str, instruction: &str, path: &str) -> String {
    format!(
        "Data path \"{data_path}\" cannot be resolved as instruction \"{instruction}\" has no data in {path}."
    )
}

fn no_discriminator_data_message(field_path: &str, path: &str) -> String {
    format!(
        "Field discriminator path \"{field_path}\" cannot be resolved as there is no data in {path}."
    )
}

fn not_a_struct_field_message(field_path: &str, path: &str) -> String {
    format!("Field discriminator path \"{field_path}\" must point to a struct field in {path}.")
}

fn crosses_account_link_message(field_path: &str, path: &str) -> String {
    format!("Field discriminator path \"{field_path}\" must not cross an account link in {path}.")
}

#[test]
fn data_value() {
    let transfer_ix = InstructionNode {
        provides: vec![
            ProvidedNode::new(id("amount"), DataValueNode::new(path("amount"))),
            ProvidedNode::new(id("fee"), DataValueNode::new(path("fee"))),
        ],
        ..instruction_with_data("transfer", amount_data())
    };
    let root = RootNode::new(program("token").add_instruction(transfer_ix));
    assert_eq!(
        path_errors(&root),
        vec![unresolvable_segment_message(
            "fee",
            "fee",
            r#"program "token" > instruction "transfer" > provided "fee" > node"#
        )]
    );
}

#[test]
fn data_value_of_an_instruction_without_data() {
    let close_ix = InstructionNode {
        provides: vec![ProvidedNode::new(
            id("amount"),
            DataValueNode::new(path("amount")),
        )],
        ..instruction("close")
    };
    let root = RootNode::new(program("token").add_instruction(close_ix));
    assert_eq!(
        path_errors(&root),
        vec![no_instruction_data_message(
            "amount",
            "close",
            r#"program "token" > instruction "close" > provided "amount" > node"#
        )]
    );
}

#[test]
fn data_value_crossing_an_account_link() {
    let mint_field = field(
        "mint",
        public_key_with_account_link(AccountLinkNode::new(id("mint"))),
    );
    let transfer_ix = InstructionNode {
        provides: vec![
            ProvidedNode::new(
                id("decimals"),
                DataValueNode::new(path("mint.data.decimals")),
            ),
            ProvidedNode::new(id("supply"), DataValueNode::new(path("mint.data.supply"))),
        ],
        ..instruction_with_data("transfer", StructTypeNode::new(vec![mint_field]))
    };
    let root = RootNode::new(
        program("token")
            .add_account(mint_account())
            .add_instruction(transfer_ix),
    );
    assert_eq!(
        path_errors(&root),
        vec![unresolvable_segment_message(
            "supply",
            "mint.data.supply",
            r#"program "token" > instruction "transfer" > provided "supply" > node"#
        )]
    );
}

#[test]
fn account_value_and_account_bump_value() {
    let transfer_ix = InstructionNode {
        accounts: vec![instruction_account("authority")],
        provides: vec![
            ProvidedNode::new(id("owner"), AccountValueNode::new(id("authority"))),
            ProvidedNode::new(id("bump"), AccountBumpValueNode::new(id("authority"))),
            ProvidedNode::new(id("payer"), AccountValueNode::new(id("payer"))),
            ProvidedNode::new(id("payer_bump"), AccountBumpValueNode::new(id("payer"))),
        ],
        ..instruction("transfer")
    };
    let root = RootNode::new(program("token").add_instruction(transfer_ix));
    assert_eq!(
        path_errors(&root),
        vec![
            unknown_account_message(
                "payer",
                "transfer",
                r#"program "token" > instruction "transfer" > provided "payer" > accountValue "payer""#
            ),
            unknown_account_message(
                "payer",
                "transfer",
                r#"program "token" > instruction "transfer" > provided "payer_bump" > accountBumpValue "payer""#
            ),
        ]
    );
}

#[test]
fn remaining_accounts_are_not_instruction_accounts() {
    let transfer_ix = InstructionNode {
        accounts: vec![instruction_account("authority")],
        remaining_accounts: vec![InstructionRemainingAccountsNode {
            identifier: id("signers"),
            ..InstructionRemainingAccountsNode::default()
        }],
        provides: vec![ProvidedNode::new(
            id("signer"),
            AccountValueNode::new(id("signers")),
        )],
        ..instruction("transfer")
    };
    let root = RootNode::new(program("token").add_instruction(transfer_ix));
    assert_eq!(
        path_errors(&root),
        vec![unknown_account_message(
            "signers",
            "transfer",
            r#"program "token" > instruction "transfer" > provided "signer" > accountValue "signers""#
        )]
    );
}

#[test]
fn account_data_value() {
    let transfer_ix = InstructionNode {
        accounts: vec![
            instruction_account_with_account_link("mint", AccountLinkNode::new(id("mint"))),
            instruction_account("vault"),
        ],
        provides: vec![
            ProvidedNode::new(
                id("decimals"),
                AccountDataValueNode::with_path(id("mint"), path("decimals")),
            ),
            ProvidedNode::new(id("mint_data"), AccountDataValueNode::new(id("mint"))),
            ProvidedNode::new(
                id("supply"),
                AccountDataValueNode::with_path(id("mint"), path("supply")),
            ),
            ProvidedNode::new(
                id("unknown"),
                AccountDataValueNode::with_path(id("authority"), path("decimals")),
            ),
            ProvidedNode::new(id("vault_data"), AccountDataValueNode::new(id("vault"))),
            ProvidedNode::new(
                id("vault_amount"),
                AccountDataValueNode::with_path(id("vault"), path("amount")),
            ),
        ],
        ..instruction("transfer")
    };
    let root = RootNode::new(
        program("token")
            .add_account(mint_account())
            .add_instruction(transfer_ix),
    );
    let path_to_provided = |provided: &str| {
        format!(r#"program "token" > instruction "transfer" > provided "{provided}" > node"#)
    };
    assert_eq!(
        path_errors(&root),
        vec![
            unresolvable_segment_message("supply", "supply", &path_to_provided("supply")),
            unknown_account_message("authority", "transfer", &path_to_provided("unknown")),
            no_account_link_message("vault", &path_to_provided("vault_data")),
            no_account_link_message("vault", &path_to_provided("vault_amount")),
        ]
    );
}

#[test]
fn account_data_value_of_another_program() {
    let token_mint_link =
        AccountLinkNode::new_from_program(id("mint"), ProgramLinkNode::new(id("token")));
    let swap_ix = InstructionNode {
        accounts: vec![instruction_account_with_account_link(
            "mint",
            token_mint_link,
        )],
        provides: vec![
            ProvidedNode::new(
                id("decimals"),
                AccountDataValueNode::with_path(id("mint"), path("config.decimals")),
            ),
            ProvidedNode::new(
                id("supply"),
                AccountDataValueNode::with_path(id("mint"), path("config.supply")),
            ),
        ],
        ..instruction("swap")
    };
    let mint_data = StructTypeNode::new(vec![field(
        "config",
        DefinedTypeLinkNode::new(id("mint_config")),
    )]);
    let mint_config = StructTypeNode::new(vec![field("decimals", IntegerTypeNode::le(U8))]);
    let root = RootNode::new(program("app").add_instruction(swap_ix)).add_program(
        program("token")
            .add_account(AccountNode::new(id("mint"), mint_data))
            .add_defined_type(DefinedTypeNode::new(id("mint_config"), mint_config)),
    );
    assert_eq!(
        path_errors(&root),
        vec![unresolvable_segment_message(
            "supply",
            "config.supply",
            r#"program "app" > instruction "swap" > provided "supply" > node"#
        )]
    );
}

#[test]
fn closest_instruction() {
    let transfer_ix = InstructionNode {
        provides: vec![
            ProvidedNode::new(id("owner"), AccountValueNode::new(id("authority"))),
            ProvidedNode::new(id("amount"), DataValueNode::new(path("amount"))),
        ],
        ..instruction("transfer")
    };
    let batch_ix = InstructionNode {
        accounts: vec![instruction_account("authority")],
        sub_instructions: vec![transfer_ix],
        provides: vec![ProvidedNode::new(
            id("owner"),
            AccountValueNode::new(id("authority")),
        )],
        ..instruction_with_data("batch", amount_data())
    };
    let root = RootNode::new(program("token").add_instruction(batch_ix));
    let path_to_transfer_ix = r#"program "token" > instruction "batch" > instruction "transfer""#;
    assert_eq!(
        path_errors(&root),
        vec![
            unknown_account_message(
                "authority",
                "transfer",
                &format!(r#"{path_to_transfer_ix} > provided "owner" > accountValue "authority""#)
            ),
            no_instruction_data_message(
                "amount",
                "transfer",
                &format!(r#"{path_to_transfer_ix} > provided "amount" > node"#)
            ),
        ]
    );
}

#[test]
fn values_at_every_position() {
    let pda_value = PdaValueNode::new_with_program_id(
        PdaLinkNode::new(id("vault")),
        vec![PdaSeedValueNode::new(
            id("amount"),
            DataValueNode::new(path("fee")),
        )],
        AccountValueNode::new(id("token_program")),
    );
    let vault_account = InstructionAccountNode {
        default_value: Box::new(Some(pda_value.into())),
        ..instruction_account("vault")
    };
    let conditional_value = ConditionalValueNode {
        condition: Box::new(DataValueNode::new(path("amount.value")).into()),
        value: Box::new(None),
        if_true: Box::new(Some(AccountValueNode::new(id("owner")).into())),
        if_false: Box::new(Some(AccountBumpValueNode::new(id("vault_pda")).into())),
        plugins: vec![],
    };
    let authority_account = InstructionAccountNode {
        default_value: Box::new(Some(conditional_value.into())),
        ..instruction_account("authority")
    };
    let deposit_ix = InstructionNode {
        accounts: vec![vault_account, authority_account],
        byte_deltas: vec![InstructionByteDeltaNode::new(
            DataValueNode::new(path("size")),
            true,
        )],
        ..instruction_with_data("deposit", amount_data())
    };
    let root = RootNode::new(program("token").add_instruction(deposit_ix));
    let path_to_vault_account =
        r#"program "token" > instruction "deposit" > instructionAccount "vault" > defaultValue"#;
    let path_to_authority_account = r#"program "token" > instruction "deposit" > instructionAccount "authority" > defaultValue"#;
    assert_eq!(
        path_errors(&root),
        vec![
            unresolvable_segment_message(
                "fee",
                "fee",
                &format!(r#"{path_to_vault_account} > pdaSeedValue "amount" > value"#)
            ),
            unknown_account_message(
                "token_program",
                "deposit",
                &format!(r#"{path_to_vault_account} > accountValue "token_program""#)
            ),
            unresolvable_segment_message(
                "value",
                "amount.value",
                &format!("{path_to_authority_account} > condition")
            ),
            unknown_account_message(
                "owner",
                "deposit",
                &format!(r#"{path_to_authority_account} > accountValue "owner""#)
            ),
            unknown_account_message(
                "vault_pda",
                "deposit",
                &format!(r#"{path_to_authority_account} > accountBumpValue "vault_pda""#)
            ),
            unresolvable_segment_message(
                "size",
                "size",
                r#"program "token" > instruction "deposit" > byteDeltas[0] > value"#
            ),
        ]
    );
}

#[test]
fn dangling_links_are_skipped() {
    let transfer_ix = InstructionNode {
        accounts: vec![
            instruction_account_with_account_link("mint", AccountLinkNode::new(id("missing"))),
            instruction_account_with_account_link(
                "vault",
                AccountLinkNode::new_from_program(id("vault"), ProgramLinkNode::new(id("missing"))),
            ),
            InstructionAccountNode {
                default_value: Box::new(Some(DataValueNode::new(path("config.fee")).into())),
                ..instruction_account("payer")
            },
        ],
        provides: vec![
            ProvidedNode::new(
                id("decimals"),
                AccountDataValueNode::with_path(id("mint"), path("decimals")),
            ),
            ProvidedNode::new(
                id("amount"),
                AccountDataValueNode::with_path(id("vault"), path("amount")),
            ),
        ],
        ..instruction_with_data(
            "transfer",
            StructTypeNode::new(vec![field(
                "config",
                DefinedTypeLinkNode::new(id("config")),
            )]),
        )
    };
    let root = RootNode::new(program("token").add_instruction(transfer_ix));
    assert_eq!(path_errors(&root), Vec::<String>::new());
}

#[test]
fn field_discriminator_errors() {
    let vault = AccountNode {
        discriminators: vec![
            SizeDiscriminatorNode::new(8).into(),
            FieldDiscriminatorNode::new(path("missing"), 0).into(),
        ],
        ..AccountNode::new(id("vault"), amount_data())
    };
    let deposited = EventNode {
        discriminators: field_discriminators(&["[0]"]),
        ..EventNode::new(
            id("deposited"),
            TupleTypeNode::new(vec![IntegerTypeNode::le(U8).into()]),
        )
    };
    let close_ix = InstructionNode {
        discriminators: field_discriminators(&["kind"]),
        ..instruction("close")
    };
    let root = RootNode::new(
        program("token")
            .add_account(vault)
            .add_event(deposited)
            .add_instruction(close_ix),
    );
    assert_eq!(
        path_errors(&root),
        vec![
            unresolvable_segment_message(
                "missing",
                "missing",
                r#"program "token" > account "vault" > discriminators[1]"#
            ),
            no_discriminator_data_message(
                "kind",
                r#"program "token" > instruction "close" > discriminators[0]"#
            ),
            not_a_struct_field_message(
                "[0]",
                r#"program "token" > event "deposited" > discriminators[0]"#
            ),
        ]
    );
}

#[test]
fn field_discriminators_across_account_links() {
    let mint_field = field(
        "mint",
        public_key_with_account_link(AccountLinkNode::new(id("mint"))),
    );
    let token = account_with_field_discriminators(
        "token",
        StructTypeNode::new(vec![mint_field]),
        &["mint.data.decimals", "mint.decimals", "mint"],
    );
    let root = RootNode::new(
        program("token")
            .add_account(token)
            .add_account(mint_account()),
    );
    let path_to_discriminator =
        |index: usize| format!(r#"program "token" > account "token" > discriminators[{index}]"#);
    assert_eq!(
        path_errors(&root),
        vec![
            crosses_account_link_message("mint.data.decimals", &path_to_discriminator(0)),
            unresolvable_segment_message("decimals", "mint.decimals", &path_to_discriminator(1)),
        ]
    );
}

#[test]
fn field_discriminators_through_defined_type_links() {
    let header = DefinedTypeNode::new(
        id("header"),
        StructTypeNode::new(vec![field("kind", IntegerTypeNode::le(U8))]),
    );
    let data = StructTypeNode::new(vec![
        field("header", DefinedTypeLinkNode::new(id("header"))),
        field("config", DefinedTypeLinkNode::new(id("missing"))),
    ]);
    let vault = account_with_field_discriminators(
        "vault",
        data,
        &["header.kind", "config.kind", "header.mode"],
    );
    let root = RootNode::new(program("token").add_defined_type(header).add_account(vault));
    let path_to_discriminator =
        |index: usize| format!(r#"program "token" > account "vault" > discriminators[{index}]"#);
    assert_eq!(
        path_errors(&root),
        vec![unresolvable_segment_message(
            "mode",
            "header.mode",
            &path_to_discriminator(2)
        )]
    );
}

#[test]
fn field_discriminator_of_a_sub_instruction() {
    let transfer_ix = InstructionNode {
        discriminators: field_discriminators(&["amount", "kind"]),
        ..instruction_with_data("transfer", amount_data())
    };
    let batch_ix = InstructionNode {
        sub_instructions: vec![transfer_ix],
        ..instruction("batch")
    };
    let root = RootNode::new(program("token").add_instruction(batch_ix));
    assert_eq!(
        path_errors(&root),
        vec![unresolvable_segment_message(
            "kind",
            "kind",
            r#"program "token" > instruction "batch" > instruction "transfer" > discriminators[1]"#
        )]
    );
}
