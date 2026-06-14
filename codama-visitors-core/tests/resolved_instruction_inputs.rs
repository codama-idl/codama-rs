use codama_nodes::{
    AccountBumpValueNode, AccountValueNode, InstructionAccountNode, InstructionArgumentNode,
    InstructionInputValueNode, InstructionNode, IsSigner, NumberTypeNode, PdaNode,
    PdaSeedValueNode, PdaValueNode, U8,
};
use codama_visitors_core::{
    get_resolved_instruction_inputs, InstructionDependency, ResolvedInstructionInput,
    ResolvedInstructionInputError,
};
use pretty_assertions::assert_eq;

fn account(name: &str, signer: IsSigner) -> InstructionAccountNode {
    InstructionAccountNode::new(name, false, signer)
}

fn with_default(
    mut account: InstructionAccountNode,
    value: InstructionInputValueNode,
) -> InstructionAccountNode {
    account.default_value = Box::new(Some(value));
    account
}

fn names(resolved: &[ResolvedInstructionInput]) -> Vec<String> {
    resolved
        .iter()
        .map(|r| match r {
            ResolvedInstructionInput::Account(a) => a.account.name.to_string(),
            ResolvedInstructionInput::Argument(a) => a.argument.name.to_string(),
        })
        .collect()
}

#[test]
fn resolves_dependencies_before_dependents() {
    // `ata` is declared first but depends (via a PDA seed) on `owner`.
    let pda_value = PdaValueNode::new(
        PdaNode::new("ataPda", vec![]),
        vec![PdaSeedValueNode::new(
            "owner",
            AccountValueNode::new("owner"),
        )],
    );
    let ata = with_default(
        account("ata", IsSigner::False),
        InstructionInputValueNode::PdaValue(pda_value),
    );
    let owner = account("owner", IsSigner::True);
    let ix = InstructionNode {
        name: "create".into(),
        accounts: vec![ata, owner],
        ..Default::default()
    };

    let resolved = get_resolved_instruction_inputs(&ix, false).unwrap();
    // `owner` is resolved before `ata` despite being declared after it.
    assert_eq!(names(&resolved), vec!["owner", "ata"]);

    let ResolvedInstructionInput::Account(ata) = &resolved[1] else {
        panic!("expected account")
    };
    assert_eq!(
        ata.depends_on,
        vec![InstructionDependency::Account("owner".to_string())]
    );
    // A PDA-defaulted account is never a signer-only / optional.
    assert!(!ata.resolved_is_optional);
}

#[test]
fn resolves_signer_from_an_account_value_default() {
    // `authority` defaults to `payer`; both are signers => resolved signer.
    let authority = with_default(
        account("authority", IsSigner::True),
        InstructionInputValueNode::AccountValue(AccountValueNode::new("payer")),
    );
    let payer = account("payer", IsSigner::True);
    let ix = InstructionNode {
        name: "transfer".into(),
        accounts: vec![authority, payer],
        ..Default::default()
    };

    let resolved = get_resolved_instruction_inputs(&ix, false).unwrap();
    let ResolvedInstructionInput::Account(authority) = resolved
        .iter()
        .find(|r| matches!(r, ResolvedInstructionInput::Account(a) if a.account.name.as_ref() == "authority"))
        .unwrap()
    else {
        unreachable!()
    };
    assert_eq!(authority.resolved_is_signer, IsSigner::True);
}

#[test]
fn flags_pda_accounts_from_account_bump_arguments() {
    let mut bump = InstructionArgumentNode::new("bump", NumberTypeNode::le(U8));
    bump.default_value = Box::new(Some(InstructionInputValueNode::AccountBumpValue(
        AccountBumpValueNode { name: "pda".into() },
    )));
    let ix = InstructionNode {
        name: "create".into(),
        accounts: vec![account("pda", IsSigner::False)],
        arguments: vec![bump],
        ..Default::default()
    };

    let resolved = get_resolved_instruction_inputs(&ix, false).unwrap();
    let ResolvedInstructionInput::Account(pda) = resolved
        .iter()
        .find(|r| matches!(r, ResolvedInstructionInput::Account(_)))
        .unwrap()
    else {
        unreachable!()
    };
    assert!(pda.is_pda);
}

#[test]
fn excludes_plain_value_data_arguments_by_default() {
    let mut amount = InstructionArgumentNode::new("amount", NumberTypeNode::le(U8));
    amount.default_value = Box::new(Some(InstructionInputValueNode::NumberValue(
        codama_nodes::NumberValueNode::new(5u8),
    )));
    let ix = InstructionNode {
        name: "transfer".into(),
        arguments: vec![amount],
        ..Default::default()
    };

    // Excluded by default...
    assert!(get_resolved_instruction_inputs(&ix, false)
        .unwrap()
        .is_empty());
    // ...but included when requested.
    assert_eq!(
        names(&get_resolved_instruction_inputs(&ix, true).unwrap()),
        vec!["amount"]
    );
}

#[test]
fn detects_cyclic_dependencies() {
    let a = with_default(
        account("a", IsSigner::False),
        InstructionInputValueNode::AccountValue(AccountValueNode::new("b")),
    );
    let b = with_default(
        account("b", IsSigner::False),
        InstructionInputValueNode::AccountValue(AccountValueNode::new("a")),
    );
    let ix = InstructionNode {
        name: "cyclic".into(),
        accounts: vec![a, b],
        ..Default::default()
    };

    assert!(matches!(
        get_resolved_instruction_inputs(&ix, false),
        Err(ResolvedInstructionInputError::CyclicDependency { .. })
    ));
}

#[test]
fn errors_on_an_invalid_dependency() {
    let a = with_default(
        account("a", IsSigner::False),
        InstructionInputValueNode::AccountValue(AccountValueNode::new("ghost")),
    );
    let ix = InstructionNode {
        name: "broken".into(),
        accounts: vec![a],
        ..Default::default()
    };

    assert!(matches!(
        get_resolved_instruction_inputs(&ix, false),
        Err(ResolvedInstructionInputError::InvalidDependency { .. })
    ));
}
