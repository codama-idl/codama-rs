use crate::{get_defined_type_histogram, unwrap_defined_types_named};
use codama_nodes::{DefinedTypeLinkNode, RootNode, TypeNode};
use codama_visitors_core::LinkableDictionary;

/// Inlines defined types that are referenced exactly once, where that single
/// reference is an instruction argument's type used directly (not nested) --
/// except enums, which read better as standalone external types.
///
/// The Rust counterpart of `@codama/visitors`'
/// `unwrapInstructionArgsDefinedTypesVisitor`. It composes existing pieces:
/// [`get_defined_type_histogram`] finds the single-use direct-argument types, a
/// [`LinkableDictionary`] resolves each to skip enums, and finally
/// [`unwrap_defined_types_named`] inlines the survivors and removes them.
///
/// ```
/// use codama_nodes::{
///     DefinedTypeLinkNode, DefinedTypeNode, InstructionArgumentNode, InstructionNode,
///     NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, TypeNode, U8,
/// };
/// use codama_visitors::unwrap_instruction_args_defined_types;
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new(
///     "args",
///     StructTypeNode::new(vec![StructFieldTypeNode::new("amount", NumberTypeNode::le(U8))]),
/// ));
/// let mut ix = InstructionNode { name: "doStuff".into(), ..Default::default() };
/// ix.arguments.push(InstructionArgumentNode::new(
///     "data",
///     DefinedTypeLinkNode { name: "args".into(), program: None },
/// ));
/// program.instructions.push(ix);
///
/// let root = unwrap_instruction_args_defined_types(RootNode::new(program));
/// // `args` was used once, directly as an argument, and is not an enum -> inlined.
/// assert!(root.program.defined_types.is_empty());
/// assert!(matches!(
///     *root.program.instructions[0].arguments[0].r#type,
///     TypeNode::Struct(_)
/// ));
/// ```
pub fn unwrap_instruction_args_defined_types(root: RootNode) -> RootNode {
    let histogram = get_defined_type_histogram(&root);
    let linkables = LinkableDictionary::from_root(&root);

    let to_inline: Vec<String> = histogram
        .iter()
        .filter(|(_, entry)| entry.total == 1 && entry.directly_as_instruction_args == 1)
        .filter_map(|(key, _)| {
            let (program, name) = key.split_once('.')?;
            let link = DefinedTypeLinkNode {
                name: name.into(),
                program: None,
            };
            // Enums are better defined as external types -- leave them alone.
            match &*linkables.get_defined_type(&link, Some(program))?.r#type {
                TypeNode::Enum(_) => None,
                _ => Some(key.clone()),
            }
        })
        .collect();

    if to_inline.is_empty() {
        return root;
    }
    unwrap_defined_types_named(root, to_inline)
}
