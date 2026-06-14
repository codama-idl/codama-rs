use codama_nodes::{
    ConditionalValueCondition, InstructionAccountNode, InstructionArgumentNode,
    InstructionInputValueNode, InstructionNode, IsSigner, PdaSeedValueValue, PdaValueProgramId,
    ResolverDependency,
};
use std::collections::HashMap;

/// A dependency of an instruction input: another account or argument referenced
/// by a default value. Mirrors the upstream `accountValueNode | argumentValueNode`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionDependency {
    Account(String),
    Argument(String),
}

/// A resolved instruction account: the original node plus the resolved signer /
/// optional flags, whether it is a PDA, and what it depends on.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedInstructionAccount {
    pub account: InstructionAccountNode,
    pub depends_on: Vec<InstructionDependency>,
    pub is_pda: bool,
    pub resolved_is_optional: bool,
    pub resolved_is_signer: IsSigner,
}

/// A resolved instruction argument: the original node plus what it depends on.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedInstructionArgument {
    pub argument: InstructionArgumentNode,
    pub depends_on: Vec<InstructionDependency>,
}

/// One resolved instruction input, in dependency order.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedInstructionInput {
    Account(ResolvedInstructionAccount),
    Argument(ResolvedInstructionArgument),
}

/// Error raised while resolving instruction inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedInstructionInputError {
    CyclicDependency {
        instruction: String,
        cycle: String,
    },
    InvalidDependency {
        instruction: String,
        dependency: String,
    },
    OptionalAccountAsPdaSeed {
        instruction: String,
        seed: String,
    },
}

impl std::fmt::Display for ResolvedInstructionInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CyclicDependency { instruction, cycle } => write!(
                f,
                "cyclic dependency while resolving instruction `{instruction}`: {cycle}"
            ),
            Self::InvalidDependency {
                instruction,
                dependency,
            } => write!(
                f,
                "invalid default-value dependency `{dependency}` in instruction `{instruction}`"
            ),
            Self::OptionalAccountAsPdaSeed { instruction, seed } => write!(
                f,
                "optional account used as PDA seed `{seed}` in instruction `{instruction}`"
            ),
        }
    }
}

impl std::error::Error for ResolvedInstructionInputError {}

type Result<T> = std::result::Result<T, ResolvedInstructionInputError>;

/// Resolves an instruction's accounts and (defaulted) arguments into a
/// dependency-ordered list, computing each account's resolved signer/optional
/// flags and whether it is a PDA.
///
/// The Rust counterpart of `@codama/visitors-core`'s
/// `getResolvedInstructionInputsVisitor`. By default, regular arguments whose
/// default value is a plain `valueNode` are excluded; set
/// `include_data_argument_value_nodes` to include them.
pub fn get_resolved_instruction_inputs(
    instruction: &InstructionNode,
    include_data_argument_value_nodes: bool,
) -> Result<Vec<ResolvedInstructionInput>> {
    let mut resolver = Resolver {
        instruction,
        stack: Vec::new(),
        resolved: Vec::new(),
        visited_accounts: HashMap::new(),
        visited_args: Vec::new(),
    };

    // accounts, then defaulted data arguments (filtered), then defaulted extras.
    let mut inputs: Vec<Input> = instruction
        .accounts
        .iter()
        .cloned()
        .map(Input::Account)
        .collect();
    for argument in &instruction.arguments {
        let Some(default) = argument.default_value.as_ref() else {
            continue;
        };
        if include_data_argument_value_nodes || !is_value_node(default) {
            inputs.push(Input::Argument(argument.clone()));
        }
    }
    for argument in &instruction.extra_arguments {
        if argument.default_value.is_some() {
            inputs.push(Input::Argument(argument.clone()));
        }
    }

    for input in inputs {
        resolver.resolve_input(input)?;
    }
    Ok(resolver.resolved)
}

#[derive(Clone)]
enum Input {
    Account(InstructionAccountNode),
    Argument(InstructionArgumentNode),
}

impl Input {
    fn is_account(&self) -> bool {
        matches!(self, Input::Account(_))
    }
    fn name(&self) -> &str {
        match self {
            Input::Account(a) => a.name.as_ref(),
            Input::Argument(a) => a.name.as_ref(),
        }
    }
}

struct Resolver<'a> {
    instruction: &'a InstructionNode,
    stack: Vec<(bool, String)>, // (is_account, name) for cycle detection
    resolved: Vec<ResolvedInstructionInput>,
    visited_accounts: HashMap<String, ResolvedInstructionAccount>,
    visited_args: Vec<String>,
}

impl Resolver<'_> {
    fn resolve_input(&mut self, input: Input) -> Result<()> {
        let name = input.name().to_string();
        let is_account = input.is_account();

        // Skip inputs we've already resolved.
        if (is_account && self.visited_accounts.contains_key(&name))
            || (!is_account && self.visited_args.contains(&name))
        {
            return Ok(());
        }

        // Detect circular dependencies.
        if self
            .stack
            .iter()
            .any(|(kind, n)| *kind == is_account && n == &name)
        {
            let mut cycle: Vec<String> = self.stack.iter().map(|(_, n)| n.clone()).collect();
            cycle.push(name);
            return Err(ResolvedInstructionInputError::CyclicDependency {
                instruction: self.instruction.name.to_string(),
                cycle: cycle.join(" -> "),
            });
        }

        self.stack.push((is_account, name.clone()));
        let resolved = match input {
            Input::Account(account) => {
                ResolvedInstructionInput::Account(self.resolve_account(account)?)
            }
            Input::Argument(argument) => {
                ResolvedInstructionInput::Argument(self.resolve_argument(argument)?)
            }
        };
        self.stack.pop();

        match &resolved {
            ResolvedInstructionInput::Account(account) => {
                self.visited_accounts.insert(name, account.clone());
            }
            ResolvedInstructionInput::Argument(_) => self.visited_args.push(name),
        }
        self.resolved.push(resolved);
        Ok(())
    }

    fn resolve_account(
        &mut self,
        account: InstructionAccountNode,
    ) -> Result<ResolvedInstructionAccount> {
        let depends_on = dependencies_of(account.default_value.as_ref().as_ref());
        self.resolve_dependencies(&depends_on)?;

        let is_pda = self.all_arguments().any(|argument| {
            matches!(
                argument.default_value.as_ref(),
                Some(InstructionInputValueNode::AccountBumpValue(bump)) if bump.name == account.name
            )
        });

        let mut resolved = ResolvedInstructionAccount {
            depends_on,
            is_pda,
            resolved_is_optional: account.is_optional == Some(true),
            resolved_is_signer: account.is_signer,
            account: account.clone(),
        };

        match account.default_value.as_ref() {
            Some(InstructionInputValueNode::AccountValue(value)) => {
                if let Some(default) = self.visited_accounts.get(value.name.as_ref()) {
                    let is_public_key = account.is_signer == IsSigner::False
                        && default.account.is_signer == IsSigner::False;
                    let is_signer = account.is_signer == IsSigner::True
                        && default.account.is_signer == IsSigner::True;
                    resolved.resolved_is_signer = if !is_public_key && !is_signer {
                        IsSigner::Either
                    } else if is_signer {
                        IsSigner::True
                    } else {
                        IsSigner::False
                    };
                    resolved.resolved_is_optional = default.account.is_optional == Some(true);
                }
            }
            Some(InstructionInputValueNode::PublicKeyValue(_))
            | Some(InstructionInputValueNode::ProgramLink(_))
            | Some(InstructionInputValueNode::ProgramIdValue(_)) => {
                resolved.resolved_is_signer = signer_or_either(account.is_signer);
                resolved.resolved_is_optional = false;
            }
            Some(InstructionInputValueNode::PdaValue(pda)) => {
                resolved.resolved_is_signer = signer_or_either(account.is_signer);
                resolved.resolved_is_optional = false;
                for seed in &pda.seeds {
                    if let PdaSeedValueValue::Account(seed_account) = &*seed.value {
                        if let Some(dependency) =
                            self.visited_accounts.get(seed_account.name.as_ref())
                        {
                            if dependency.resolved_is_optional {
                                return Err(
                                    ResolvedInstructionInputError::OptionalAccountAsPdaSeed {
                                        instruction: self.instruction.name.to_string(),
                                        seed: seed.name.to_string(),
                                    },
                                );
                            }
                        }
                    }
                }
            }
            Some(InstructionInputValueNode::IdentityValue(_))
            | Some(InstructionInputValueNode::PayerValue(_))
            | Some(InstructionInputValueNode::ResolverValue(_)) => {
                resolved.resolved_is_optional = false;
            }
            _ => {}
        }

        Ok(resolved)
    }

    fn resolve_argument(
        &mut self,
        argument: InstructionArgumentNode,
    ) -> Result<ResolvedInstructionArgument> {
        let depends_on = dependencies_of(argument.default_value.as_ref().as_ref());
        self.resolve_dependencies(&depends_on)?;
        Ok(ResolvedInstructionArgument {
            argument,
            depends_on,
        })
    }

    fn resolve_dependencies(&mut self, dependencies: &[InstructionDependency]) -> Result<()> {
        for dependency in dependencies {
            let input = match dependency {
                InstructionDependency::Account(name) => self
                    .instruction
                    .accounts
                    .iter()
                    .find(|a| a.name.as_ref() == name)
                    .cloned()
                    .map(Input::Account),
                InstructionDependency::Argument(name) => self
                    .all_arguments()
                    .find(|a| a.name.as_ref() == name)
                    .cloned()
                    .map(Input::Argument),
            };
            let Some(input) = input else {
                let name = match dependency {
                    InstructionDependency::Account(n) | InstructionDependency::Argument(n) => {
                        n.clone()
                    }
                };
                return Err(ResolvedInstructionInputError::InvalidDependency {
                    instruction: self.instruction.name.to_string(),
                    dependency: name,
                });
            };
            self.resolve_input(input)?;
        }
        Ok(())
    }

    fn all_arguments(&self) -> impl Iterator<Item = &InstructionArgumentNode> {
        self.instruction
            .arguments
            .iter()
            .chain(self.instruction.extra_arguments.iter())
    }
}

fn signer_or_either(is_signer: IsSigner) -> IsSigner {
    if is_signer == IsSigner::False {
        IsSigner::False
    } else {
        IsSigner::Either
    }
}

/// `true` if `value` is a plain value node (vs a contextual value node).
fn is_value_node(value: &InstructionInputValueNode) -> bool {
    use InstructionInputValueNode::*;
    matches!(
        value,
        ArrayValue(_)
            | BooleanValue(_)
            | BytesValue(_)
            | ConstantValue(_)
            | EnumValue(_)
            | MapValue(_)
            | NoneValue(_)
            | NumberValue(_)
            | PublicKeyValue(_)
            | SetValue(_)
            | SomeValue(_)
            | StringValue(_)
            | StructValue(_)
            | TupleValue(_)
    )
}

/// Extracts the account/argument dependencies referenced by a default value.
fn dependencies_of(
    default_value: Option<&InstructionInputValueNode>,
) -> Vec<InstructionDependency> {
    let Some(value) = default_value else {
        return Vec::new();
    };
    dependencies_of_value(value)
}

fn dependencies_of_value(value: &InstructionInputValueNode) -> Vec<InstructionDependency> {
    use InstructionInputValueNode::*;
    match value {
        AccountValue(v) => vec![InstructionDependency::Account(v.name.to_string())],
        AccountBumpValue(v) => vec![InstructionDependency::Account(v.name.to_string())],
        ArgumentValue(v) => vec![InstructionDependency::Argument(v.name.to_string())],
        PdaValue(v) => {
            let mut deps: Vec<InstructionDependency> = Vec::new();
            for seed in &v.seeds {
                match &*seed.value {
                    PdaSeedValueValue::Account(a) => {
                        deps.push(InstructionDependency::Account(a.name.to_string()))
                    }
                    PdaSeedValueValue::Argument(a) => {
                        deps.push(InstructionDependency::Argument(a.name.to_string()))
                    }
                    _ => {}
                }
            }
            let mut deduped = deduplicate(deps);
            if let Some(program_id) = v.program_id.as_ref() {
                deduped.push(match program_id {
                    PdaValueProgramId::Account(a) => {
                        InstructionDependency::Account(a.name.to_string())
                    }
                    PdaValueProgramId::Argument(a) => {
                        InstructionDependency::Argument(a.name.to_string())
                    }
                });
            }
            deduped
        }
        ResolverValue(v) => v
            .depends_on
            .iter()
            .map(|dependency| match dependency {
                ResolverDependency::Account(a) => {
                    InstructionDependency::Account(a.name.to_string())
                }
                ResolverDependency::Argument(a) => {
                    InstructionDependency::Argument(a.name.to_string())
                }
            })
            .collect(),
        ConditionalValue(v) => {
            let mut deps = dependencies_of_condition(&v.condition);
            deps.extend(dependencies_of(v.if_true.as_ref().as_ref()));
            deps.extend(dependencies_of(v.if_false.as_ref().as_ref()));
            deduplicate(deps)
        }
        _ => Vec::new(),
    }
}

fn dependencies_of_condition(condition: &ConditionalValueCondition) -> Vec<InstructionDependency> {
    match condition {
        ConditionalValueCondition::Account(a) => {
            vec![InstructionDependency::Account(a.name.to_string())]
        }
        ConditionalValueCondition::Argument(a) => {
            vec![InstructionDependency::Argument(a.name.to_string())]
        }
        ConditionalValueCondition::Resolver(r) => r
            .depends_on
            .iter()
            .map(|dependency| match dependency {
                ResolverDependency::Account(a) => {
                    InstructionDependency::Account(a.name.to_string())
                }
                ResolverDependency::Argument(a) => {
                    InstructionDependency::Argument(a.name.to_string())
                }
            })
            .collect(),
    }
}

/// Dedupes dependencies, returning accounts (in first-seen order) then
/// arguments (in first-seen order), mirroring the upstream behavior.
fn deduplicate(dependencies: Vec<InstructionDependency>) -> Vec<InstructionDependency> {
    let mut accounts: Vec<String> = Vec::new();
    let mut arguments: Vec<String> = Vec::new();
    for dependency in dependencies {
        match dependency {
            InstructionDependency::Account(name) => {
                if !accounts.contains(&name) {
                    accounts.push(name);
                }
            }
            InstructionDependency::Argument(name) => {
                if !arguments.contains(&name) {
                    arguments.push(name);
                }
            }
        }
    }
    accounts
        .into_iter()
        .map(InstructionDependency::Account)
        .chain(arguments.into_iter().map(InstructionDependency::Argument))
        .collect()
}

/// All account/argument dependencies of an instruction, deduplicated.
///
/// The Rust counterpart of the upstream `getInstructionDependencies` for an
/// `instructionNode`.
pub fn get_instruction_dependencies(instruction: &InstructionNode) -> Vec<InstructionDependency> {
    let mut deps = Vec::new();
    for account in &instruction.accounts {
        deps.extend(dependencies_of(account.default_value.as_ref().as_ref()));
    }
    for argument in instruction
        .arguments
        .iter()
        .chain(instruction.extra_arguments.iter())
    {
        deps.extend(dependencies_of(argument.default_value.as_ref().as_ref()));
    }
    deduplicate(deps)
}
