use codama_nodes::test_utils::id;
use codama_nodes::{
    AccountLinkNode, AccountNode, DefinedTypeLinkNode, DefinedTypeNode, EnumValueNode,
    InstructionAccountLinkNode, InstructionAccountNode, InstructionByteDeltaNode,
    InstructionLinkNode, InstructionNode, Node, NodePath, NodeVisitor, PdaLinkNode, PdaNode,
    PdaValueNode, ProgramLinkNode, ProgramNode, ProvidedNode, PublicKeyTypeNode, RootNode,
    StructFieldTypeNode, StructTypeNode, ValidateLinksVisitor,
};

fn link_errors(root: &RootNode) -> Vec<String> {
    let mut visitor = ValidateLinksVisitor::new(root);
    visitor.visit_root(root, &NodePath::default());
    visitor.errors
}

fn program(identifier: &str) -> ProgramNode {
    ProgramNode::new(id(identifier), "1111")
}

fn account(identifier: &str) -> AccountNode {
    AccountNode::new(id(identifier), StructTypeNode::new(vec![]))
}

fn defined_type(identifier: &str) -> DefinedTypeNode {
    DefinedTypeNode::new(id(identifier), StructTypeNode::new(vec![]))
}

fn defined_type_with_account_link(identifier: &str, link: AccountLinkNode) -> DefinedTypeNode {
    let r#type = PublicKeyTypeNode {
        account_link: Some(link),
        ..PublicKeyTypeNode::new()
    };
    DefinedTypeNode::new(id(identifier), r#type)
}

fn instruction_account(identifier: &str) -> InstructionAccountNode {
    InstructionAccountNode::new(id(identifier), false, false)
}

fn instruction(identifier: &str, accounts: &[&str]) -> InstructionNode {
    InstructionNode {
        identifier: id(identifier),
        accounts: accounts
            .iter()
            .map(|account| instruction_account(account))
            .collect(),
        ..InstructionNode::default()
    }
}

fn instruction_with_provided_link(
    identifier: &str,
    accounts: &[&str],
    link: impl Into<Node>,
) -> InstructionNode {
    InstructionNode {
        provides: vec![ProvidedNode::new(id("link"), link)],
        ..instruction(identifier, accounts)
    }
}

fn missing_message(target: &str, identifier: &str, path: &str) -> String {
    format!("Pointing to a missing {target} named \"{identifier}\" in {path}.")
}

#[test]
fn account_link() {
    let root = RootNode::new(
        program("token")
            .add_account(account("mint"))
            .add_defined_type(defined_type_with_account_link(
                "mint_holder",
                AccountLinkNode::new(id("mint")),
            ))
            .add_defined_type(defined_type_with_account_link(
                "lost_holder",
                AccountLinkNode::new(id("token")),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "account",
            "token",
            r#"program "token" > definedType "lost_holder" > type > accountLink "token""#
        )]
    );
}

#[test]
fn link_identifier_matches_exactly() {
    let root = RootNode::new(
        program("token")
            .add_account(account("token_mint"))
            .add_defined_type(defined_type_with_account_link(
                "holder",
                AccountLinkNode::new(id("tokenMint")),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "account",
            "tokenMint",
            r#"program "token" > definedType "holder" > type > accountLink "tokenMint""#
        )]
    );
}

#[test]
fn defined_type_link() {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new(id("kind"), DefinedTypeLinkNode::new(id("config_kind"))),
        StructFieldTypeNode::new(id("mode"), DefinedTypeLinkNode::new(id("mode"))),
    ]);
    let root = RootNode::new(
        program("token")
            .add_account(AccountNode::new(id("config"), data))
            .add_defined_type(defined_type("config_kind")),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "defined type",
            "mode",
            r#"program "token" > account "config" > data > structFieldType "mode" > definedTypeLink "mode""#
        )]
    );
}

#[test]
fn pda_link() {
    let vault_account = AccountNode {
        pda: Some(PdaLinkNode::new(id("vault"))),
        ..account("vault")
    };
    let escrow_account = AccountNode {
        pda: Some(PdaLinkNode::new(id("escrow"))),
        ..account("escrow")
    };
    let root = RootNode::new(
        program("token")
            .add_account(vault_account)
            .add_account(escrow_account)
            .add_pda(PdaNode::new(id("vault"), vec![])),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "PDA",
            "escrow",
            r#"program "token" > account "escrow" > pdaLink "escrow""#
        )]
    );
}

#[test]
fn program_link() {
    let system_program = InstructionAccountNode {
        default_value: Box::new(Some(ProgramLinkNode::new(id("system")).into())),
        ..instruction_account("system_program")
    };
    let token_program = InstructionAccountNode {
        default_value: Box::new(Some(ProgramLinkNode::new(id("token_2022")).into())),
        ..instruction_account("token_program")
    };
    let create_ix = InstructionNode {
        accounts: vec![system_program, token_program],
        ..instruction("create", &[])
    };
    let root =
        RootNode::new(program("app").add_instruction(create_ix)).add_program(program("system"));
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "program",
            "token_2022",
            r#"program "app" > instruction "create" > instructionAccount "token_program" > programLink "token_2022""#
        )]
    );
}

#[test]
fn instruction_link() {
    let root = RootNode::new(
        program("token")
            .add_instruction(instruction("approve", &[]))
            .add_instruction(instruction_with_provided_link(
                "transfer",
                &[],
                InstructionLinkNode::new(id("approve")),
            ))
            .add_instruction(instruction_with_provided_link(
                "burn",
                &[],
                InstructionLinkNode::new(id("mint_to")),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "instruction",
            "mint_to",
            r#"program "token" > instruction "burn" > provided "link" > instructionLink "mint_to""#
        )]
    );
}

#[test]
fn instruction_account_link_in_the_closest_instruction() {
    let root = RootNode::new(
        program("token")
            .add_instruction(instruction_with_provided_link(
                "transfer",
                &["source"],
                InstructionAccountLinkNode::new(id("source")),
            ))
            .add_instruction(instruction_with_provided_link(
                "burn",
                &["mint"],
                InstructionAccountLinkNode::new(id("source")),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "instruction account",
            "source",
            r#"program "token" > instruction "burn" > provided "link" > instructionAccountLink "source""#
        )]
    );
}

#[test]
fn instruction_account_link_in_a_named_instruction() {
    let transfer_account_link = |account: &str| {
        InstructionAccountLinkNode::new_from_instruction(
            id(account),
            InstructionLinkNode::new(id("transfer")),
        )
    };
    let root = RootNode::new(
        program("token")
            .add_instruction(instruction("transfer", &["source"]))
            .add_instruction(instruction_with_provided_link(
                "burn",
                &[],
                transfer_account_link("source"),
            ))
            .add_instruction(instruction_with_provided_link(
                "close",
                &[],
                transfer_account_link("authority"),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "instruction account",
            "authority",
            r#"program "token" > instruction "close" > provided "link" > instructionAccountLink "authority""#
        )]
    );
}

#[test]
fn instruction_account_link_in_another_program() {
    let token_transfer_account_link = |account: &str| {
        let transfer_link = InstructionLinkNode::new_from_program(
            id("transfer"),
            ProgramLinkNode::new(id("token")),
        );
        InstructionAccountLinkNode::new_from_instruction(id(account), transfer_link)
    };
    let root = RootNode::new(
        program("app")
            .add_instruction(instruction_with_provided_link(
                "swap",
                &[],
                token_transfer_account_link("source"),
            ))
            .add_instruction(instruction_with_provided_link(
                "deposit",
                &[],
                token_transfer_account_link("destination"),
            )),
    )
    .add_program(program("token").add_instruction(instruction("transfer", &["source"])));
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "instruction account",
            "destination",
            r#"program "app" > instruction "deposit" > provided "link" > instructionAccountLink "destination""#
        )]
    );
}

#[test]
fn links_with_and_without_program() {
    let token_mint_link =
        AccountLinkNode::new_from_program(id("mint"), ProgramLinkNode::new(id("token")));
    let app_config_link =
        AccountLinkNode::new_from_program(id("config"), ProgramLinkNode::new(id("app")));
    let root = RootNode::new(
        program("app")
            .add_account(account("config"))
            .add_defined_type(defined_type_with_account_link(
                "mint_holder",
                token_mint_link,
            ))
            .add_defined_type(defined_type_with_account_link(
                "local_holder",
                AccountLinkNode::new(id("mint")),
            )),
    )
    .add_program(
        program("token")
            .add_account(account("mint"))
            .add_defined_type(defined_type_with_account_link(
                "config_holder",
                app_config_link,
            ))
            .add_defined_type(defined_type_with_account_link(
                "mint_holder",
                AccountLinkNode::new(id("mint")),
            ))
            .add_defined_type(defined_type_with_account_link(
                "local_holder",
                AccountLinkNode::new(id("config")),
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![
            missing_message(
                "account",
                "mint",
                r#"program "app" > definedType "local_holder" > type > accountLink "mint""#
            ),
            missing_message(
                "account",
                "config",
                r#"program "token" > definedType "local_holder" > type > accountLink "config""#
            ),
        ]
    );
}

#[test]
fn sub_instructions() {
    let batch_ix = InstructionNode {
        sub_instructions: vec![
            instruction_with_provided_link(
                "transfer",
                &["source"],
                InstructionAccountLinkNode::new(id("source")),
            ),
            instruction_with_provided_link(
                "close",
                &[],
                InstructionAccountLinkNode::new(id("payer")),
            ),
        ],
        ..instruction("batch", &["payer"])
    };
    let root = RootNode::new(program("token").add_instruction(batch_ix).add_instruction(
        instruction_with_provided_link("retry", &[], InstructionLinkNode::new(id("transfer"))),
    ));
    assert_eq!(
        link_errors(&root),
        vec![missing_message(
            "instruction account",
            "payer",
            r#"program "token" > instruction "batch" > instruction "close" > provided "link" > instructionAccountLink "payer""#
        )]
    );
}

#[test]
fn dangling_scope_link_is_reported_once() {
    let unknown_program_link = || ProgramLinkNode::new(id("unknown"));
    let source_link_via_unknown_ix = InstructionAccountLinkNode::new_from_instruction(
        id("source"),
        InstructionLinkNode::new(id("unknown")),
    );
    let source_link_via_unknown_program = InstructionAccountLinkNode::new_from_instruction(
        id("source"),
        InstructionLinkNode::new_from_program(id("transfer"), unknown_program_link()),
    );
    let root = RootNode::new(
        program("token")
            .add_defined_type(defined_type_with_account_link(
                "holder",
                AccountLinkNode::new_from_program(id("mint"), unknown_program_link()),
            ))
            .add_instruction(instruction("transfer", &["source"]))
            .add_instruction(instruction_with_provided_link(
                "burn",
                &[],
                source_link_via_unknown_ix,
            ))
            .add_instruction(instruction_with_provided_link(
                "close",
                &[],
                source_link_via_unknown_program,
            )),
    );
    assert_eq!(
        link_errors(&root),
        vec![
            missing_message(
                "instruction",
                "unknown",
                r#"program "token" > instruction "burn" > provided "link" > instructionAccountLink "source" > instructionLink "unknown""#
            ),
            missing_message(
                "program",
                "unknown",
                r#"program "token" > instruction "close" > provided "link" > instructionAccountLink "source" > instructionLink "transfer" > programLink "unknown""#
            ),
            missing_message(
                "program",
                "unknown",
                r#"program "token" > definedType "holder" > type > accountLink "mint" > programLink "unknown""#
            ),
        ]
    );
}

#[test]
fn links_at_every_position() {
    let config_account = InstructionAccountNode {
        account_link: Some(AccountLinkNode::new(id("config_state"))),
        ..instruction_account("config")
    };
    let vault_account = InstructionAccountNode {
        default_value: Box::new(Some(
            PdaValueNode::new(PdaLinkNode::new(id("vault")), vec![]).into(),
        )),
        ..instruction_account("vault")
    };
    let mode_account = InstructionAccountNode {
        default_value: Box::new(Some(
            EnumValueNode::empty(DefinedTypeLinkNode::new(id("mode")), id("fast")).into(),
        )),
        ..instruction_account("mode")
    };
    let init_ix = InstructionNode {
        accounts: vec![config_account, vault_account, mode_account],
        byte_deltas: vec![InstructionByteDeltaNode::new(
            AccountLinkNode::new(id("buffer")),
            true,
        )],
        ..instruction("init", &[])
    };
    let root = RootNode::new(program("token").add_instruction(init_ix));
    assert_eq!(
        link_errors(&root),
        vec![
            missing_message(
                "account",
                "config_state",
                r#"program "token" > instruction "init" > instructionAccount "config" > accountLink "config_state""#
            ),
            missing_message(
                "PDA",
                "vault",
                r#"program "token" > instruction "init" > instructionAccount "vault" > defaultValue > pdaLink "vault""#
            ),
            missing_message(
                "defined type",
                "mode",
                r#"program "token" > instruction "init" > instructionAccount "mode" > defaultValue > definedTypeLink "mode""#
            ),
            missing_message(
                "account",
                "buffer",
                r#"program "token" > instruction "init" > byteDeltas[0] > accountLink "buffer""#
            ),
        ]
    );
}
