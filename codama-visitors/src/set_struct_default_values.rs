use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, InstructionArgumentNode, InstructionInputValueNode,
    Node, RegisteredTypeNode, StructFieldTypeNode, ValueNode,
};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// A default-value directive for a named struct field or instruction argument.
///
/// Mirrors the upstream `ValueNode | { strategy?, value } | null` union:
/// [`Value`](StructDefaultValue::Value) sets the default with no strategy,
/// [`WithStrategy`](StructDefaultValue::WithStrategy) sets it with an explicit
/// strategy, and [`Clear`](StructDefaultValue::Clear) removes any existing
/// default. A bare [`ValueNode`] converts into [`Value`](StructDefaultValue::Value).
#[derive(Debug, Clone)]
pub enum StructDefaultValue {
    Value(ValueNode),
    WithStrategy(ValueNode, Option<DefaultValueStrategy>),
    Clear,
}

impl From<ValueNode> for StructDefaultValue {
    fn from(value: ValueNode) -> Self {
        Self::Value(value)
    }
}

/// Sets default values on named fields of structs — and on the matching
/// instruction arguments — selected by a [`NodeSelector`] stack.
///
/// The Rust counterpart of `@codama/visitors`' `setStructDefaultValuesVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
/// Each entry maps a stack to `(fieldName, default)` pairs and produces two
/// rules: one for `structTypeNode`s under that stack, and one for the arguments
/// (including `extraArguments`) of an instruction matching the stack — so the
/// defaults follow fields whether or not they have been flattened into
/// instruction arguments.
///
/// [`NodeSelector`]: codama_visitors_core::NodeSelector
///
/// ```
/// use codama_nodes::{DefinedTypeNode, NumberTypeNode, NumberValueNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U8};
/// use codama_visitors::{set_struct_default_values, StructDefaultValue, TransformVisitor};
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("bump", NumberTypeNode::le(U8))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.defined_types.push(DefinedTypeNode::new("config", data));
/// let root = RootNode::new(program);
///
/// let mut visitor = set_struct_default_values([(
///     "[definedTypeNode]config",
///     vec![("bump".to_string(), StructDefaultValue::Value(NumberValueNode::new(0u8).into()))],
/// )]);
/// let _root = visitor.visit_root(root);
/// ```
pub fn set_struct_default_values<S: Into<String>>(
    map: impl IntoIterator<Item = (S, Vec<(String, StructDefaultValue)>)>,
) -> BottomUpTransformer {
    let mut rules = Vec::new();
    for (stack, defaults) in map {
        let stack = stack.into();
        let defaults: Vec<(CamelCaseString, StructDefaultValue)> = defaults
            .into_iter()
            .map(|(name, value)| (CamelCaseString::new(name), value))
            .collect();

        // 1. Struct fields under the stack.
        let struct_defaults = defaults.clone();
        rules.push(TransformRule::new(
            format!("{stack}.[structTypeNode]"),
            move |node, _path| {
                let Node::Type(RegisteredTypeNode::Struct(mut node)) = node else {
                    return node;
                };
                for field in node.fields.iter_mut() {
                    if let Some((_, default)) =
                        struct_defaults.iter().find(|(name, _)| name == &field.name)
                    {
                        apply_to_field(field, default);
                    }
                }
                Node::Type(RegisteredTypeNode::Struct(node))
            },
        ));

        // 2. Arguments of an instruction matching the stack. A bracketed,
        //    non-instruction selector can never match an instruction, so the
        //    argument rule is only emitted for name (or instruction) stacks.
        let Some(selector) = instruction_selector(&stack) else {
            continue;
        };
        rules.push(TransformRule::new(selector, move |node, _path| {
            let Node::Instruction(mut instruction) = node else {
                return node;
            };
            for argument in instruction
                .arguments
                .iter_mut()
                .chain(instruction.extra_arguments.iter_mut())
            {
                if let Some((_, default)) = defaults.iter().find(|(name, _)| name == &argument.name)
                {
                    apply_to_argument(argument, default);
                }
            }
            Node::Instruction(instruction)
        }));
    }
    bottom_up_transformer(rules)
}

fn instruction_selector(stack: &str) -> Option<String> {
    if stack.starts_with("[instructionNode]") {
        Some(stack.to_string())
    } else if stack.starts_with('[') {
        None
    } else {
        Some(format!("[instructionNode]{stack}"))
    }
}

fn resolve(default: &StructDefaultValue) -> (Option<ValueNode>, Option<DefaultValueStrategy>) {
    match default {
        StructDefaultValue::Value(value) => (Some(value.clone()), None),
        StructDefaultValue::WithStrategy(value, strategy) => (Some(value.clone()), *strategy),
        StructDefaultValue::Clear => (None, None),
    }
}

fn apply_to_field(field: &mut StructFieldTypeNode, default: &StructDefaultValue) {
    let (value, strategy) = resolve(default);
    field.default_value = Box::new(value);
    field.default_value_strategy = strategy;
}

fn apply_to_argument(argument: &mut InstructionArgumentNode, default: &StructDefaultValue) {
    let (value, strategy) = resolve(default);
    // Instruction-argument defaults use the broader `InstructionInputValueNode`.
    argument.default_value = Box::new(value.map(InstructionInputValueNode::from));
    argument.default_value_strategy = strategy;
}
