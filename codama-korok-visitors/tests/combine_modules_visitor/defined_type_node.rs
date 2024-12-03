use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumTypeNode, NumberTypeNode, ProgramNode, RootNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, U32,
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
