use codama_nodes::{
    AccountNode, DefinedTypeNode, ErrorNode, InstructionArgumentNode, InstructionNode,
    NumberTypeNode, ProgramNode, RootNode, StructTypeNode, U32,
};
use codama_visitors_core::{delete_nodes, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .accounts
        .push(AccountNode::new("keep", StructTypeNode::new(vec![])));
    program
        .accounts
        .push(AccountNode::new("drop", StructTypeNode::new(vec![])));
    program
        .defined_types
        .push(DefinedTypeNode::new("aType", NumberTypeNode::le(U32)));
    program.errors.push(ErrorNode {
        name: "someError".into(),
        code: 1,
        message: "boom".into(),
        docs: Default::default(),
    });

    let mut instruction = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    instruction.arguments.push(InstructionArgumentNode::new(
        "amount",
        NumberTypeNode::le(U32),
    ));
    instruction.arguments.push(InstructionArgumentNode::new(
        "bump",
        NumberTypeNode::le(U32),
    ));
    program.instructions.push(instruction);

    RootNode::new(program)
}

#[test]
fn deletes_a_named_account() {
    let root = delete_nodes(["[accountNode]drop"]).visit_root(sample_root());
    let names: Vec<_> = root
        .program
        .accounts
        .iter()
        .map(|a| a.name.to_string())
        .collect();
    assert_eq!(names, vec!["keep".to_string()]);
}

#[test]
fn deletes_nodes_of_different_kinds_and_nested_args() {
    let root = delete_nodes([
        "[definedTypeNode]aType",
        "[errorNode]someError",
        "[instructionArgumentNode]bump",
    ])
    .visit_root(sample_root());

    assert!(root.program.defined_types.is_empty());
    assert!(root.program.errors.is_empty());
    // The `bump` argument is gone but `amount` remains.
    let args: Vec<_> = root.program.instructions[0]
        .arguments
        .iter()
        .map(|a| a.name.to_string())
        .collect();
    assert_eq!(args, vec!["amount".to_string()]);
}

#[test]
fn ancestry_selector_scopes_the_deletion() {
    // Only delete `amount` arguments that belong to the `transfer` instruction.
    let root = delete_nodes(["[instructionNode]transfer.[instructionArgumentNode]amount"])
        .visit_root(sample_root());
    let args: Vec<_> = root.program.instructions[0]
        .arguments
        .iter()
        .map(|a| a.name.to_string())
        .collect();
    assert_eq!(args, vec!["bump".to_string()]);
}

#[test]
fn non_matching_selector_keeps_everything() {
    let root = delete_nodes(["[accountNode]missing"]).visit_root(sample_root());
    assert_eq!(root.program.accounts.len(), 2);
}
