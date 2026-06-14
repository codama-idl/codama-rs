use codama_nodes::{
    AccountValueNode, ArgumentValueNode, IdentityValueNode, InstructionAccountNode,
    InstructionArgumentNode, InstructionInputValueNode, InstructionNode, IsSigner, NumberTypeNode,
    PayerValueNode, PdaLinkNode, PdaNode, PdaSeedNode, PdaSeedValueValue, PdaValueNode,
    ProgramNode, PublicKeyTypeNode, PublicKeyValueNode, RootNode, VariablePdaSeedNode, U8,
};
use codama_visitors::{
    get_common_instruction_account_default_rules, set_instruction_account_default_values,
    InstructionAccountDefaultRule,
};
use pretty_assertions::assert_eq;

fn account(name: &str) -> InstructionAccountNode {
    InstructionAccountNode::new(name, false, IsSigner::False)
}

fn sample_root() -> RootNode {
    let mut transfer = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    transfer.accounts.push(account("payer"));
    transfer.accounts.push(account("authority"));
    transfer.accounts.push(account("systemProgram"));

    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(transfer);
    RootNode::new(program)
}

fn default_value(root: &RootNode, account: usize) -> Option<InstructionInputValueNode> {
    (*root.program.instructions[0].accounts[account].default_value).clone()
}

#[test]
fn assigns_default_values_to_matching_accounts() {
    let root = set_instruction_account_default_values(
        sample_root(),
        vec![
            InstructionAccountDefaultRule::new("payer", PayerValueNode::new()),
            InstructionAccountDefaultRule::new("authority", IdentityValueNode::new()),
        ],
    );

    assert_eq!(
        default_value(&root, 0),
        Some(InstructionInputValueNode::PayerValue(PayerValueNode::new())),
    );
    assert_eq!(
        default_value(&root, 1),
        Some(InstructionInputValueNode::IdentityValue(
            IdentityValueNode::new()
        )),
    );
    // No rule for `systemProgram`.
    assert_eq!(default_value(&root, 2), None);
}

#[test]
fn ignore_if_optional_skips_optional_accounts() {
    let mut root = sample_root();
    root.program.instructions[0].accounts[0].is_optional = Some(true); // payer is optional

    let root = set_instruction_account_default_values(
        root,
        vec![
            InstructionAccountDefaultRule::new("payer", PayerValueNode::new()).ignore_if_optional(),
        ],
    );

    assert_eq!(default_value(&root, 0), None);
}

#[test]
fn instruction_scoped_rules_only_apply_to_that_instruction() {
    let root = set_instruction_account_default_values(
        sample_root(),
        vec![
            InstructionAccountDefaultRule::new("payer", PayerValueNode::new()).instruction("other"),
        ],
    );

    // The rule is scoped to `other`, so `transfer`'s payer is untouched.
    assert_eq!(default_value(&root, 0), None);
}

#[test]
fn predicate_matcher_matches_multiple_names() {
    let root = set_instruction_account_default_values(
        sample_root(),
        vec![InstructionAccountDefaultRule::matching(
            |name| matches!(name, "payer" | "feePayer"),
            PayerValueNode::new(),
        )],
    );

    assert_eq!(
        default_value(&root, 0),
        Some(InstructionInputValueNode::PayerValue(PayerValueNode::new())),
    );
}

#[test]
fn fills_pda_seeds_in_the_default_value() {
    // An instruction with an account whose default is a PDA whose variable seeds
    // (a pubkey `authority` account + an `amount` argument) should be auto-filled.
    let mut ix = InstructionNode {
        name: "mint".into(),
        ..Default::default()
    };
    ix.accounts.push(InstructionAccountNode::new(
        "authority",
        false,
        IsSigner::True,
    ));
    ix.accounts.push(account("mintPda"));
    ix.arguments.push(InstructionArgumentNode::new(
        "amount",
        NumberTypeNode::le(U8),
    ));

    let pda = PdaNode::new(
        "myPda",
        vec![
            PdaSeedNode::Variable(VariablePdaSeedNode::new(
                "authority",
                PublicKeyTypeNode::new(),
            )),
            PdaSeedNode::Variable(VariablePdaSeedNode::new("amount", NumberTypeNode::le(U8))),
        ],
    );

    let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    program.pdas.push(pda);
    program.instructions.push(ix);

    let pda_default = PdaValueNode::new(
        PdaLinkNode {
            name: "myPda".into(),
            program: None,
        },
        vec![],
    );
    let root = set_instruction_account_default_values(
        RootNode::new(program),
        vec![InstructionAccountDefaultRule::new("mintPda", pda_default)],
    );

    let Some(InstructionInputValueNode::PdaValue(pda_value)) = default_value(&root, 1) else {
        panic!("expected a pdaValueNode default")
    };
    let names: Vec<String> = pda_value.seeds.iter().map(|s| s.name.to_string()).collect();
    assert_eq!(names, vec!["authority", "amount"]);
    assert!(matches!(
        &*pda_value.seeds[0].value,
        PdaSeedValueValue::Account(AccountValueNode { .. })
    ));
    assert!(matches!(
        &*pda_value.seeds[1].value,
        PdaSeedValueValue::Argument(ArgumentValueNode { .. })
    ));
}

#[test]
fn common_rules_assign_known_accounts() {
    let root = set_instruction_account_default_values(
        sample_root(),
        get_common_instruction_account_default_rules(),
    );

    assert_eq!(
        default_value(&root, 0),
        Some(InstructionInputValueNode::PayerValue(PayerValueNode::new())),
    );
    assert_eq!(
        default_value(&root, 1),
        Some(InstructionInputValueNode::IdentityValue(
            IdentityValueNode::new()
        )),
    );
    assert_eq!(
        default_value(&root, 2),
        Some(InstructionInputValueNode::PublicKeyValue(
            PublicKeyValueNode {
                public_key: "11111111111111111111111111111111".to_string(),
                identifier: Some("splSystem".into()),
            }
        )),
    );
}
