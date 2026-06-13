use codama_nodes::{
    IdentityValueNode, InstructionAccountNode, InstructionInputValueNode, InstructionNode,
    IsSigner, PayerValueNode, ProgramNode, RootNode,
};
use codama_visitors::{
    set_instruction_account_default_values, InstructionAccountDefaultRule, TransformVisitor,
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
    let mut visitor = set_instruction_account_default_values(vec![
        InstructionAccountDefaultRule::new("payer", PayerValueNode::new()),
        InstructionAccountDefaultRule::new("authority", IdentityValueNode::new()),
    ]);
    let root = visitor.visit_root(sample_root());

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

    let mut visitor =
        set_instruction_account_default_values(vec![InstructionAccountDefaultRule::new(
            "payer",
            PayerValueNode::new(),
        )
        .ignore_if_optional()]);
    let root = visitor.visit_root(root);

    assert_eq!(default_value(&root, 0), None);
}

#[test]
fn instruction_scoped_rules_only_apply_to_that_instruction() {
    let mut visitor =
        set_instruction_account_default_values(vec![InstructionAccountDefaultRule::new(
            "payer",
            PayerValueNode::new(),
        )
        .instruction("other")]);
    let root = visitor.visit_root(sample_root());

    // The rule is scoped to `other`, so `transfer`'s payer is untouched.
    assert_eq!(default_value(&root, 0), None);
}

#[test]
fn predicate_matcher_matches_multiple_names() {
    let mut visitor =
        set_instruction_account_default_values(vec![InstructionAccountDefaultRule::matching(
            |name| matches!(name, "payer" | "feePayer"),
            PayerValueNode::new(),
        )]);
    let root = visitor.visit_root(sample_root());

    assert_eq!(
        default_value(&root, 0),
        Some(InstructionInputValueNode::PayerValue(PayerValueNode::new())),
    );
}
