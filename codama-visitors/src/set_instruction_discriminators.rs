use codama_nodes::{
    DefaultValueStrategy, DiscriminatorNode, Docs, FieldDiscriminatorNode, InstructionArgumentNode,
    Node, NumberTypeNode, TypeNode, ValueNode, U8,
};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// Describes the discriminator to prepend to an instruction: a fixed-value
/// argument plus a matching field discriminator.
#[derive(Debug, Clone)]
pub struct Discriminator {
    value: ValueNode,
    name: Option<String>,
    type_node: Option<TypeNode>,
    strategy: Option<DefaultValueStrategy>,
    docs: Docs,
}

impl Discriminator {
    /// A discriminator with the given constant value. Defaults: name
    /// `"discriminator"`, type `numberTypeNode(u8)`, strategy `omitted`, no docs.
    pub fn new(value: impl Into<ValueNode>) -> Self {
        Self {
            value: value.into(),
            name: None,
            type_node: None,
            strategy: None,
            docs: Docs::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn type_node(mut self, type_node: impl Into<TypeNode>) -> Self {
        self.type_node = Some(type_node.into());
        self
    }

    pub fn strategy(mut self, strategy: DefaultValueStrategy) -> Self {
        self.strategy = Some(strategy);
        self
    }

    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = docs.into();
        self
    }
}

/// Prepends a discriminator to each selected instruction: a fixed-value
/// argument at the front of its data, plus a `fieldDiscriminatorNode` pointing
/// at that argument.
///
/// The Rust counterpart of `@codama/visitors`' `setInstructionDiscriminatorsVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
/// Each entry maps an instruction selector (typically its name) to a
/// [`Discriminator`].
///
/// ```
/// use codama_nodes::{InstructionNode, NumberValueNode, ProgramNode, RootNode};
/// use codama_visitors::{set_instruction_discriminators, Discriminator, TransformVisitor};
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(InstructionNode { name: "transfer".into(), ..Default::default() });
/// let root = RootNode::new(program);
///
/// let mut visitor =
///     set_instruction_discriminators([("transfer", Discriminator::new(NumberValueNode::new(1u8)))]);
/// let root = visitor.visit_root(root);
/// assert_eq!(root.program.instructions[0].arguments[0].name.as_ref(), "discriminator");
/// ```
pub fn set_instruction_discriminators<S: Into<String>>(
    map: impl IntoIterator<Item = (S, Discriminator)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(selector, discriminator)| {
            TransformRule::new(selector.into(), move |node, _path| {
                set_discriminator(node, discriminator.clone())
            })
        })
        .collect();
    bottom_up_transformer(rules)
}

fn set_discriminator(node: Node, discriminator: Discriminator) -> Node {
    // The conjunction with `[instructionNode]` is enforced here: a non-instruction
    // that happens to match the selector is left untouched (upstream throws).
    let Node::Instruction(mut instruction) = node else {
        return node;
    };

    let name = discriminator
        .name
        .unwrap_or_else(|| "discriminator".to_string());
    let type_node = discriminator
        .type_node
        .unwrap_or_else(|| NumberTypeNode::le(U8).into());
    let strategy = discriminator
        .strategy
        .unwrap_or(DefaultValueStrategy::Omitted);

    let argument = InstructionArgumentNode {
        name: name.clone().into(),
        default_value_strategy: Some(strategy),
        docs: discriminator.docs,
        r#type: Box::new(type_node),
        default_value: Box::new(Some(discriminator.value.into())),
    };

    instruction.arguments.insert(0, argument);
    instruction.discriminators.insert(
        0,
        DiscriminatorNode::Field(FieldDiscriminatorNode::new(name, 0)),
    );

    Node::Instruction(instruction)
}
