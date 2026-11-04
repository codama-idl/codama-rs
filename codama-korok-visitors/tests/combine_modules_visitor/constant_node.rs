use super::utils::{combine_modules, try_combine_modules, CombineModulesInput};
use codama_nodes::test_utils::id;
use codama_nodes::{ConstantNode, IntegerTypeNode, IntegerValueNode, ProgramNode, RootNode, U64};

#[test]
fn it_merges_constants_into_root_nodes() {
    let max_items = ConstantNode::new(
        id("maxItems"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(10u64),
    );
    let max_size = ConstantNode::new(
        id("maxSize"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(20u64),
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
    let constant_a = ConstantNode::new(
        id("foo"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(1u64),
    );
    let constant_b = ConstantNode::new(
        id("bar"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(2u64),
    );
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
fn it_fails_on_constants_with_identical_names() {
    let first_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(2u64),
    );
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(first_constant)
            .add_node(second_constant),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `constantNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_constants_with_identical_names_inside_programs() {
    let first_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(2u64),
    );
    let program_a = ProgramNode::default().add_constant(first_constant);
    let program_b = ProgramNode::default().add_constant(second_constant);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `constantNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_constants_with_identical_names_with_an_initial_program_node() {
    let first_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(1u64),
    );
    let second_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(2u64),
    );
    let program_a = ProgramNode::default().add_constant(first_constant);
    let program_b = ProgramNode::default().add_constant(second_constant);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .set_initial_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `constantNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_keeps_a_single_constant_when_identical_names_have_equal_content() {
    let first_constant = ConstantNode::new(
        id("duplicated"),
        IntegerTypeNode::le(U64),
        IntegerValueNode::new(1u64),
    );
    let program = ProgramNode::default().add_constant(first_constant.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program.clone())
                .add_node(first_constant.clone())
                .add_node(first_constant.clone())
                .add_node(program.clone())
        ),
        Some(RootNode::new(program).into())
    );
}
