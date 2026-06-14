use codama_nodes::{
    InstructionAccountNode, InstructionArgumentNode, InstructionInputValueNode, InstructionNode,
    IsSigner, NumberTypeNode, PdaLinkNode, PdaNode, PdaSeedNode, PdaSeedValueValue, PdaValueNode,
    ProgramNode, PublicKeyTypeNode, RootNode, VariablePdaSeedNode, U8,
};
use codama_visitors::{fill_default_pda_seed_values, InvalidPdaSeedValues};
use codama_visitors_core::LinkableDictionary;
use pretty_assertions::assert_eq;

fn variable(name: &str, type_node: impl Into<codama_nodes::TypeNode>) -> PdaSeedNode {
    PdaSeedNode::Variable(VariablePdaSeedNode::new(name, type_node))
}

fn sample_instruction() -> InstructionNode {
    let mut ix = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    ix.accounts.push(InstructionAccountNode::new(
        "authority",
        false,
        IsSigner::True,
    ));
    ix.arguments.push(InstructionArgumentNode::new(
        "amount",
        NumberTypeNode::le(U8),
    ));
    ix
}

fn seed_names(pda_value: &PdaValueNode) -> Vec<String> {
    pda_value.seeds.iter().map(|s| s.name.to_string()).collect()
}

#[test]
fn fills_pubkey_seed_from_account_and_other_seed_from_argument() {
    let pda = PdaNode::new(
        "myPda",
        vec![
            variable("authority", PublicKeyTypeNode::new()),
            variable("amount", NumberTypeNode::le(U8)),
        ],
    );
    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(pda, vec![]));

    let filled = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &LinkableDictionary::default(),
        None,
        false,
    )
    .unwrap();

    let InstructionInputValueNode::PdaValue(pda_value) = filled else {
        panic!("expected pdaValueNode")
    };
    assert_eq!(seed_names(&pda_value), vec!["authority", "amount"]);
    assert!(matches!(
        &*pda_value.seeds[0].value,
        PdaSeedValueValue::Account(_)
    ));
    assert!(matches!(
        &*pda_value.seeds[1].value,
        PdaSeedValueValue::Argument(_)
    ));
}

#[test]
fn keeps_existing_seeds_and_skips_unmatched_seeds() {
    // `authority` already provided; `ghost` has no matching account/argument.
    let pda = PdaNode::new(
        "myPda",
        vec![
            variable("authority", PublicKeyTypeNode::new()),
            variable("ghost", NumberTypeNode::le(U8)),
        ],
    );
    let existing = vec![codama_nodes::PdaSeedValueNode::new(
        "authority",
        codama_nodes::AccountValueNode::new("authority"),
    )];
    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(pda, existing));

    let filled = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &LinkableDictionary::default(),
        None,
        false,
    )
    .unwrap();

    let InstructionInputValueNode::PdaValue(pda_value) = filled else {
        panic!("expected pdaValueNode")
    };
    // Only the pre-existing `authority` seed; `ghost` is skipped (no default).
    assert_eq!(seed_names(&pda_value), vec!["authority"]);
}

#[test]
fn resolves_pda_through_the_linkable_dictionary() {
    let pda = PdaNode::new(
        "myPda",
        vec![variable("authority", PublicKeyTypeNode::new())],
    );
    let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    program.pdas.push(pda);
    let linkables = LinkableDictionary::from_root(&RootNode::new(program));

    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(
        PdaLinkNode {
            name: "myPda".into(),
            program: None,
        },
        vec![],
    ));

    let filled = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &linkables,
        Some("myProgram"),
        false,
    )
    .unwrap();

    let InstructionInputValueNode::PdaValue(pda_value) = filled else {
        panic!("expected pdaValueNode")
    };
    assert_eq!(seed_names(&pda_value), vec!["authority"]);
}

#[test]
fn unresolvable_pda_link_is_left_untouched() {
    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(
        PdaLinkNode {
            name: "ghostPda".into(),
            program: None,
        },
        vec![],
    ));

    let filled = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &LinkableDictionary::default(),
        Some("myProgram"),
        false,
    )
    .unwrap();

    let InstructionInputValueNode::PdaValue(pda_value) = filled else {
        panic!("expected pdaValueNode")
    };
    assert!(pda_value.seeds.is_empty());
}

#[test]
fn strict_mode_errors_when_a_variable_seed_cannot_be_filled() {
    let pda = PdaNode::new(
        "myPda",
        vec![
            variable("authority", PublicKeyTypeNode::new()),
            variable("ghost", NumberTypeNode::le(U8)),
        ],
    );
    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(pda, vec![]));

    let result = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &LinkableDictionary::default(),
        None,
        true,
    );

    assert_eq!(
        result,
        Err(InvalidPdaSeedValues {
            instruction_name: "transfer".to_string(),
            pda_name: "myPda".to_string(),
        })
    );
}

#[test]
fn strict_mode_succeeds_when_all_variable_seeds_are_filled() {
    let pda = PdaNode::new(
        "myPda",
        vec![
            variable("authority", PublicKeyTypeNode::new()),
            variable("amount", NumberTypeNode::le(U8)),
        ],
    );
    let value = InstructionInputValueNode::PdaValue(PdaValueNode::new(pda, vec![]));

    let result = fill_default_pda_seed_values(
        value,
        &sample_instruction(),
        &LinkableDictionary::default(),
        None,
        true,
    );

    assert!(result.is_ok());
}
