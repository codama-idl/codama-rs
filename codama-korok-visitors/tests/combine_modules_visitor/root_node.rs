use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{ProgramNode, RootNode};

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
                .add_node(RootNode::new(ProgramNode {
                    name: "foo".into(),
                    public_key: "1234".into(),
                    ..Default::default()
                }))
                .add_node(RootNode::new(ProgramNode {
                    name: "bar".into(),
                    public_key: "1234".into(),
                    ..Default::default()
                }))
        ),
        Some(
            RootNode::new(ProgramNode {
                name: "foo".into(),
                public_key: "1234".into(),
                ..Default::default()
            })
            .into()
        )
    );
}
