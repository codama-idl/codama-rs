use codama_nodes::{
    InstructionAccountNode, InstructionArgumentNode, InstructionNode, IsSigner, NumberTypeNode,
    ProgramNode, RootNode, TypeNode, U32, U8,
};
use codama_visitors::{
    update_instructions, InstructionAccountUpdate, InstructionArgumentUpdate, InstructionUpdate,
    TransformVisitor,
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
    let root = update_instructions([(
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
    )])
    .visit_root(sample_root());

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
    let root = update_instructions([("other", InstructionUpdate::new().name("changed"))])
        .visit_root(sample_root());
    assert_eq!(root.program.instructions[0].name.as_ref(), "transfer");
}
