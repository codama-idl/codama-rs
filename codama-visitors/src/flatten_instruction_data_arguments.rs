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
    // First, check (by borrowing) whether inlining would collide -- so we can
    // bail out and return the arguments untouched without cloning them.
    let would_collide = {
        let mut seen = HashSet::new();
        !arguments.iter().all(|argument| match &*argument.r#type {
            TypeNode::Struct(inner) => inner.fields.iter().all(|field| seen.insert(&field.name)),
            _ => seen.insert(&argument.name),
        })
    };
    if would_collide {
        return arguments;
    }

    let mut inlined = Vec::with_capacity(arguments.len());
    for argument in arguments {
        if let TypeNode::Struct(inner) = *argument.r#type {
            inlined.extend(inner.fields.into_iter().map(InstructionArgumentNode::from));
        } else {
            inlined.push(argument);
        }
    }
    inlined
}
