use super::utils::{combine_modules, try_combine_modules, CombineModulesInput};
use codama_nodes::{
    DefinedTypeNode, EnumTypeNode, EnumVariantTypeNode,
    IntegerFormat::{U32, U64},
    IntegerTypeNode, ProgramNode, RootNode, StringTypeNode, StructFieldTypeNode, StructTypeNode,
};

#[test]
fn it_merges_defined_types_into_root_nodes() {
    let membership = DefinedTypeNode::new(
        "membership",
        EnumTypeNode::new(vec![
            EnumVariantTypeNode::new("free"),
            EnumVariantTypeNode::new("premium"),
        ]),
    );
    let person = DefinedTypeNode::new(
        "person",
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
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
        .add_defined_type(DefinedTypeNode::new("foo", IntegerTypeNode::le(U32)));
    let program_b = ProgramNode::default()
        .add_defined_type(DefinedTypeNode::new("bar", IntegerTypeNode::le(U32)));
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(
                ProgramNode::default()
                    .add_defined_type(DefinedTypeNode::new("foo", IntegerTypeNode::le(U32)))
                    .add_defined_type(DefinedTypeNode::new("bar", IntegerTypeNode::le(U32)))
            )
            .into()
        )
    );
}

#[test]
fn it_fails_on_defined_types_with_identical_names() {
    let first_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U32));
    let second_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U64));
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(first_counter)
            .add_node(second_counter),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `counter` in module `my_module`"
    );
}

#[test]
fn it_fails_on_defined_types_with_identical_names_inside_programs() {
    let first_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U32));
    let second_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U64));
    let program_a = ProgramNode::default().add_defined_type(first_counter);
    let program_b = ProgramNode::default().add_defined_type(second_counter);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `counter` in module `my_module`"
    );
}

#[test]
fn it_fails_on_defined_types_with_identical_names_with_an_initial_program_node() {
    let first_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U32));
    let second_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U64));
    let program_a = ProgramNode::default().add_defined_type(first_counter);
    let program_b = ProgramNode::default().add_defined_type(second_counter);
    let error = try_combine_modules(
        CombineModulesInput::new()
            .set_initial_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `counter` in module `my_module`"
    );
}

#[test]
fn it_keeps_a_single_defined_type_when_identical_names_have_equal_content() {
    let first_counter = DefinedTypeNode::new("counter", IntegerTypeNode::le(U32));
    let program = ProgramNode::default().add_defined_type(first_counter.clone());
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .set_initial_node(program.clone())
                .add_node(first_counter.clone())
                .add_node(first_counter.clone())
                .add_node(program.clone())
        ),
        Some(RootNode::new(program).into())
    );
}
