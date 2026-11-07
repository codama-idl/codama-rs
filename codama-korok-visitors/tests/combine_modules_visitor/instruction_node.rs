use super::utils::{combine_modules, try_combine_modules, CombineModulesInput};
use codama_nodes::{
    BooleanTypeNode, Identifier, InstructionAccountNode, InstructionNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode,
};

fn create_instruction_with_single_field(name: &str, field_name: &str) -> InstructionNode {
    InstructionNode {
        identifier: name.parse::<Identifier>().unwrap(),
        data: Box::new(Some(
            StructTypeNode::new(vec![StructFieldTypeNode::new(
                field_name.parse::<Identifier>().unwrap(),
                BooleanTypeNode::default(),
            )])
            .into(),
        )),
        ..InstructionNode::default()
    }
}

#[test]
fn it_merges_instructions_into_root_nodes() {
    let create_instruction = InstructionNode {
        identifier: "create".into(),
        accounts: vec![
            InstructionAccountNode::new("payer", true, true),
            InstructionAccountNode::new("authority", false, false),
        ],
        ..InstructionNode::default()
    };
    let update_instruction = InstructionNode {
        identifier: "update".into(),
        accounts: vec![InstructionAccountNode::new("authority", false, true)],
        ..InstructionNode::default()
    };
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(create_instruction.clone())
                .add_node(update_instruction.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_instruction(create_instruction)
                    .add_instruction(update_instruction)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_instructions_inside_programs_into_root_nodes() {
    let foo_instruction = create_instruction_with_single_field("foo", "arg");
    let bar_instruction = create_instruction_with_single_field("bar", "arg");
    let program_a = ProgramNode::default().add_instruction(foo_instruction.clone());
    let program_b = ProgramNode::default().add_instruction(bar_instruction.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_instruction(foo_instruction)
                    .add_instruction(bar_instruction)
            )
            .into()
        )
    );
}

#[test]
fn it_fails_on_instructions_with_identical_names() {
    let first_instruction = create_instruction_with_single_field("create", "first");
    let second_instruction = create_instruction_with_single_field("create", "second");
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(first_instruction)
            .add_node(second_instruction),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `instructionNode` share the identifier `create` in module `my_module`"
    );
}

#[test]
fn it_fails_on_instructions_with_identical_names_inside_programs() {
    let first_instruction = create_instruction_with_single_field("create", "first");
    let second_instruction = create_instruction_with_single_field("create", "second");
    let program_a = ProgramNode::default().add_instruction(first_instruction);
    let program_b = ProgramNode::default().add_instruction(second_instruction);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `instructionNode` share the identifier `create` in module `my_module`"
    );
}

#[test]
fn it_fails_on_instructions_with_identical_names_with_an_initial_program_node() {
    let first_instruction = create_instruction_with_single_field("create", "first");
    let second_instruction = create_instruction_with_single_field("create", "second");
    let program_a = ProgramNode::default().add_instruction(first_instruction);
    let program_b = ProgramNode::default().add_instruction(second_instruction);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .set_initial_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `instructionNode` share the identifier `create` in module `my_module`"
    );
}

#[test]
fn it_keeps_a_single_instruction_when_identical_names_have_equal_content() {
    let first_instruction = create_instruction_with_single_field("create", "first");
    let program = ProgramNode::default().add_instruction(first_instruction.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program.clone())
                .add_node(first_instruction.clone())
                .add_node(first_instruction.clone())
                .add_node(program.clone())
        ),
        Some(RootNode::new(program).into())
    );
}
