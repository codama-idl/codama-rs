use crate::fill_default_pda_seed_values;
use crate::rename_helpers::scoped_selector;
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, InstructionAccountNode, InstructionArgumentNode,
    InstructionInputValueNode, InstructionNode, IsSigner, Node, RootNode, TypeNode,
};
use codama_visitors_core::{
    bottom_up_transformer, LinkableDictionary, TransformRule, TransformVisitor,
};
use std::collections::HashSet;
use std::rc::Rc;

/// Field updates for one instruction account (matched by its current name).
#[derive(Debug, Clone, Default)]
pub struct InstructionAccountUpdate {
    name: Option<String>,
    is_signer: Option<IsSigner>,
    is_writable: Option<bool>,
    is_optional: Option<bool>,
    docs: Option<Docs>,
    default_value: Option<InstructionInputValueNode>,
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
    /// Set the account's default value (its PDA seeds are auto-filled from the
    /// instruction context).
    pub fn default_value(mut self, default_value: impl Into<InstructionInputValueNode>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

/// Field updates for one instruction argument (matched by its current name). An
/// update whose name matches no existing argument becomes a new `extraArgument`
/// (a `type_node` is required for that).
#[derive(Debug, Clone, Default)]
pub struct InstructionArgumentUpdate {
    name: Option<String>,
    type_node: Option<TypeNode>,
    docs: Option<Docs>,
    default_value: Option<InstructionInputValueNode>,
    default_value_strategy: Option<DefaultValueStrategy>,
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
    pub fn default_value(mut self, default_value: impl Into<InstructionInputValueNode>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
    pub fn default_value_strategy(mut self, strategy: DefaultValueStrategy) -> Self {
        self.default_value_strategy = Some(strategy);
        self
    }
}

/// A partial update for an instruction: metadata, per-account and per-argument
/// field updates, plus deletion.
#[derive(Debug, Clone, Default)]
pub struct InstructionUpdate {
    name: Option<String>,
    docs: Option<Docs>,
    accounts: Vec<(String, InstructionAccountUpdate)>,
    arguments: Vec<(String, InstructionArgumentUpdate)>,
    delete: bool,
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
    /// Mark the instruction for deletion.
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }
}

/// Updates instructions selected by name: instruction metadata, field updates to
/// named accounts (whose default values have their PDA seeds auto-filled) and
/// arguments, injection of brand-new `extraArguments`, and deletion.
///
/// The Rust counterpart of `@codama/visitors`' `updateInstructionsVisitor`.
/// Root-coupled: it builds a [`LinkableDictionary`] from `root` so account
/// default-value PDA seeds resolve against the instruction context.
///
/// ```
/// use codama_nodes::{InstructionAccountNode, InstructionNode, IsSigner, ProgramNode, RootNode};
/// use codama_visitors::{update_instructions, InstructionAccountUpdate, InstructionUpdate};
///
/// let mut ix = InstructionNode { name: "transfer".into(), ..Default::default() };
/// ix.accounts.push(InstructionAccountNode::new("payer", true, IsSigner::True));
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(ix);
/// let root = RootNode::new(program);
///
/// let root = update_instructions(root, [(
///     "transfer",
///     InstructionUpdate::new().name("send").account("payer", InstructionAccountUpdate::new().name("authority")),
/// )]);
/// assert_eq!(root.program.instructions[0].name.as_ref(), "send");
/// assert_eq!(root.program.instructions[0].accounts[0].name.as_ref(), "authority");
/// ```
pub fn update_instructions<S: Into<String>>(
    root: RootNode,
    map: impl IntoIterator<Item = (S, InstructionUpdate)>,
) -> RootNode {
    let linkables = Rc::new(LinkableDictionary::from_root(&root));
    let mut rules = Vec::new();
    let mut deletes: Vec<String> = Vec::new();

    for (name, update) in map {
        let name = name.into();
        if update.delete {
            deletes.push(scoped_selector("instructionNode", &name));
            continue;
        }
        let linkables = Rc::clone(&linkables);
        rules.push(TransformRule::new(
            scoped_selector("instructionNode", &name),
            move |node, path| {
                let Node::Instruction(mut instruction) = node else {
                    return node;
                };
                if let Some(name) = &update.name {
                    instruction.name = name.clone().into();
                }
                if let Some(docs) = &update.docs {
                    instruction.docs = docs.clone();
                }

                let current_program = path
                    .program()
                    .and_then(|d| d.name.as_ref())
                    .map(|n| n.to_string());
                let context = instruction.clone();
                for account in instruction.accounts.iter_mut() {
                    if let Some((_, account_update)) = update
                        .accounts
                        .iter()
                        .find(|(n, _)| CamelCaseString::new(n) == account.name)
                    {
                        apply_account_update(
                            account,
                            account_update,
                            &context,
                            &linkables,
                            current_program.as_deref(),
                        );
                    }
                }

                handle_arguments(&mut instruction, &update.arguments);
                Node::Instruction(instruction)
            },
        ));
    }

    let mut visitor = bottom_up_transformer(rules).also_delete(deletes);
    visitor.visit_root(root)
}

fn apply_account_update(
    account: &mut InstructionAccountNode,
    update: &InstructionAccountUpdate,
    context: &InstructionNode,
    linkables: &LinkableDictionary,
    current_program: Option<&str>,
) {
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
    // The effective default is the update's (if any) else the account's existing
    // one; when present, its PDA seeds are filled from the instruction context.
    let effective = update
        .default_value
        .clone()
        .or_else(|| (*account.default_value).clone());
    account.default_value = Box::new(effective.and_then(|value| {
        fill_default_pda_seed_values(value, context, linkables, current_program, false).ok()
    }));
}

fn handle_arguments(
    instruction: &mut InstructionNode,
    updates: &[(String, InstructionArgumentUpdate)],
) {
    let find = |name: &CamelCaseString| {
        updates
            .iter()
            .find(|(n, _)| &CamelCaseString::new(n) == name)
            .map(|(_, update)| update)
    };
    let mut used: HashSet<String> = HashSet::new();

    for argument in instruction.arguments.iter_mut() {
        if let Some(update) = find(&argument.name) {
            used.insert(argument.name.to_string());
            apply_argument_update(argument, update);
        }
    }
    for argument in instruction.extra_arguments.iter_mut() {
        if used.contains(argument.name.as_ref()) {
            continue;
        }
        if let Some(update) = find(&argument.name) {
            used.insert(argument.name.to_string());
            apply_argument_update(argument, update);
        }
    }

    // Inject new extra arguments for updates that matched no existing argument
    // (a type is required; without one the update is skipped, where upstream throws).
    for (name, update) in updates {
        let camel = CamelCaseString::new(name);
        if used.contains(camel.as_ref()) {
            continue;
        }
        let Some(type_node) = &update.type_node else {
            continue;
        };
        let arg_name = update.name.clone().unwrap_or_else(|| name.clone());
        let mut argument = InstructionArgumentNode::new(arg_name, type_node.clone());
        if let Some(docs) = &update.docs {
            argument.docs = docs.clone();
        }
        argument.default_value = Box::new(update.default_value.clone());
        argument.default_value_strategy = update.default_value_strategy;
        instruction.extra_arguments.push(argument);
    }
}

fn apply_argument_update(
    argument: &mut InstructionArgumentNode,
    update: &InstructionArgumentUpdate,
) {
    if let Some(name) = &update.name {
        argument.name = name.clone().into();
    }
    if let Some(type_node) = &update.type_node {
        argument.r#type = Box::new(type_node.clone());
    }
    if let Some(docs) = &update.docs {
        argument.docs = docs.clone();
    }
    if let Some(default_value) = &update.default_value {
        argument.default_value = Box::new(Some(default_value.clone()));
    }
    if let Some(strategy) = update.default_value_strategy {
        argument.default_value_strategy = Some(strategy);
    }
}
