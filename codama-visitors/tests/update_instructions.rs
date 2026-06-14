use codama_nodes::{
    InstructionAccountNode, InstructionArgumentNode, InstructionInputValueNode, InstructionNode,
    IsSigner, NumberTypeNode, PdaLinkNode, PdaNode, PdaSeedNode, PdaValueNode, ProgramNode,
    PublicKeyTypeNode, RootNode, TypeNode, VariablePdaSeedNode, U32, U8,
};
use codama_visitors::{
    update_instructions, InstructionAccountUpdate, InstructionArgumentUpdate, InstructionUpdate,
};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let mut ix = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    ix.accounts
        .push(InstructionAccountNode::new("payer", false, IsSigner::False));
    ix.arguments
        .push(InstructionArgumentNode::new("amt", NumberTypeNode::le(U8)));
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(ix);
    RootNode::new(program)
}

#[test]
fn updates_metadata_accounts_and_arguments() {
    let root = update_instructions(
        sample_root(),
        [(
            "transfer",
            InstructionUpdate::new()
                .name("send")
                .account(
                    "payer",
                    InstructionAccountUpdate::new()
                        .name("authority")
                        .signer(IsSigner::True)
                        .writable(true),
                )
                .argument(
                    "amt",
                    InstructionArgumentUpdate::new()
                        .name("amount")
                        .type_node(NumberTypeNode::le(U32)),
                ),
        )],
    );

    let ix = &root.program.instructions[0];
    assert_eq!(ix.name.as_ref(), "send");

    let account = &ix.accounts[0];
    assert_eq!(account.name.as_ref(), "authority");
    assert_eq!(account.is_signer, IsSigner::True);
    assert!(account.is_writable);

    let argument = &ix.arguments[0];
    assert_eq!(argument.name.as_ref(), "amount");
    assert_eq!(*argument.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
}

#[test]
fn only_named_instructions_change() {
    let root = update_instructions(
        sample_root(),
        [("other", InstructionUpdate::new().name("changed"))],
    );
    assert_eq!(root.program.instructions[0].name.as_ref(), "transfer");
}

#[test]
fn fills_pda_seeds_in_an_account_default_value() {
    let mut ix = InstructionNode {
        name: "mint".into(),
        ..Default::default()
    };
    ix.accounts.push(InstructionAccountNode::new(
        "authority",
        false,
        IsSigner::True,
    ));
    ix.accounts.push(InstructionAccountNode::new(
        "mintPda",
        false,
        IsSigner::False,
    ));
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
    let root = update_instructions(
        RootNode::new(program),
        [(
            "mint",
            InstructionUpdate::new().account(
                "mintPda",
                InstructionAccountUpdate::new().default_value(pda_default),
            ),
        )],
    );

    let account = &root.program.instructions[0].accounts[1];
    let Some(InstructionInputValueNode::PdaValue(pda_value)) = account.default_value.as_ref()
    else {
        panic!("expected a pdaValueNode default")
    };
    let names: Vec<String> = pda_value.seeds.iter().map(|s| s.name.to_string()).collect();
    assert_eq!(names, vec!["authority", "amount"]);
}

#[test]
fn injects_a_new_extra_argument() {
    let root = update_instructions(
        sample_root(),
        [(
            "transfer",
            InstructionUpdate::new().argument(
                "bump",
                InstructionArgumentUpdate::new().type_node(NumberTypeNode::le(U8)),
            ),
        )],
    );

    let ix = &root.program.instructions[0];
    // `bump` matched no existing argument, so it becomes a new extra argument.
    assert_eq!(ix.extra_arguments.len(), 1);
    assert_eq!(ix.extra_arguments[0].name.as_ref(), "bump");
}

#[test]
fn deletes_an_instruction() {
    let root = update_instructions(
        sample_root(),
        [("transfer", InstructionUpdate::new().delete())],
    );
    assert!(root.program.instructions.is_empty());
}
