use crate::flatten_instruction_arguments;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, EnumTypeNode, EnumVariantTypeNode,
    InstructionArgumentNode, InstructionInputValueNode, InstructionNode, Node, NumberTypeNode,
    NumberValueNode, RootNode, TypeNode, U8,
};
use codama_visitors_core::{
    bottom_up_transformer, LinkableDictionary, TransformRule, TransformVisitor,
};
use std::rc::Rc;

/// Splits selected instructions that carry an enum argument into one
/// sub-instruction per enum variant.
///
/// `map` pairs an instruction selector (typically the instruction name) with the
/// name of an enum argument on that instruction. For each variant, a
/// sub-instruction is created that: keeps the arguments before the enum arg,
/// inserts an omitted `u8` discriminator argument defaulting to the variant
/// index, replaces the enum arg with the variant's payload (a struct variant is
/// flattened into its fields; a tuple variant becomes a single tuple argument;
/// an empty variant contributes nothing), and keeps the arguments after the enum
/// arg. The parent instruction is left intact, gaining the new sub-instructions.
///
/// The enum type may be given inline or via a `definedTypeLinkNode` resolved
/// through a [`LinkableDictionary`] built from `root`. Unlike upstream (which
/// throws), if the named argument is missing or does not resolve to an enum the
/// instruction is left unchanged.
///
/// The Rust counterpart of `@codama/visitors`'
/// `createSubInstructionsFromEnumArgsVisitor`.
///
/// ```
/// use codama_nodes::{
///     EnumEmptyVariantTypeNode, EnumTypeNode, InstructionArgumentNode, InstructionNode,
///     ProgramNode, RootNode,
/// };
/// use codama_visitors::create_sub_instructions_from_enum_args;
///
/// let enum_type = EnumTypeNode::new(vec![
///     EnumEmptyVariantTypeNode::new("add").into(),
///     EnumEmptyVariantTypeNode::new("remove").into(),
/// ]);
/// let mut ix = InstructionNode { name: "update".into(), ..Default::default() };
/// ix.arguments.push(InstructionArgumentNode::new("action", enum_type));
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(ix);
///
/// let root = create_sub_instructions_from_enum_args(RootNode::new(program), [("update", "action")]);
/// let subs = &root.program.instructions[0].sub_instructions;
/// assert_eq!(subs.len(), 2);
/// assert_eq!(subs[0].name.as_ref(), "updateAdd");
/// assert_eq!(subs[1].name.as_ref(), "updateRemove");
/// ```
pub fn create_sub_instructions_from_enum_args<K, V>(
    root: RootNode,
    map: impl IntoIterator<Item = (K, V)>,
) -> RootNode
where
    K: Into<String>,
    V: Into<String>,
{
    let linkables = Rc::new(LinkableDictionary::from_root(&root));
    let rules = map
        .into_iter()
        .map(|(selector, arg_name)| {
            let linkables = Rc::clone(&linkables);
            let arg_name = CamelCaseString::new(arg_name.into());
            TransformRule::new(
                format!("[instructionNode]{}", selector.into()),
                move |node, path| {
                    let Node::Instruction(instruction) = node else {
                        return node;
                    };
                    let current_program = path
                        .program()
                        .and_then(|d| d.name.as_ref())
                        .map(|n| n.as_ref());
                    Node::Instruction(split_instruction(
                        instruction,
                        &arg_name,
                        &linkables,
                        current_program,
                    ))
                },
            )
        })
        .collect();
    bottom_up_transformer(rules).visit_root(root)
}

fn split_instruction(
    instruction: InstructionNode,
    arg_name: &CamelCaseString,
    linkables: &LinkableDictionary,
    current_program: Option<&str>,
) -> InstructionNode {
    let Some(index) = instruction
        .arguments
        .iter()
        .position(|a| &a.name == arg_name)
    else {
        return instruction;
    };
    let arg_field = instruction.arguments[index].clone();
    let Some(enum_type) = resolve_enum(&arg_field, linkables, current_program) else {
        return instruction;
    };

    let mut sub_instructions = Vec::with_capacity(enum_type.variants.len());
    for (variant_index, variant) in enum_type.variants.iter().enumerate() {
        let sub_name = CamelCaseString::new(format!(
            "{} {}",
            instruction.name.as_ref(),
            variant_name(variant)
        ));

        let mut sub_fields: Vec<InstructionArgumentNode> = instruction.arguments[..index].to_vec();
        sub_fields.push(discriminator_argument(sub_name.as_ref(), variant_index));
        match variant {
            EnumVariantTypeNode::Struct(v) => {
                let mut arg = arg_field.clone();
                arg.r#type = Box::new(v.r#struct.clone().into());
                sub_fields.push(arg);
            }
            EnumVariantTypeNode::Tuple(v) => {
                let mut arg = arg_field.clone();
                arg.r#type = Box::new(v.tuple.clone().into());
                sub_fields.push(arg);
            }
            EnumVariantTypeNode::Empty(_) => {}
        }
        sub_fields.extend_from_slice(&instruction.arguments[index + 1..]);

        // Mirrors upstream's `{ ...node, name, arguments }`: every other field
        // (accounts, etc.) is inherited from the parent instruction.
        let mut sub = instruction.clone();
        sub.name = sub_name;
        sub.arguments = flatten_instruction_arguments(sub_fields);
        sub_instructions.push(sub);
    }

    let mut result = instruction;
    result.sub_instructions.extend(sub_instructions);
    result
}

fn resolve_enum(
    arg: &InstructionArgumentNode,
    linkables: &LinkableDictionary,
    current_program: Option<&str>,
) -> Option<EnumTypeNode> {
    match &*arg.r#type {
        TypeNode::Enum(enum_type) => Some(enum_type.clone()),
        TypeNode::Link(link) => match &*linkables.get_defined_type(link, current_program)?.r#type {
            TypeNode::Enum(enum_type) => Some(enum_type.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn variant_name(variant: &EnumVariantTypeNode) -> &str {
    match variant {
        EnumVariantTypeNode::Empty(v) => v.name.as_ref(),
        EnumVariantTypeNode::Struct(v) => v.name.as_ref(),
        EnumVariantTypeNode::Tuple(v) => v.name.as_ref(),
    }
}

fn discriminator_argument(sub_name: &str, index: usize) -> InstructionArgumentNode {
    InstructionArgumentNode {
        name: CamelCaseString::new(format!("{sub_name}Discriminator")),
        default_value_strategy: Some(DefaultValueStrategy::Omitted),
        docs: Docs::default(),
        r#type: Box::new(NumberTypeNode::le(U8).into()),
        default_value: Box::new(Some(InstructionInputValueNode::NumberValue(
            NumberValueNode::new(index as u32),
        ))),
    }
}
