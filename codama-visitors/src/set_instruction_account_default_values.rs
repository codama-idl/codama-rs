use crate::fill_default_pda_seed_values;
use codama_nodes::{
    CamelCaseString, IdentityValueNode, InstructionAccountNode, InstructionInputValueNode, Node,
    PayerValueNode, ProgramIdValueNode, PublicKeyValueNode, RootNode,
};
use codama_visitors_core::{
    bottom_up_transformer, LinkableDictionary, TransformRule, TransformVisitor,
};
use std::rc::Rc;

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
/// The Rust counterpart of `@codama/visitors`'
/// `setInstructionAccountDefaultValuesVisitor`. Root-coupled: it builds a
/// [`LinkableDictionary`] from `root` so that any PDA seeds in a rule's default
/// value are auto-filled from the instruction context via
/// [`fill_default_pda_seed_values`](crate::fill_default_pda_seed_values) (strict
/// mode — if the seeds can't be resolved, the account is left unchanged, like
/// the upstream `try/catch`). Instruction-scoped rules take precedence over
/// global ones.
///
/// ```
/// use codama_nodes::{InstructionAccountNode, InstructionNode, IsSigner, PayerValueNode, ProgramNode, RootNode};
/// use codama_visitors::{set_instruction_account_default_values, InstructionAccountDefaultRule};
///
/// let mut transfer = InstructionNode { name: "transfer".into(), ..Default::default() };
/// transfer.accounts.push(InstructionAccountNode::new("payer", true, IsSigner::True));
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.instructions.push(transfer);
/// let root = RootNode::new(program);
///
/// let root = set_instruction_account_default_values(root, vec![
///     InstructionAccountDefaultRule::new("payer", PayerValueNode::new()).ignore_if_optional(),
/// ]);
/// assert!(root.program.instructions[0].accounts[0].default_value.is_some());
/// ```
pub fn set_instruction_account_default_values(
    root: RootNode,
    mut rules: Vec<InstructionAccountDefaultRule>,
) -> RootNode {
    // Instruction-scoped rules first, so they win over global ones.
    rules.sort_by_key(|rule| rule.instruction.is_none());
    let linkables = Rc::new(LinkableDictionary::from_root(&root));
    let rules = Rc::new(rules);

    let mut visitor = bottom_up_transformer(vec![TransformRule::new("[instructionNode]", {
        let linkables = Rc::clone(&linkables);
        let rules = Rc::clone(&rules);
        move |node, path| {
            let Node::Instruction(mut instruction) = node else {
                return node;
            };
            let current_program = path
                .program()
                .and_then(|d| d.name.as_ref())
                .map(|n| n.as_ref());
            // Snapshot for matching + PDA-seed resolution while we mutate accounts.
            let context = instruction.clone();
            for account in instruction.accounts.iter_mut() {
                let Some(rule) = match_rule(&rules, &context.name, account) else {
                    continue;
                };
                if rule.ignore_if_optional
                    && (account.is_optional == Some(true) || account.default_value.is_some())
                {
                    continue;
                }
                // Strict PDA-seed fill: on failure leave the account untouched.
                if let Ok(value) = fill_default_pda_seed_values(
                    rule.default_value.clone(),
                    &context,
                    &linkables,
                    current_program,
                    true,
                ) {
                    account.default_value = Box::new(Some(value));
                }
            }
            Node::Instruction(instruction)
        }
    })]);
    visitor.visit_root(root)
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

/// The default rules for common Solana accounts (payer, system program, the
/// SPL/Metaplex program ids, the sysvars, ...), all flagged `ignore_if_optional`.
///
/// The Rust counterpart of `getCommonInstructionAccountDefaultRules`. Upstream
/// `RegExp` alternations are expanded into explicit name predicates.
pub fn get_common_instruction_account_default_rules() -> Vec<InstructionAccountDefaultRule> {
    fn pubkey(address: &str, identifier: Option<&str>) -> PublicKeyValueNode {
        PublicKeyValueNode {
            public_key: address.to_string(),
            identifier: identifier.map(CamelCaseString::new),
        }
    }
    fn rule(
        predicate: fn(&str) -> bool,
        value: impl Into<InstructionInputValueNode>,
    ) -> InstructionAccountDefaultRule {
        InstructionAccountDefaultRule::matching(predicate, value).ignore_if_optional()
    }

    vec![
        rule(|n| matches!(n, "payer" | "feePayer"), PayerValueNode::new()),
        rule(|n| n == "authority", IdentityValueNode::new()),
        rule(|n| n == "programId", ProgramIdValueNode::new()),
        rule(
            |n| matches!(n, "systemProgram" | "splSystemProgram"),
            pubkey("11111111111111111111111111111111", Some("splSystem")),
        ),
        rule(
            |n| matches!(n, "tokenProgram" | "splTokenProgram"),
            pubkey(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                Some("splToken"),
            ),
        ),
        rule(
            |n| matches!(n, "ataProgram" | "splAtaProgram"),
            pubkey(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
                Some("splAssociatedToken"),
            ),
        ),
        rule(
            |n| matches!(n, "tokenMetadataProgram" | "mplTokenMetadataProgram"),
            pubkey(
                "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
                Some("mplTokenMetadata"),
            ),
        ),
        rule(
            |n| {
                matches!(
                    n,
                    "tokenAuthRulesProgram"
                        | "mplTokenAuthRulesProgram"
                        | "authorizationRulesProgram"
                        | "mplAuthorizationRulesProgram"
                        | "authRulesProgram"
                        | "mplAuthRulesProgram"
                )
            },
            pubkey(
                "auth9SigNpDKz4sJJ1DfCTuZrZNSAgh9sFD3rboVmgg",
                Some("mplTokenAuthRules"),
            ),
        ),
        rule(
            |n| matches!(n, "candyMachineProgram" | "mplCandyMachineProgram"),
            pubkey(
                "CndyV3LdqHUfDLmE5naZjVN8rBZz4tqhdefbAnjHG3JR",
                Some("mplCandyMachine"),
            ),
        ),
        rule(
            |n| matches!(n, "candyGuardProgram" | "mplCandyGuardProgram"),
            pubkey(
                "Guard1JwRhJkVH6XZhzoYxeBVQe872VH6QggF4BWmS9g",
                Some("mplCandyGuard"),
            ),
        ),
        rule(
            |n| matches!(n, "clockSysvar" | "sysvarClock"),
            pubkey("SysvarC1ock11111111111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "epochScheduleSysvar" | "sysvarEpochSchedule"),
            pubkey("SysvarEpochSchedu1e111111111111111111111111", None),
        ),
        rule(
            |n| {
                matches!(
                    n,
                    "instructionSysvar"
                        | "instructionsSysvar"
                        | "sysvarInstruction"
                        | "sysvarInstructions"
                        | "instructionSysvarAccount"
                        | "instructionsSysvarAccount"
                        | "sysvarInstructionAccount"
                        | "sysvarInstructionsAccount"
                )
            },
            pubkey("Sysvar1nstructions1111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "recentBlockhashesSysvar" | "sysvarRecentBlockhashes"),
            pubkey("SysvarRecentB1ockHashes11111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "rent" | "rentSysvar" | "sysvarRent"),
            pubkey("SysvarRent111111111111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "rewardsSysvar" | "sysvarRewards"),
            pubkey("SysvarRewards111111111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "slotHashesSysvar" | "sysvarSlotHashes"),
            pubkey("SysvarS1otHashes111111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "slotHistorySysvar" | "sysvarSlotHistory"),
            pubkey("SysvarS1otHistory11111111111111111111111111", None),
        ),
        rule(
            |n| matches!(n, "stakeHistorySysvar" | "sysvarStakeHistory"),
            pubkey("SysvarStakeHistory1111111111111111111111111", None),
        ),
        rule(
            |n| n == "mplCoreProgram",
            pubkey(
                "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
                Some("mplCore"),
            ),
        ),
    ]
}
