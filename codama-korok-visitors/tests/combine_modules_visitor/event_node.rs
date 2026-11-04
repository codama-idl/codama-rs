use super::utils::{combine_modules, try_combine_modules, CombineModulesInput};
use codama_nodes::test_utils::id;
use codama_nodes::{
    EventNode, IntegerFormat::U64, IntegerTypeNode, ProgramNode, RootNode, StructFieldTypeNode,
    StructTypeNode,
};

#[test]
fn it_merges_events_into_root_nodes() {
    let transfer = EventNode::new(id("transfer"), StructTypeNode::default());
    let burn = EventNode::new(id("burn"), StructTypeNode::default());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(transfer.clone())
                .add_node(burn.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_event(transfer).add_event(burn)).into())
    );
}

#[test]
fn it_merges_events_inside_programs_into_root_nodes() {
    let event_a = EventNode::new(id("foo"), StructTypeNode::default());
    let event_b = EventNode::new(id("bar"), StructTypeNode::default());
    let program_a = ProgramNode::default().add_event(event_a.clone());
    let program_b = ProgramNode::default().add_event(event_b.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_event(event_a).add_event(event_b)).into())
    );
}

#[test]
fn it_fails_on_events_with_identical_names() {
    let first = EventNode::new(id("duplicated"), StructTypeNode::default());
    let second = EventNode::new(
        id("duplicated"),
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            id("amount"),
            IntegerTypeNode::le(U64),
        )]),
    );
    let error = try_combine_modules(CombineModulesInput::new().add_node(first).add_node(second))
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `eventNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_events_with_identical_names_inside_programs() {
    let first = EventNode::new(id("duplicated"), StructTypeNode::default());
    let second = EventNode::new(
        id("duplicated"),
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            id("amount"),
            IntegerTypeNode::le(U64),
        )]),
    );
    let program_a = ProgramNode::default().add_event(first);
    let program_b = ProgramNode::default().add_event(second);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `eventNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_fails_on_events_with_identical_names_with_an_initial_program_node() {
    let first = EventNode::new(id("duplicated"), StructTypeNode::default());
    let second = EventNode::new(
        id("duplicated"),
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            id("amount"),
            IntegerTypeNode::le(U64),
        )]),
    );
    let program_a = ProgramNode::default().add_event(first);
    let program_b = ProgramNode::default().add_event(second);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .set_initial_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `eventNode` share the identifier `duplicated` in module `my_module`"
    );
}

#[test]
fn it_keeps_a_single_event_when_identical_names_have_equal_content() {
    let first = EventNode::new(id("duplicated"), StructTypeNode::default());
    let program = ProgramNode::default().add_event(first.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program.clone())
                .add_node(first.clone())
                .add_node(first.clone())
                .add_node(program.clone())
        ),
        Some(RootNode::new(program).into())
    );
}
