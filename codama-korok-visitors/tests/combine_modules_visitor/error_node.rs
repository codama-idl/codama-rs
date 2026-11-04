use super::utils::{combine_modules, try_combine_modules, CombineModulesInput};
use codama_nodes::{ErrorNode, ProgramNode, RootNode};

#[test]
fn it_merges_errors_into_root_nodes() {
    let wrong_account = ErrorNode::new("wrongAccount", 1, "This account is not valid");
    let wrong_argument = ErrorNode::new("wrongArgument", 2, "This argument is not valid");
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(wrong_account.clone())
                .add_node(wrong_argument.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_error(wrong_account)
                    .add_error(wrong_argument)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_errors_inside_programs_into_root_nodes() {
    let error_a = ErrorNode::new("foo", 1, "foo");
    let error_b = ErrorNode::new("bar", 2, "bar");
    let program_a = ProgramNode::default().add_error(error_a.clone());
    let program_b = ProgramNode::default().add_error(error_b.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_error(error_a).add_error(error_b)).into())
    );
}

#[test]
fn it_fails_on_errors_with_identical_names() {
    let first_error = ErrorNode::new("duplicated", 1, "first");
    let second_error = ErrorNode::new("duplicated", 2, "second");
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(first_error)
            .add_node(second_error),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `errorNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_errors_with_identical_names_inside_programs() {
    let first_error = ErrorNode::new("duplicated", 1, "first");
    let second_error = ErrorNode::new("duplicated", 2, "second");
    let program_a = ProgramNode::default().add_error(first_error);
    let program_b = ProgramNode::default().add_error(second_error);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `errorNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_errors_with_identical_names_with_an_initial_program_node() {
    let first_error = ErrorNode::new("duplicated", 1, "first");
    let second_error = ErrorNode::new("duplicated", 2, "second");
    let program_a = ProgramNode::default().add_error(first_error);
    let program_b = ProgramNode::default().add_error(second_error);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .set_initial_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `errorNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_keeps_a_single_error_when_identical_names_have_equal_content() {
    let first_error = ErrorNode::new("duplicated", 1, "first");
    let program = ProgramNode::default().add_error(first_error.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program.clone())
                .add_node(first_error.clone())
                .add_node(first_error.clone())
                .add_node(program.clone())
        ),
        Some(RootNode::new(program).into())
    );
}
