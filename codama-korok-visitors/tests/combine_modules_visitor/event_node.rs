use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    EventNode,
    NumberFormat::{U32, U64},
    NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_merges_events_into_root_nodes() {
    let transfer = EventNode::new("transfer", StructTypeNode::default());
    let burn = EventNode::new("burn", StructTypeNode::default());
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
    let event_a = EventNode::new("foo", StructTypeNode::default());
    let event_b = EventNode::new("bar", StructTypeNode::default());
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
fn it_deduplicates_events_with_identical_names_by_using_the_last_one() {
    let first = EventNode::new("duplicated", StructTypeNode::default());
    let second = EventNode::new(
        "duplicated",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U64),
        )]),
    );
    let third = EventNode::new(
        "duplicated",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "value",
            NumberTypeNode::le(U32),
        )]),
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first)
                .add_node(second)
                .add_node(third.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_event(third)).into())
    );
}

#[test]
fn it_deduplicates_events_with_identical_names_inside_programs() {
    let first = EventNode::new("duplicated", StructTypeNode::default());
    let second = EventNode::new(
        "duplicated",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U64),
        )]),
    );
    let program_a = ProgramNode::default().add_event(first);
    let program_b = ProgramNode::default().add_event(second.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_event(second)).into())
    );
}

#[test]
fn it_deduplicates_events_with_identical_names_with_an_initial_program_node() {
    let first = EventNode::new("duplicated", StructTypeNode::default());
    let second = EventNode::new(
        "duplicated",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U64),
        )]),
    );
    let program_a = ProgramNode::default().add_event(first);
    let program_b = ProgramNode::default().add_event(second.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_event(second)).into())
    );
}
