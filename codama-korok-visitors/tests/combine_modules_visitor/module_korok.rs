use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    DefinedTypeNode, EnumEmptyVariantTypeNode, EnumTypeNode, NumberTypeNode, ProgramNode, RootNode,
    StringTypeNode, StructFieldTypeNode, StructTypeNode, U32,
};

#[test]
fn it_merges_types_into_root_nodes() {
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
        combine_modules(CombineModulesInput {
            nodes: vec![Some(membership.clone().into()), Some(person.clone().into())],
            ..Default::default()
        }),
        Some(
            RootNode::new(ProgramNode {
                defined_types: vec![membership, person],
                ..Default::default()
            })
            .into()
        )
    );
}
