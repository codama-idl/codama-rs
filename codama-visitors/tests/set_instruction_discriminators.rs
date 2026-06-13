use codama_nodes::{
    DefaultValueStrategy, DiscriminatorNode, FieldDiscriminatorNode, InstructionInputValueNode,
    InstructionNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, TypeNode, U32, U8,
};
use codama_visitors::{set_instruction_discriminators, Discriminator, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    });
    program.instructions.push(InstructionNode {
        name: "close".into(),
        ..Default::default()
    });
    RootNode::new(program)
}

#[test]
fn prepends_a_default_u8_discriminator() {
    let mut visitor = set_instruction_discriminators([(
        "transfer",
        Discriminator::new(NumberValueNode::new(1u8)),
    )]);
    let root = visitor.visit_root(sample_root());
    let ix = &root.program.instructions[0];

    // A `discriminator` argument was prepended.
    assert_eq!(ix.arguments.len(), 1);
    let arg = &ix.arguments[0];
    assert_eq!(arg.name.as_ref(), "discriminator");
    assert_eq!(*arg.r#type, TypeNode::Number(NumberTypeNode::le(U8)));
    assert_eq!(
        arg.default_value_strategy,
        Some(DefaultValueStrategy::Omitted)
    );
    assert_eq!(
        *arg.default_value,
        Some(InstructionInputValueNode::NumberValue(
            NumberValueNode::new(1u8)
        )),
    );

    // A matching field discriminator was prepended.
    assert_eq!(
        ix.discriminators,
        vec![DiscriminatorNode::Field(FieldDiscriminatorNode::new(
            "discriminator",
            0
        ))],
    );
}

#[test]
fn respects_custom_name_type_and_strategy() {
    let mut visitor = set_instruction_discriminators([(
        "transfer",
        Discriminator::new(NumberValueNode::new(42u32))
            .name("ixType")
            .type_node(NumberTypeNode::le(U32))
            .strategy(DefaultValueStrategy::Optional),
    )]);
    let root = visitor.visit_root(sample_root());
    let arg = &root.program.instructions[0].arguments[0];

    assert_eq!(arg.name.as_ref(), "ixType");
    assert_eq!(*arg.r#type, TypeNode::Number(NumberTypeNode::le(U32)));
    assert_eq!(
        arg.default_value_strategy,
        Some(DefaultValueStrategy::Optional)
    );
    assert_eq!(
        root.program.instructions[0].discriminators,
        vec![DiscriminatorNode::Field(FieldDiscriminatorNode::new(
            "ixType", 0
        ))],
    );
}

#[test]
fn only_selected_instructions_are_changed() {
    let mut visitor = set_instruction_discriminators([(
        "transfer",
        Discriminator::new(NumberValueNode::new(1u8)),
    )]);
    let root = visitor.visit_root(sample_root());

    // `transfer` got a discriminator; `close` did not.
    assert_eq!(root.program.instructions[0].arguments.len(), 1);
    assert_eq!(root.program.instructions[1].arguments.len(), 0);
    assert!(root.program.instructions[1].discriminators.is_empty());
}
