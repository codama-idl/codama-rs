use codama_nodes::{EnumVariantTypeNode, InstructionNode, ProgramNode, RootNode, TypeNode};
use std::collections::HashMap;

/// How often a defined type is referenced (via `definedTypeLinkNode`), broken
/// down by the context the references appear in. Keyed by `"{program}.{type}"`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DefinedTypeHistogramEntry {
    /// Used directly as an instruction argument's type (not nested).
    pub directly_as_instruction_args: usize,
    pub in_accounts: usize,
    pub in_defined_types: usize,
    pub in_events: usize,
    pub in_instruction_args: usize,
    pub total: usize,
}

/// Builds a usage histogram of every defined type referenced through a
/// `definedTypeLinkNode`.
///
/// The Rust counterpart of `@codama/visitors`' `getDefinedTypeHistogramVisitor`.
/// Unlike that traversal-based visitor, this walks the (fixed-shape) `RootNode`
/// directly. Keys are program-qualified (`"myProgram.myType"`).
///
/// ```
/// use codama_nodes::{AccountNode, DefinedTypeLinkNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode};
/// use codama_visitors::get_defined_type_histogram;
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new(
///     "ref",
///     DefinedTypeLinkNode { name: "foo".into(), program: None },
/// )]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("acc", data));
/// let histogram = get_defined_type_histogram(&RootNode::new(program));
/// assert_eq!(histogram["p.foo"].in_accounts, 1);
/// ```
pub fn get_defined_type_histogram(root: &RootNode) -> HashMap<String, DefinedTypeHistogramEntry> {
    let mut out = HashMap::new();
    for program in std::iter::once(&root.program).chain(root.additional_programs.iter()) {
        record_program(program, &mut out);
    }
    out
}

#[derive(Clone, Copy)]
enum Mode {
    Account,
    DefinedType,
    Event,
    Instruction,
}

fn record_program(program: &ProgramNode, out: &mut HashMap<String, DefinedTypeHistogramEntry>) {
    let name = program.name.as_ref();
    for account in &program.accounts {
        let data: TypeNode = account.data.clone().into();
        count_links(&data, name, Mode::Account, 0, out);
    }
    for defined_type in &program.defined_types {
        count_links(&defined_type.r#type, name, Mode::DefinedType, 0, out);
    }
    for event in &program.events {
        count_links(&event.data, name, Mode::Event, 0, out);
    }
    for instruction in &program.instructions {
        record_instruction(instruction, name, out);
    }
}

fn record_instruction(
    instruction: &InstructionNode,
    program: &str,
    out: &mut HashMap<String, DefinedTypeHistogramEntry>,
) {
    for argument in instruction
        .arguments
        .iter()
        .chain(instruction.extra_arguments.iter())
    {
        count_links(&argument.r#type, program, Mode::Instruction, 0, out);
    }
    for sub in &instruction.sub_instructions {
        record_instruction(sub, program, out);
    }
}

fn count_links(
    node: &TypeNode,
    program: &str,
    mode: Mode,
    depth: usize,
    out: &mut HashMap<String, DefinedTypeHistogramEntry>,
) {
    match node {
        TypeNode::Link(link) => {
            let entry = out
                .entry(format!("{program}.{}", link.name.as_ref()))
                .or_default();
            entry.total += 1;
            match mode {
                Mode::Account => entry.in_accounts += 1,
                Mode::DefinedType => entry.in_defined_types += 1,
                Mode::Event => entry.in_events += 1,
                Mode::Instruction => {
                    entry.in_instruction_args += 1;
                    if depth == 0 {
                        entry.directly_as_instruction_args += 1;
                    }
                }
            }
        }
        TypeNode::Array(n) => count_links(&n.item, program, mode, depth + 1, out),
        TypeNode::Set(n) => count_links(&n.item, program, mode, depth + 1, out),
        TypeNode::Map(n) => {
            count_links(&n.key, program, mode, depth + 1, out);
            count_links(&n.value, program, mode, depth + 1, out);
        }
        TypeNode::Option(n) => count_links(&n.item, program, mode, depth + 1, out),
        TypeNode::RemainderOption(n) => count_links(&n.item, program, mode, depth + 1, out),
        TypeNode::ZeroableOption(n) => count_links(&n.item, program, mode, depth + 1, out),
        TypeNode::Struct(n) => {
            for field in &n.fields {
                count_links(&field.r#type, program, mode, depth + 1, out);
            }
        }
        TypeNode::Tuple(n) => {
            for item in &n.items {
                count_links(item, program, mode, depth + 1, out);
            }
        }
        TypeNode::Enum(n) => {
            for variant in &n.variants {
                count_variant(variant, program, mode, depth + 1, out);
            }
        }
        TypeNode::FixedSize(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::HiddenPrefix(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::HiddenSuffix(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::PreOffset(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::PostOffset(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::SizePrefix(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        TypeNode::Sentinel(n) => count_links(&n.r#type, program, mode, depth + 1, out),
        // Leaf / number-only types carry no defined-type links.
        _ => {}
    }
}

fn count_variant(
    variant: &EnumVariantTypeNode,
    program: &str,
    mode: Mode,
    depth: usize,
    out: &mut HashMap<String, DefinedTypeHistogramEntry>,
) {
    match variant {
        EnumVariantTypeNode::Empty(_) => {}
        EnumVariantTypeNode::Struct(v) => {
            let struct_type: TypeNode = v.r#struct.clone().into();
            count_links(&struct_type, program, mode, depth, out);
        }
        EnumVariantTypeNode::Tuple(v) => {
            let tuple_type: TypeNode = v.tuple.clone().into();
            count_links(&tuple_type, program, mode, depth, out);
        }
    }
}
