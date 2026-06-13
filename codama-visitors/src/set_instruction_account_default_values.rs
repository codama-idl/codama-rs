use codama_nodes::{CamelCaseString, InstructionAccountNode, InstructionInputValueNode, Node};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// How a rule matches an instruction account by name.
pub enum AccountMatch {
    /// Exact (camelCase-normalised) name.
    Name(CamelCaseString),
    /// Arbitrary predicate over the account's name.
    Predicate(Box<dyn Fn(&str) -> bool>),
}

/// A rule assigning a default value to instruction accounts matching `account`
/// (optionally scoped to a single `instruction`).
pub struct InstructionAccountDefaultRule {
    account: AccountMatch,
    default_value: InstructionInputValueNode,
    ignore_if_optional: bool,
    instruction: Option<CamelCaseString>,
}

impl InstructionAccountDefaultRule {
    /// Match accounts by exact name.
    pub fn new(
        account: impl Into<CamelCaseString>,
        default_value: impl Into<InstructionInputValueNode>,
    ) -> Self {
        Self {
            account: AccountMatch::Name(account.into()),
            default_value: default_value.into(),
            ignore_if_optional: false,
            instruction: None,
        }
    }

    /// Match accounts by a predicate over their name (the Rust stand-in for the
    /// upstream `RegExp` matcher).
    pub fn matching(
        predicate: impl Fn(&str) -> bool + 'static,
        default_value: impl Into<InstructionInputValueNode>,
    ) -> Self {
        Self {
            account: AccountMatch::Predicate(Box::new(predicate)),
            default_value: default_value.into(),
            ignore_if_optional: false,
            instruction: None,
        }
    }

    /// Skip accounts that are optional or already have a default value.
    pub fn ignore_if_optional(mut self) -> Self {
        self.ignore_if_optional = true;
        self
    }

    /// Restrict this rule to a single instruction (by name).
    pub fn instruction(mut self, instruction: impl Into<CamelCaseString>) -> Self {
        self.instruction = Some(instruction.into());
        self
    }
}

/// Assigns default values to instruction accounts that match the given rules
/// (e.g. a `payerValueNode` for every `payer` account).
///
/// The Rust counterpart of `@codama/visitors`' `setInstructionAccountDefaultValuesVisitor`,
/// built on [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
/// Instruction-scoped rules take precedence over global ones.
///
/// ```
/// use codama_nodes::{InstructionAccountNode, InstructionNode, IsSigner, PayerValueNode, ProgramNode, RootNode};
/// use codama_visitors::{set_instruction_account_default_values, InstructionAccountDefaultRule, TransformVisitor};
///
/// let mut transfer = InstructionNode { name: "transfer".into(), ..Default::default() };
/// transfer.accounts.push(InstructionAccountNode::new("payer", true, IsSigner::True));
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(transfer);
/// let root = RootNode::new(program);
///
/// let mut visitor = set_instruction_account_default_values(vec![
///     InstructionAccountDefaultRule::new("payer", PayerValueNode::new()).ignore_if_optional(),
/// ]);
/// let _root = visitor.visit_root(root);
/// ```
///
/// Note: unlike upstream, the default value is assigned verbatim — PDA seed
/// values are not yet auto-filled from the instruction context (that needs the
/// `LinkableDictionary`, a follow-up). Common-account presets
/// (`getCommonInstructionAccountDefaultRules`) are also not yet provided.
pub fn set_instruction_account_default_values(
    mut rules: Vec<InstructionAccountDefaultRule>,
) -> BottomUpTransformer {
    // Instruction-scoped rules first, so they win over global ones.
    rules.sort_by_key(|rule| rule.instruction.is_none());
    bottom_up_transformer(vec![TransformRule::new(
        "[instructionNode]",
        move |node, _path| set_defaults(node, &rules),
    )])
}

fn set_defaults(node: Node, rules: &[InstructionAccountDefaultRule]) -> Node {
    let Node::Instruction(mut instruction) = node else {
        return node;
    };
    let instruction_name = instruction.name.clone();
    for account in instruction.accounts.iter_mut() {
        let Some(rule) = match_rule(rules, &instruction_name, account) else {
            continue;
        };
        if rule.ignore_if_optional
            && (account.is_optional == Some(true) || account.default_value.is_some())
        {
            continue;
        }
        account.default_value = Box::new(Some(rule.default_value.clone()));
    }
    Node::Instruction(instruction)
}

fn match_rule<'a>(
    rules: &'a [InstructionAccountDefaultRule],
    instruction: &CamelCaseString,
    account: &InstructionAccountNode,
) -> Option<&'a InstructionAccountDefaultRule> {
    rules.iter().find(|rule| {
        if let Some(rule_instruction) = &rule.instruction {
            if rule_instruction != instruction {
                return false;
            }
        }
        match &rule.account {
            AccountMatch::Name(name) => name == &account.name,
            AccountMatch::Predicate(predicate) => predicate(account.name.as_ref()),
        }
    })
}
