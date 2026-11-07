use super::utils::{combine_modules, get_plugin, try_combine_modules, CombineModulesInput};
use codama_nodes::{
    AccountNode, ConstantNode, DefinedTypeNode, ErrorNode, EventNode, Identifier, InstructionNode,
    IntegerTypeNode, IntegerValueNode, Node, PdaNode, PluginNode, ProgramNode, RootNode,
    StructTypeNode, U8,
};
use serde_json::json;

#[test]
fn it_merges_plugins_together() {
    let program_a = ProgramNode {
        plugins: vec![get_plugin("namespace.a")],
        ..get_mock_program("a", "1234")
    };
    let program_b = ProgramNode {
        plugins: vec![get_plugin("namespace.b")],
        ..get_mock_program("b", "1234")
    };
    let result = combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    );
    let Some(Node::Root(RootNode { program, .. })) = result else {
        panic!("Expected a RootNode");
    };

    assert_eq!(
        program.plugins,
        vec![get_plugin("namespace.a"), get_plugin("namespace.b")]
    );
}

#[test]
fn it_fails_on_plugins_with_the_same_namespace_and_different_payloads() {
    let plugin_a = PluginNode {
        payload: Some(json!({ "from": "a" })),
        ..get_plugin("namespace.shared")
    };
    let plugin_b = PluginNode {
        payload: Some(json!({ "from": "b" })),
        ..get_plugin("namespace.shared")
    };
    let program_a = ProgramNode {
        plugins: vec![plugin_a],
        ..get_mock_program("a", "1234")
    };
    let program_b = ProgramNode {
        plugins: vec![plugin_b, get_plugin("namespace.b")],
        ..get_mock_program("b", "1234")
    };
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different plugins share the namespace `namespace.shared` in program `program_a`"
    );
}

#[test]
fn it_keeps_a_single_plugin_when_namespaces_have_equal_payloads() {
    let plugin = PluginNode {
        payload: Some(json!({ "from": "a" })),
        ..get_plugin("namespace.shared")
    };
    let program_a = ProgramNode {
        plugins: vec![plugin.clone()],
        ..get_mock_program("a", "1234")
    };
    let program_b = ProgramNode {
        plugins: vec![plugin.clone(), get_plugin("namespace.b")],
        ..get_mock_program("b", "1234")
    };
    let result = combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    );
    let Some(Node::Root(RootNode { program, .. })) = result else {
        panic!("Expected a RootNode");
    };

    assert_eq!(program.plugins, vec![plugin, get_plugin("namespace.b")]);
}

#[test]
fn it_names_the_program_of_conflicting_nodes_by_identifier() {
    let program_a = get_mock_program("a", "1234");
    let program_b = ProgramNode {
        defined_types: vec![DefinedTypeNode::new(
            "defined_type_a",
            IntegerTypeNode::le(U8),
        )],
        ..get_mock_program("b", "1234")
    };
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `definedTypeNode` share the identifier `defined_type_a` in program `program_a`"
    );
}

#[test]
fn it_names_the_program_of_conflicting_nodes_by_public_key() {
    let program_a = ProgramNode {
        public_key: "1234".into(),
        ..ProgramNode::default()
    }
    .add_error(ErrorNode::new("duplicated", 1, "first"));
    let program_b = ProgramNode {
        public_key: "1234".into(),
        ..ProgramNode::default()
    }
    .add_error(ErrorNode::new("duplicated", 2, "second"));
    let error = try_combine_modules(
        CombineModulesInput::new()
            .add_node(program_a)
            .add_node(program_b),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Two different `errorNode` share the identifier `duplicated` in program `1234`"
    );
}

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
    assert_eq!(program.events, concat(program_a.events, program_b.events));
    assert_eq!(program.errors, concat(program_a.errors, program_b.errors));
    assert_eq!(
        program.constants,
        concat(program_a.constants, program_b.constants)
    );
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

    assert_eq!(program.identifier, program_b.identifier);
    assert_eq!(program.public_key, "1234");
    assert_eq!(program.version, program_b.version);
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

    assert_eq!(program.identifier, program_a.identifier);
    assert_eq!(program.public_key, program_a.public_key);
    assert_eq!(program.version, program_a.version);
    assert_eq!(program.docs, program_a.docs);
}

/// Returns a ProgramNode with the given public key
/// whilst using an identifier to tag all fields with unique values.
fn get_mock_program(identifier: &str, public_key: &str) -> ProgramNode {
    ProgramNode {
        identifier: Identifier::try_from(format!("program_{}", identifier)).unwrap(),
        public_key: public_key.into(),
        version: format!("version_{}", identifier),
        docs: vec![format!("docs_{}", identifier)].into(),
        accounts: vec![AccountNode::new(
            Identifier::try_from(format!("account_{}", identifier)).unwrap(),
            StructTypeNode::default(),
        )],
        instructions: vec![InstructionNode {
            identifier: Identifier::try_from(format!("instruction_{}", identifier)).unwrap(),
            ..InstructionNode::default()
        }],
        defined_types: vec![DefinedTypeNode::new(
            Identifier::try_from(format!("defined_type_{}", identifier)).unwrap(),
            StructTypeNode::default(),
        )],
        pdas: vec![PdaNode::new(
            Identifier::try_from(format!("pda_{}", identifier)).unwrap(),
            vec![],
        )],
        events: vec![EventNode::new(
            Identifier::try_from(format!("event_{}", identifier)).unwrap(),
            StructTypeNode::default(),
        )],
        errors: vec![ErrorNode::new(
            Identifier::try_from(format!("error_{}", identifier)).unwrap(),
            42,
            format!("message_{}", identifier),
        )],
        constants: vec![ConstantNode::new(
            Identifier::try_from(format!("constant_{}", identifier)).unwrap(),
            IntegerTypeNode::le(U8),
            IntegerValueNode::new(42u8),
        )],
        plugins: vec![],
    }
}

fn concat<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = a;
    result.extend(b);
    result
}
