use codama_nodes::{
    AccountNode, DefinedTypeLinkNode, DefinedTypeNode, InstructionArgumentNode, InstructionNode,
    ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode,
};
use codama_visitors::{get_defined_type_histogram, DefinedTypeHistogramEntry};
use pretty_assertions::assert_eq;

fn link(name: &str) -> DefinedTypeLinkNode {
    DefinedTypeLinkNode {
        name: name.into(),
        program: None,
    }
}

#[test]
fn counts_references_by_context() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");

    // account: foo nested in a struct field
    program.accounts.push(AccountNode::new(
        "acc",
        StructTypeNode::new(vec![StructFieldTypeNode::new("ref", link("foo"))]),
    ));
    // defined type: foo as the body
    program
        .defined_types
        .push(DefinedTypeNode::new("bar", link("foo")));
    // instruction: foo directly as one arg, and nested in another
    let mut ix = InstructionNode {
        name: "ix".into(),
        ..Default::default()
    };
    ix.arguments
        .push(InstructionArgumentNode::new("x", link("foo")));
    ix.arguments.push(InstructionArgumentNode::new(
        "y",
        StructTypeNode::new(vec![StructFieldTypeNode::new("nested", link("foo"))]),
    ));
    program.instructions.push(ix);

    let histogram = get_defined_type_histogram(&RootNode::new(program));

    assert_eq!(
        histogram["p.foo"],
        DefinedTypeHistogramEntry {
            directly_as_instruction_args: 1, // arg `x`
            in_accounts: 1,
            in_defined_types: 1,
            in_events: 0,
            in_instruction_args: 2, // args `x` and `y`
            total: 4,
        },
    );
}

#[test]
fn unreferenced_types_are_absent() {
    let program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    let histogram = get_defined_type_histogram(&RootNode::new(program));
    assert!(histogram.is_empty());
}
