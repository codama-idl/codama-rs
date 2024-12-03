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
            RootNode::new(ProgramNode {
                defined_types: vec![membership, person],
                ..Default::default()
            })
            .into()
        )
    );
}

#[test]
fn it_merges_defined_types_inside_programs_into_root_nodes() {
    let program_a = ProgramNode {
        defined_types: vec![DefinedTypeNode::new("foo", NumberTypeNode::le(U32))],
        ..Default::default()
    };
    let program_b = ProgramNode {
        defined_types: vec![DefinedTypeNode::new("bar", NumberTypeNode::le(U32))],
        ..Default::default()
    };
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(program_a)
                .add_node(program_b)
        ),
        Some(
            RootNode::new(ProgramNode {
                defined_types: vec![
                    DefinedTypeNode::new("foo", NumberTypeNode::le(U32)),
                    DefinedTypeNode::new("bar", NumberTypeNode::le(U32))
                ],
                ..Default::default()
            })
            .into()
        )
    );
}
