use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    BooleanTypeNode, InstructionAccountNode, InstructionArgumentNode, InstructionNode, ProgramNode,
    RootNode,
};

fn create_instruction_with_single_arg(name: &str, arg_name: &str) -> InstructionNode {
    InstructionNode {
        name: name.into(),
        arguments: vec![InstructionArgumentNode::new(
            arg_name,
            BooleanTypeNode::default(),
        )],
        ..InstructionNode::default()
    }
}

#[test]
fn it_merges_instructions_into_root_nodes() {
    let create_instruction = InstructionNode {
        name: "create".into(),
        accounts: vec![
            InstructionAccountNode::new("payer", true, true),
            InstructionAccountNode::new("authority", false, false),
        ],
        ..InstructionNode::default()
    };
    let update_instruction = InstructionNode {
        name: "update".into(),
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
    let foo_instruction = create_instruction_with_single_arg("foo", "arg");
    let bar_instruction = create_instruction_with_single_arg("bar", "arg");
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
fn it_deduplicates_instructions_with_identical_names_by_using_the_last_one() {
    let first_instruction = create_instruction_with_single_arg("create", "first");
    let second_instruction = create_instruction_with_single_arg("create", "second");
    let third_instruction = create_instruction_with_single_arg("create", "third");
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first_instruction)
                .add_node(second_instruction)
                .add_node(third_instruction.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_instruction(third_instruction)).into())
    );
}

#[test]
fn it_deduplicates_instructions_with_identical_names_inside_programs() {
    let first_instruction = create_instruction_with_single_arg("create", "first");
    let second_instruction = create_instruction_with_single_arg("create", "second");
    let program_a = ProgramNode::default().add_instruction(first_instruction);
    let program_b = ProgramNode::default().add_instruction(second_instruction.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_instruction(second_instruction)).into())
    );
}

#[test]
fn it_deduplicates_instructions_with_identical_names_with_an_initial_program_node() {
    let first_instruction = create_instruction_with_single_arg("create", "first");
    let second_instruction = create_instruction_with_single_arg("create", "second");
    let program_a = ProgramNode::default().add_instruction(first_instruction.clone());
    let program_b = ProgramNode::default().add_instruction(second_instruction.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_instruction(second_instruction)).into())
    );
}
