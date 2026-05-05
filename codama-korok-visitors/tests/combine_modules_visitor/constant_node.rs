use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{ConstantNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, U64};

#[test]
fn it_merges_constants_into_root_nodes() {
    let max_items = ConstantNode::new(
        "maxItems",
        NumberTypeNode::le(U64),
        NumberValueNode::new(10u64),
    );
    let max_size = ConstantNode::new(
        "maxSize",
        NumberTypeNode::le(U64),
        NumberValueNode::new(20u64),
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(max_items.clone())
                .add_node(max_size.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_constant(max_items)
                    .add_constant(max_size)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_constants_inside_programs_into_root_nodes() {
    let constant_a = ConstantNode::new("foo", NumberTypeNode::le(U64), NumberValueNode::new(1u64));
    let constant_b = ConstantNode::new("bar", NumberTypeNode::le(U64), NumberValueNode::new(2u64));
    let program_a = ProgramNode::default().add_constant(constant_a.clone());
    let program_b = ProgramNode::default().add_constant(constant_b.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_constant(constant_a)
                    .add_constant(constant_b)
            )
            .into()
        )
    );
}

#[test]
fn it_deduplicates_constants_with_identical_names_by_using_the_last_one() {
    let first_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(2u64),
    );
    let third_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(3u64),
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first_constant)
                .add_node(second_constant)
                .add_node(third_constant.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_constant(third_constant)).into())
    );
}

#[test]
fn it_deduplicates_constants_with_identical_names_inside_programs() {
    let first_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(2u64),
    );
    let program_a = ProgramNode::default().add_constant(first_constant);
    let program_b = ProgramNode::default().add_constant(second_constant.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_constant(second_constant)).into())
    );
}

#[test]
fn it_deduplicates_constants_with_identical_names_with_an_initial_program_node() {
    let first_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        "duplicated",
        NumberTypeNode::le(U64),
        NumberValueNode::new(2u64),
    );
    let program_a = ProgramNode::default().add_constant(first_constant);
    let program_b = ProgramNode::default().add_constant(second_constant.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_constant(second_constant)).into())
    );
}
