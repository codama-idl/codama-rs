use codama_nodes::{
    DefinedTypeLinkNode, DefinedTypeNode, EnumEmptyVariantTypeNode, EnumTypeNode,
    InstructionArgumentNode, InstructionNode, NumberTypeNode, ProgramNode, RootNode,
    StructFieldTypeNode, StructTypeNode, TypeNode, U8,
};
use codama_visitors::unwrap_instruction_args_defined_types;
use pretty_assertions::assert_eq;

fn link(name: &str) -> DefinedTypeLinkNode {
    DefinedTypeLinkNode {
        name: name.into(),
        program: None,
    }
}

fn argument(name: &str, type_node: impl Into<TypeNode>) -> InstructionArgumentNode {
    InstructionArgumentNode::new(name, type_node)
}

fn defined_type_names(root: &RootNode) -> Vec<String> {
    root.program
        .defined_types
        .iter()
        .map(|d| d.name.to_string())
        .collect()
}

#[test]
fn inlines_single_use_direct_argument_struct() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.defined_types.push(DefinedTypeNode::new(
        "args",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U8),
        )]),
    ));
    let mut ix = InstructionNode {
        name: "doStuff".into(),
        ..Default::default()
    };
    ix.arguments.push(argument("data", link("args")));
    program.instructions.push(ix);

    let root = unwrap_instruction_args_defined_types(RootNode::new(program));

    assert!(defined_type_names(&root).is_empty());
    assert!(matches!(
        *root.program.instructions[0].arguments[0].r#type,
        TypeNode::Struct(_)
    ));
}

#[test]
fn keeps_enums_even_when_used_once_directly() {
    let enum_type = EnumTypeNode::new(vec![EnumEmptyVariantTypeNode::new("a").into()]);
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program
        .defined_types
        .push(DefinedTypeNode::new("mode", enum_type));
    let mut ix = InstructionNode {
        name: "doStuff".into(),
        ..Default::default()
    };
    ix.arguments.push(argument("data", link("mode")));
    program.instructions.push(ix);

    let root = unwrap_instruction_args_defined_types(RootNode::new(program));

    // Enum is kept, and the argument still links to it.
    assert_eq!(defined_type_names(&root), vec!["mode".to_string()]);
    assert!(matches!(
        *root.program.instructions[0].arguments[0].r#type,
        TypeNode::Link(_)
    ));
}

#[test]
fn keeps_types_used_more_than_once() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.defined_types.push(DefinedTypeNode::new(
        "args",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U8),
        )]),
    ));
    let mut ix = InstructionNode {
        name: "doStuff".into(),
        ..Default::default()
    };
    // Two direct uses -> total == 2, so not inlined.
    ix.arguments.push(argument("a", link("args")));
    ix.arguments.push(argument("b", link("args")));
    program.instructions.push(ix);

    let root = unwrap_instruction_args_defined_types(RootNode::new(program));

    assert_eq!(defined_type_names(&root), vec!["args".to_string()]);
}

#[test]
fn keeps_types_not_used_directly_as_an_argument() {
    let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
    program.defined_types.push(DefinedTypeNode::new(
        "inner",
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            NumberTypeNode::le(U8),
        )]),
    ));
    let mut ix = InstructionNode {
        name: "doStuff".into(),
        ..Default::default()
    };
    // Nested inside a struct argument -> directly_as_instruction_args == 0.
    ix.arguments.push(argument(
        "data",
        StructTypeNode::new(vec![StructFieldTypeNode::new("x", link("inner"))]),
    ));
    program.instructions.push(ix);

    let root = unwrap_instruction_args_defined_types(RootNode::new(program));

    assert_eq!(defined_type_names(&root), vec!["inner".to_string()]);
}
