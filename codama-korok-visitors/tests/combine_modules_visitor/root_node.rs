use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{DefinedTypeNode, NumberFormat::U32, NumberTypeNode, ProgramNode, RootNode};

#[test]
fn default_roots() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(RootNode::default())
                .add_node(RootNode::default())
        ),
        Some(RootNode::default().into())
    );
}

#[test]
fn roots_with_same_pubkey_programs() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(RootNode::new(ProgramNode::new("foo", "1234")))
                .add_node(RootNode::new(ProgramNode::new("bar", "1234")))
        ),
        Some(RootNode::new(ProgramNode::new("foo", "1234")).into())
    );
}

#[test]
fn defined_root_within_scraps() {
    assert_eq!(
        combine_modules(
            CombineModulesInput::new()
                .add_node(DefinedTypeNode::new("foo", NumberTypeNode::le(U32)))
                .add_node(RootNode::new(ProgramNode::new("my_program", "1234")))
                .add_node(DefinedTypeNode::new("bar", NumberTypeNode::le(U32)))
        ),
        Some(
            RootNode {
                program: ProgramNode::new("my_program", "1234"),
                additional_programs: vec![ProgramNode {
                    defined_types: vec![
                        DefinedTypeNode::new("foo", NumberTypeNode::le(U32)),
                        DefinedTypeNode::new("bar", NumberTypeNode::le(U32))
                    ],
                    ..Default::default()
                }]
            }
            .into()
        )
    );
}
