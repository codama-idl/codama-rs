use codama_nodes::{InstructionArgumentNode, Node, TypeNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};
use std::collections::HashSet;

/// Inlines every instruction argument whose type is a struct, hoisting that
/// struct's fields up to be top-level arguments.
///
/// The Rust counterpart of `@codama/visitors`' `flattenInstructionDataArgumentsVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
/// If inlining would create duplicate argument names, the arguments are left
/// unchanged (upstream throws).
pub fn flatten_instruction_data_arguments() -> BottomUpTransformer {
    bottom_up_transformer(vec![TransformRule::new(
        "[instructionNode]",
        |node, _path| {
            let Node::Instruction(mut instruction) = node else {
                return node;
            };
            instruction.arguments = flatten_instruction_arguments(instruction.arguments);
            Node::Instruction(instruction)
        },
    )])
}

/// Flattens a list of instruction arguments by inlining every struct-typed
/// argument into its fields. If that would create duplicate names, the original
/// list is returned unchanged (upstream throws).
///
/// The Rust counterpart of `@codama/visitors`' `flattenInstructionArguments`
/// helper, exposed so other visitors (e.g.
/// [`create_sub_instructions_from_enum_args`](crate::create_sub_instructions_from_enum_args))
/// can reuse it.
pub fn flatten_instruction_arguments(
    arguments: Vec<InstructionArgumentNode>,
) -> Vec<InstructionArgumentNode> {
    let original = arguments.clone();
    let mut inlined: Vec<InstructionArgumentNode> = Vec::with_capacity(arguments.len());
    for argument in arguments {
        if matches!(&*argument.r#type, TypeNode::Struct(_)) {
            if let TypeNode::Struct(inner) = *argument.r#type {
                inlined.extend(inner.fields.into_iter().map(InstructionArgumentNode::from));
            }
        } else {
            inlined.push(argument);
        }
    }

    if has_duplicate_names(&inlined) {
        original
    } else {
        inlined
    }
}

fn has_duplicate_names(arguments: &[InstructionArgumentNode]) -> bool {
    let mut seen = HashSet::with_capacity(arguments.len());
    !arguments.iter().all(|a| seen.insert(&a.name))
}
