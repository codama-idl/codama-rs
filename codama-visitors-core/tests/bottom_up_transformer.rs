use codama_nodes::{
    AccountNode, ArrayTypeNode, BytesTypeNode, Endianness, InstructionArgumentNode,
    InstructionNode, NestedTypeNodeTrait, Node, NumberTypeNode, ProgramNode, RegisteredTypeNode,
    RootNode, StructFieldTypeNode, StructTypeNode, TypeNode, U32, U64, U8,
};
use codama_visitors_core::{bottom_up_transformer, TransformRule, TransformVisitor};
use pretty_assertions::assert_eq;

/// root -> program "myProgram" -> {
///   account "myAccount" { data: struct { counter: u32, blob: [u8; 4] } },
///   instruction "transfer" { arg "amount": u32 },
/// }
fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("counter", NumberTypeNode::le(U32)),
        StructFieldTypeNode::new("blob", ArrayTypeNode::fixed(NumberTypeNode::le(U8), 4)),
    ]);
    let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
    program.accounts.push(AccountNode::new("myAccount", data));

    let mut instruction = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    instruction.arguments.push(InstructionArgumentNode::new(
        "amount",
        NumberTypeNode::le(U32),
    ));
    program.instructions.push(instruction);

    RootNode::new(program)
}

/// Convenience: pull `program.accounts[0].data`'s field `i` resolved type.
fn account_field_type(root: &RootNode, i: usize) -> &TypeNode {
    let data: &StructTypeNode = root.program.accounts[0].data.get_nested_type_node();
    &data.fields[i].r#type
}

#[test]
fn selects_and_renames_a_named_node() {
    let mut t = bottom_up_transformer(vec![TransformRule::new(
        "[accountNode]myAccount",
        |node, _| match node {
            Node::Account(mut a) => {
                a.name = "renamed".into();
                Node::Account(a)
            }
            other => other,
        },
    )]);

    let root = t.visit_root(sample_root());
    assert_eq!(root.program.accounts[0].name.as_ref(), "renamed");
}

#[test]
fn applies_a_type_rule_across_the_whole_tree() {
    let mut t =
        bottom_up_transformer(vec![TransformRule::new(
            "[numberTypeNode]",
            |node, _| match node {
                Node::Type(RegisteredTypeNode::Number(mut n)) => {
                    n.endian = Endianness::Be;
                    Node::Type(RegisteredTypeNode::Number(n))
                }
                other => other,
            },
        )]);

    let root = t.visit_root(sample_root());
    let json = serde_json::to_string(&root).unwrap();
    // counter (u32), blob item (u8) and the instruction arg (u32) all flipped.
    assert!(!json.contains(r#""endian":"le""#), "{json}");
    assert!(json.contains(r#""endian":"be""#));
}

#[test]
fn ancestry_selector_scopes_the_transform() {
    // Only numbers under an instruction become u64; account numbers are untouched.
    let mut t = bottom_up_transformer(vec![TransformRule::new(
        "[instructionNode].[numberTypeNode]",
        |node, _| match node {
            Node::Type(RegisteredTypeNode::Number(mut n)) => {
                n.format = U64;
                Node::Type(RegisteredTypeNode::Number(n))
            }
            other => other,
        },
    )]);

    let root = t.visit_root(sample_root());

    // Instruction argument: changed to u64.
    match &*root.program.instructions[0].arguments[0].r#type {
        TypeNode::Number(n) => assert_eq!(n.format, U64),
        other => panic!("expected number, got {other:?}"),
    }
    // Account `counter`: untouched (still u32).
    match account_field_type(&root, 0) {
        TypeNode::Number(n) => assert_eq!(n.format, U32),
        other => panic!("expected number, got {other:?}"),
    }
}

#[test]
fn a_rule_can_change_a_node_kind_within_a_union() {
    // Replace any array type with a bytes type (a kind change inside the TypeNode union).
    let mut t =
        bottom_up_transformer(vec![TransformRule::new(
            "[arrayTypeNode]",
            |node, _| match node {
                Node::Type(RegisteredTypeNode::Array(_)) => {
                    Node::Type(RegisteredTypeNode::Bytes(BytesTypeNode::new()))
                }
                other => other,
            },
        )]);

    let root = t.visit_root(sample_root());
    // The `blob` field was `[u8; 4]`; it is now a bytesTypeNode.
    assert_eq!(
        account_field_type(&root, 1),
        &TypeNode::Bytes(BytesTypeNode::new())
    );
}

#[test]
fn no_matching_rule_leaves_the_tree_unchanged() {
    let mut t = bottom_up_transformer(vec![TransformRule::new(
        "[eventNode]",
        |node, _| node, // never matches this tree
    )]);

    let root = sample_root();
    let out = t.visit_root(root.clone());
    assert_eq!(
        serde_json::to_value(&out).unwrap(),
        serde_json::to_value(&root).unwrap(),
    );
}
