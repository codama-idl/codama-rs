use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumTypeNode,
    NumberFormat::{U16, U32, U64},
    NumberTypeNode, ProgramNode, RootNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_merges_defined_types_into_root_nodes() {
    let membership = DefinedTypeNode::new(
        "membership",
        EnumTypeNode::new(vec![
            EnumEmptyVariantTypeNode::new("free").into(),
            EnumEmptyVariantTypeNode::new("premium").into(),
        ]),
    );
    let person = DefinedTypeNode::new(
        "person",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
        ]),
    );
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(membership.clone())
                .add_node(person.clone())
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(membership)
                    .add_defined_type(person)
            )
            .into()
        )
    );
}

#[test]
fn it_merges_defined_types_inside_programs_into_root_nodes() {
    let program_a = ProgramNode::default()
        .add_defined_type(DefinedTypeNode::new("foo", NumberTypeNode::le(U32)));
    let program_b = ProgramNode::default()
        .add_defined_type(DefinedTypeNode::new("bar", NumberTypeNode::le(U32)));
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(DefinedTypeNode::new("foo", NumberTypeNode::le(U32)))
                    .add_defined_type(DefinedTypeNode::new("bar", NumberTypeNode::le(U32)))
            )
            .into()
        )
    );
}

#[test]
fn it_deduplicates_defined_types_with_identical_names_by_using_the_last_one() {
    let first_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U16));
    let second_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U32));
    let third_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U64));
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(first_counter)
                .add_node(second_counter)
                .add_node(third_counter.clone())
        ),
        Some(RootNode::new(ProgramNode::default().add_defined_type(third_counter)).into())
    );
}

#[test]
fn it_deduplicates_defined_types_with_identical_names_inside_programs() {
    let first_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U32));
    let second_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U64));
    let program_a = ProgramNode::default().add_defined_type(first_counter);
    let program_b = ProgramNode::default().add_defined_type(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_defined_type(second_counter)).into())
    );
}

#[test]
fn it_deduplicates_defined_types_with_identical_names_with_an_initial_program_node() {
    let first_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U32));
    let second_counter = DefinedTypeNode::new("counter", NumberTypeNode::le(U64));
    let program_a = ProgramNode::default().add_defined_type(first_counter);
    let program_b = ProgramNode::default().add_defined_type(second_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program_a)
                .add_node(program_b)
        ),
        Some(RootNode::new(ProgramNode::default().add_defined_type(second_counter)).into())
    );
}
