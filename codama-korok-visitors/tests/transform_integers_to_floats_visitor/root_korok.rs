use super::utils::{
    account_with_field, float, instruction_with_account_default, integer,
    program_with_float_defined_type, transform_integers_to_floats,
};
use codama_korok_visitors::{KorokVisitable, TransformIntegersToFloatsVisitor};
use codama_koroks::RootKorok;
use codama_nodes::{
    ConstantNode, ConstantPdaSeedNode, DefinedTypeLinkNode, FloatFormat::F64, FloatTypeNode, Node,
    PdaLinkNode, PdaNode, PdaSeedValueNode, PdaValueNode, ProgramNode, RootNode,
    VariablePdaSeedNode,
};
use codama_stores::RootStore;

fn program_with_linked_integers() -> ProgramNode {
    let pda_value = PdaValueNode::new(
        PdaLinkNode::new("ratePda"),
        vec![PdaSeedValueNode::new("rate", integer())],
    );
    program_with_float_defined_type("rate")
        .add_account(account_with_field(
            DefinedTypeLinkNode::new("rate"),
            integer(),
        ))
        .add_constant(ConstantNode::new(
            "fee",
            DefinedTypeLinkNode::new("rate"),
            integer(),
        ))
        .add_pda(PdaNode::new(
            "ratePda",
            vec![
                ConstantPdaSeedNode::new(DefinedTypeLinkNode::new("rate"), integer()).into(),
                VariablePdaSeedNode::new("rate", DefinedTypeLinkNode::new("rate")).into(),
            ],
        ))
        .add_instruction(instruction_with_account_default(pda_value))
}

#[test]
fn a_second_run_gives_the_same_root() {
    let root = RootNode::new(program_with_linked_integers());
    let first = transform_integers_to_floats(root.clone());
    assert_ne!(first, root);
    assert_eq!(transform_integers_to_floats(first.clone()), first);
}

#[test]
fn only_a_root_node_changes() {
    let store = RootStore { crates: vec![] };
    let program = program_with_float_defined_type("rate")
        .add_account(account_with_field(FloatTypeNode::le(F64), integer()));
    for node in [None, Some(Node::from(program))] {
        let mut korok = RootKorok {
            crates: vec![],
            node: node.clone(),
            store: &store,
        };
        korok
            .accept(&mut TransformIntegersToFloatsVisitor::new())
            .unwrap();
        assert_eq!(korok.node, node);
    }
    let root = RootNode::new(
        ProgramNode::default().add_account(account_with_field(FloatTypeNode::le(F64), integer())),
    );
    let expected = RootNode::new(
        ProgramNode::default().add_account(account_with_field(FloatTypeNode::le(F64), float())),
    );
    assert_eq!(transform_integers_to_floats(root), expected);
}
