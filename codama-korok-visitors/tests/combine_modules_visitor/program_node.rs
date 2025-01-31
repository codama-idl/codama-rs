use super::utils::{combine_modules, CombineModulesInput};
use codama_nodes::{
    AccountNode, DefinedTypeNode, Docs, ErrorNode, InstructionNode, Node, PdaNode, ProgramNode,
    RootNode, StructTypeNode,
};

#[test]
fn it_merges_node_arrays_together() {
    let program_a = get_mock_program("a", "1234");
    let program_b = get_mock_program("b", "1234");
    let result = combine_modules(
        CombineModulesInput::new()
            .add_node(program_a.clone())
            .add_node(program_b.clone()),
    );
    let Some(Node::Root(RootNode { program, .. })) = result else {
        panic!("Expected a RootNode");
    };

    assert_eq!(
        program.accounts,
        concat(program_a.accounts, program_b.accounts)
    );
    assert_eq!(
        program.instructions,
        concat(program_a.instructions, program_b.instructions)
    );
    assert_eq!(
        program.defined_types,
        concat(program_a.defined_types, program_b.defined_types)
    );
    assert_eq!(program.pdas, concat(program_a.pdas, program_b.pdas));
    assert_eq!(program.errors, concat(program_a.errors, program_b.errors));
}

#[test]
fn it_updates_the_program_data_when_not_set() {
    let program_a = ProgramNode {
        public_key: "1234".into(),
        ..ProgramNode::default()
    };
    let program_b = get_mock_program("b", "1234");
    let result = combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b.clone()),
    );
    let Some(Node::Root(RootNode { program, .. })) = result else {
        panic!("Expected a RootNode");
    };

    assert_eq!(program.name, program_b.name);
    assert_eq!(program.public_key, "1234");
    assert_eq!(program.version, program_b.version);
    assert_eq!(program.origin, program_b.origin);
    assert_eq!(program.docs, program_b.docs);
}

#[test]
fn it_does_not_override_the_program_data_when_set() {
    let program_a = get_mock_program("a", "1234");
    let program_b = get_mock_program("b", "1234");
    let result = combine_modules(
        CombineModulesInput::new()
            .add_node(program_a.clone())
            .add_node(program_b),
    );
    let Some(Node::Root(RootNode { program, .. })) = result else {
        panic!("Expected a RootNode");
    };

    assert_eq!(program.name, program_a.name);
    assert_eq!(program.public_key, program_a.public_key);
    assert_eq!(program.version, program_a.version);
    assert_eq!(program.origin, program_a.origin);
    assert_eq!(program.docs, program_a.docs);
}

/// Returns a ProgramNode with the given public key
/// whilst using an identifier to tag all fields with unique values.
fn get_mock_program(identifier: &str, public_key: &str) -> ProgramNode {
    ProgramNode {
        name: format!("program_{}", identifier).into(),
        public_key: public_key.into(),
        version: format!("version_{}", identifier),
        origin: Some(format!("origin_{}", identifier)),
        docs: Docs::new().add_doc(format!("docs_{}", identifier)),
        accounts: vec![AccountNode::new(
            format!("account_{}", identifier),
            StructTypeNode::default(),
        )],
        instructions: vec![InstructionNode {
            name: format!("instruction_{}", identifier).into(),
            ..InstructionNode::default()
        }],
        defined_types: vec![DefinedTypeNode::new(
            format!("defined_type_{}", identifier),
            StructTypeNode::default(),
        )],
        pdas: vec![PdaNode::new(format!("pda_{}", identifier), vec![])],
        errors: vec![ErrorNode::new(
            format!("error_{}", identifier),
            42,
            format!("message_{}", identifier),
        )],
    }
}

fn concat<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = a;
    result.extend(b);
    result
}
