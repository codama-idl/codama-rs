use codama_nodes::{CamelCaseString, Docs, IsSigner, Node, TypeNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// Field updates for one instruction account (matched by its current name).
#[derive(Debug, Clone, Default)]
pub struct InstructionAccountUpdate {
    name: Option<String>,
    is_signer: Option<IsSigner>,
    is_writable: Option<bool>,
    is_optional: Option<bool>,
    docs: Option<Docs>,
}

impl InstructionAccountUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn signer(mut self, is_signer: IsSigner) -> Self {
        self.is_signer = Some(is_signer);
        self
    }
    pub fn writable(mut self, is_writable: bool) -> Self {
        self.is_writable = Some(is_writable);
        self
    }
    pub fn optional(mut self, is_optional: bool) -> Self {
        self.is_optional = Some(is_optional);
        self
    }
    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
}

/// Field updates for one instruction argument (matched by its current name).
#[derive(Debug, Clone, Default)]
pub struct InstructionArgumentUpdate {
    name: Option<String>,
    type_node: Option<TypeNode>,
    docs: Option<Docs>,
}

impl InstructionArgumentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn type_node(mut self, type_node: impl Into<TypeNode>) -> Self {
        self.type_node = Some(type_node.into());
        self
    }
    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
}

/// A partial update for an instruction: its own metadata plus per-account and
/// per-argument field updates.
#[derive(Debug, Clone, Default)]
pub struct InstructionUpdate {
    name: Option<String>,
    docs: Option<Docs>,
    accounts: Vec<(String, InstructionAccountUpdate)>,
    arguments: Vec<(String, InstructionArgumentUpdate)>,
}

impl InstructionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn docs(mut self, docs: impl Into<Docs>) -> Self {
        self.docs = Some(docs.into());
        self
    }
    pub fn account(mut self, name: impl Into<String>, update: InstructionAccountUpdate) -> Self {
        self.accounts.push((name.into(), update));
        self
    }
    pub fn argument(mut self, name: impl Into<String>, update: InstructionArgumentUpdate) -> Self {
        self.arguments.push((name.into(), update));
        self
    }
}

/// Updates instructions selected by name: instruction metadata, plus field
/// updates to named accounts and arguments (including `extraArguments`).
///
/// The Rust counterpart of `@codama/visitors`' `updateInstructionsVisitor`, built
/// on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{InstructionAccountNode, InstructionNode, IsSigner, ProgramNode, RootNode};
/// use codama_visitors::{update_instructions, InstructionAccountUpdate, InstructionUpdate, TransformVisitor};
///
/// let mut ix = InstructionNode { name: "transfer".into(), ..Default::default() };
/// ix.accounts.push(InstructionAccountNode::new("payer", true, IsSigner::True));
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(ix);
/// let root = RootNode::new(program);
///
/// let root = update_instructions([(
///     "transfer",
///     InstructionUpdate::new().name("send").account("payer", InstructionAccountUpdate::new().name("authority")),
/// )]).visit_root(root);
/// assert_eq!(root.program.instructions[0].name.as_ref(), "send");
/// assert_eq!(root.program.instructions[0].accounts[0].name.as_ref(), "authority");
/// ```
///
/// Note: filling account default values from PDA seeds and injecting brand-new
/// extra arguments (both upstream features) are follow-ups.
pub fn update_instructions<S: Into<String>>(
    map: impl IntoIterator<Item = (S, InstructionUpdate)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(name, update)| {
            TransformRule::new(
                format!("[instructionNode]{}", name.into()),
                move |node, _path| {
                    let Node::Instruction(mut instruction) = node else {
                        return node;
                    };
                    if let Some(name) = &update.name {
                        instruction.name = name.clone().into();
                    }
                    if let Some(docs) = &update.docs {
                        instruction.docs = docs.clone();
                    }
                    for account in instruction.accounts.iter_mut() {
                        apply_account_update(account, &update.accounts);
                    }
                    for argument in instruction
                        .arguments
                        .iter_mut()
                        .chain(instruction.extra_arguments.iter_mut())
                    {
                        apply_argument_update(argument, &update.arguments);
                    }
                    Node::Instruction(instruction)
                },
            )
        })
        .collect();
    bottom_up_transformer(rules)
}

fn apply_account_update(
    account: &mut codama_nodes::InstructionAccountNode,
    updates: &[(String, InstructionAccountUpdate)],
) {
    let Some((_, update)) = updates
        .iter()
        .find(|(name, _)| CamelCaseString::new(name) == account.name)
    else {
        return;
    };
    if let Some(name) = &update.name {
        account.name = name.clone().into();
    }
    if let Some(is_signer) = update.is_signer {
        account.is_signer = is_signer;
    }
    if let Some(is_writable) = update.is_writable {
        account.is_writable = is_writable;
    }
    if let Some(is_optional) = update.is_optional {
        account.is_optional = Some(is_optional);
    }
    if let Some(docs) = &update.docs {
        account.docs = docs.clone();
    }
}

fn apply_argument_update(
    argument: &mut codama_nodes::InstructionArgumentNode,
    updates: &[(String, InstructionArgumentUpdate)],
) {
    let Some((_, update)) = updates
        .iter()
        .find(|(name, _)| CamelCaseString::new(name) == argument.name)
    else {
        return;
    };
    if let Some(name) = &update.name {
        argument.name = name.clone().into();
    }
    if let Some(type_node) = &update.type_node {
        argument.r#type = Box::new(type_node.clone());
    }
    if let Some(docs) = &update.docs {
        argument.docs = docs.clone();
    }
}
