use codama_nodes::{
    InstructionArgumentNode, InstructionNode, NumberTypeNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, U8,
};
use codama_visitors::{flatten_instruction_data_arguments, TransformVisitor};
use pretty_assertions::assert_eq;

fn sample_root() -> RootNode {
    let data = StructTypeNode::new(vec![
        StructFieldTypeNode::new("x", NumberTypeNode::le(U8)),
        StructFieldTypeNode::new("y", NumberTypeNode::le(U8)),
    ]);
    let mut instruction = InstructionNode {
        name: "transfer".into(),
        ..Default::default()
    };
    instruction
        .arguments
        .push(InstructionArgumentNode::new("data", data));
    instruction
        .arguments
        .push(InstructionArgumentNode::new("z", NumberTypeNode::le(U8)));

    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.instructions.push(instruction);
    RootNode::new(program)
}

#[test]
fn hoists_struct_argument_fields_to_top_level() {
    let root = flatten_instruction_data_arguments().visit_root(sample_root());
    let names: Vec<_> = root.program.instructions[0]
        .arguments
        .iter()
        .map(|a| a.name.to_string())
        .collect();
    assert_eq!(
        names,
        vec!["x".to_string(), "y".to_string(), "z".to_string()]
    );
}
