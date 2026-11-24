use crate::{
    get_instruction_data_fields, get_provided_scope_from_path, LinkableDictionary, NodePath,
    ProvidedScope,
};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{
    AccountValueNode, DataValueNode, HasKind, Identifier, InstructionAccountNode,
    InstructionInputValueNode, InstructionNode, IsSigner, NodeKinds, NodeRef, Path,
    PdaSeedValueValue, PdaValueProgramId, StructFieldTypeNode, ValueNode,
};
use std::collections::{HashMap, HashSet};

/// An account or a data field of an instruction whose default value is resolved, from
/// [`get_resolved_instruction_inputs`].
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedInstructionInput<'a> {
    Account(ResolvedInstructionAccount<'a>),
    DataField(ResolvedInstructionDataField<'a>),
}

impl<'a> ResolvedInstructionInput<'a> {
    /// The node of the input: an instruction account or a struct field of the instruction data.
    pub fn node(&self) -> NodeRef<'a> {
        match self {
            Self::Account(account) => account.node.into(),
            Self::DataField(field) => field.node.into(),
        }
    }

    /// The default value of the input, once its injected values are resolved.
    pub fn resolved_default_value(&self) -> Option<&InstructionInputValueNode> {
        match self {
            Self::Account(account) => account.resolved_default_value.as_ref(),
            Self::DataField(field) => field.resolved_default_value.as_ref(),
        }
    }
}

/// An instruction account with its resolved default value.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedInstructionAccount<'a> {
    pub node: &'a InstructionAccountNode,
    /// The accounts and data fields that the default value depends on, resolved before this account.
    pub depends_on: Vec<InstructionDependency>,
    /// Whether a data field of the instruction defaults to the bump of this account.
    pub is_pda: bool,
    /// The default value of the account, once its injected values are resolved. It is `None` when the
    /// account has no default value, or when one of its injected values resolves to nothing.
    pub resolved_default_value: Option<InstructionInputValueNode>,
    /// Whether the account is optional, once its default value is accounted for.
    pub resolved_is_optional: bool,
    /// Whether the account is a signer, once its default value is accounted for.
    pub resolved_is_signer: IsSigner,
}

/// A struct field of the instruction data with its resolved default value.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedInstructionDataField<'a> {
    pub node: &'a StructFieldTypeNode,
    /// The path of the field within the data of the instruction, such as `config.bump`.
    pub path: Path,
    /// The accounts and data fields that the default value depends on, resolved before this field.
    pub depends_on: Vec<InstructionDependency>,
    /// The default value of the field, once its injected values are resolved.
    pub resolved_default_value: Option<InstructionInputValueNode>,
}

/// An account or a data field that the default value of an input depends on.
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionDependency {
    Account(AccountValueNode),
    Data(DataValueNode),
}

/// Options of [`get_resolved_instruction_inputs`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResolveInstructionInputsOptions {
    /// Whether to include data fields whose resolved default value is a plain value, which has
    /// nothing to resolve.
    pub include_data_value_nodes: bool,
}

/// Resolves the default values of the accounts and data fields of the instruction at the end of
/// `instruction_path`, and returns them in dependency order, like `getResolvedInstructionInputsVisitor`
/// in the JavaScript library.
///
/// Injected values resolve against the `provides` of every instruction of `instruction_path`, the
/// instruction itself included, so a sub-instruction resolves against its parents too. An injected
/// value that resolves to nothing is not an error: the input keeps no default value, as a reusable
/// data shape may offer an optional hook that the instruction does not fill. The path must contain the
/// program of the instruction for linked data to be followed.
///
/// Fails when the default values depend on each other in a cycle, when a default value depends on an
/// account or a data field that the instruction does not have, when a PDA default value takes an
/// optional account as a seed, and when a provided value is not an instruction input value.
///
/// ```
/// use codama_nodes::{AccountValueNode, InstructionAccountNode, InstructionNode, NodeRef};
/// use codama_visitors_core::{get_resolved_instruction_inputs, LinkableDictionary, NodePath};
///
/// let instruction = InstructionNode {
///     identifier: "transfer".into(),
///     accounts: vec![
///         InstructionAccountNode {
///             default_value: Box::new(Some(AccountValueNode::new("authority").into())),
///             ..InstructionAccountNode::new("owner", false, true)
///         },
///         InstructionAccountNode::new("authority", false, true),
///     ],
///     ..InstructionNode::default()
/// };
/// let path = NodePath::from(vec![NodeRef::from(&instruction)]);
/// let inputs =
///     get_resolved_instruction_inputs(&path, &LinkableDictionary::default(), Default::default())
///         .unwrap();
///
/// // The owner defaults to the authority, so the authority is resolved first.
/// let order: Vec<_> = inputs.iter().map(|input| input.node().identifier().unwrap().to_string()).collect();
/// assert_eq!(order, ["authority", "owner"]);
/// ```
pub fn get_resolved_instruction_inputs<'a>(
    instruction_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
    options: ResolveInstructionInputsOptions,
) -> CodamaResult<Vec<ResolvedInstructionInput<'a>>> {
    let Some(NodeRef::Instruction(instruction)) = instruction_path.last() else {
        return Ok(vec![]);
    };
    let scope = get_provided_scope_from_path(instruction_path);
    let data_fields: Vec<DataInput<'a>> = get_instruction_data_fields(instruction_path, linkables)
        .into_iter()
        .map(|field| DataInput {
            node: field.field,
            key: field.path,
            path: field.field_path,
        })
        .collect();

    // The data fields and their resolved default values are fixed for the whole resolution.
    let mut data_defaults = HashMap::new();
    for field in &data_fields {
        let value = match field.node.default_value.as_ref() {
            Some(value) => resolve_default_value(&scope, value.into())?,
            None => None,
        };
        data_defaults.insert(field.key.to_string(), value);
    }
    let bump_accounts = data_defaults
        .values()
        .filter_map(|value| match value {
            Some(InstructionInputValueNode::AccountBumpValue(bump)) => {
                Some(bump.identifier.clone())
            }
            _ => None,
        })
        .collect();

    let mut resolver = Resolver {
        instruction,
        instruction_path,
        scope,
        data_fields,
        data_defaults,
        bump_accounts,
        stack: vec![],
        resolved: vec![],
        resolved_accounts: HashMap::new(),
        resolved_data: HashMap::new(),
    };

    let mut inputs: Vec<Input<'a>> = instruction.accounts.iter().map(Input::Account).collect();
    for field in &resolver.data_fields {
        // A plain value has nothing to resolve, so it is skipped unless requested.
        let include = match &resolver.data_defaults[field.key.as_str()] {
            None => false,
            Some(value) => {
                options.include_data_value_nodes || !ValueNode::KINDS.contains(&value.kind())
            }
        };
        if include {
            inputs.push(Input::Data(field.clone()));
        }
    }
    for input in inputs {
        resolver.resolve_input(input)?;
    }
    Ok(resolver.resolved)
}

/// An input of the dependency graph: an account, or a data field with its key and path.
#[derive(Debug, Clone)]
enum Input<'a> {
    Account(&'a InstructionAccountNode),
    Data(DataInput<'a>),
}

#[derive(Debug, Clone)]
struct DataInput<'a> {
    node: &'a StructFieldTypeNode,
    key: Path,
    path: NodePath<'a>,
}

impl<'a> Input<'a> {
    fn key(&self) -> &str {
        match self {
            Self::Account(account) => account.identifier.as_str(),
            Self::Data(field) => field.key.as_str(),
        }
    }

    fn is_same(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Account(_), Self::Account(_)) | (Self::Data(_), Self::Data(_))
        ) && self.key() == other.key()
    }

    fn path(&self, instruction_path: &NodePath<'a>) -> NodePath<'a> {
        match self {
            Self::Account(account) => {
                let mut path = instruction_path.clone();
                path.push(*account);
                path
            }
            Self::Data(field) => field.path.clone(),
        }
    }
}

struct Resolver<'r, 'a> {
    instruction: &'a InstructionNode,
    instruction_path: &'r NodePath<'a>,
    scope: ProvidedScope<'a>,
    data_fields: Vec<DataInput<'a>>,
    data_defaults: HashMap<String, Option<InstructionInputValueNode>>,
    bump_accounts: HashSet<Identifier>,
    /// The inputs being resolved, to detect cycles.
    stack: Vec<Input<'a>>,
    resolved: Vec<ResolvedInstructionInput<'a>>,
    /// The index of each resolved account and data field in `resolved`.
    resolved_accounts: HashMap<Identifier, usize>,
    resolved_data: HashMap<String, usize>,
}

impl<'a> Resolver<'_, 'a> {
    fn resolve_input(&mut self, input: Input<'a>) -> CodamaResult<()> {
        let is_resolved = match &input {
            Input::Account(account) => self.resolved_accounts.contains_key(&account.identifier),
            Input::Data(field) => self.resolved_data.contains_key(field.key.as_str()),
        };
        if is_resolved {
            return Ok(());
        }
        if self.stack.iter().any(|entry| entry.is_same(&input)) {
            let cycle: Vec<&Input<'a>> = self.stack.iter().chain([&input]).collect();
            return Err(
                CodamaError::CyclicDependencyDetectedWhenResolvingInstructionDefaultValues {
                    path: self.instruction_path.to_string(),
                    formatted_cycle: cycle
                        .iter()
                        .map(|entry| entry.key())
                        .collect::<Vec<_>>()
                        .join(" -> "),
                    cycle: cycle
                        .iter()
                        .map(|entry| entry.path(self.instruction_path).to_string())
                        .collect(),
                },
            );
        }

        self.stack.push(input.clone());
        let resolved = match &input {
            Input::Account(account) => {
                ResolvedInstructionInput::Account(self.resolve_account(&input, account)?)
            }
            Input::Data(field) => {
                ResolvedInstructionInput::DataField(self.resolve_data_field(&input, field)?)
            }
        };
        self.stack.pop();

        let index = self.resolved.len();
        match &input {
            Input::Account(account) => {
                self.resolved_accounts
                    .insert(account.identifier.clone(), index);
            }
            Input::Data(field) => {
                self.resolved_data.insert(field.key.to_string(), index);
            }
        }
        self.resolved.push(resolved);
        Ok(())
    }

    fn resolve_account(
        &mut self,
        input: &Input<'a>,
        account: &'a InstructionAccountNode,
    ) -> CodamaResult<ResolvedInstructionAccount<'a>> {
        let resolved_default_value = match account.default_value.as_ref() {
            Some(value) => resolve_default_value(&self.scope, value.into())?,
            None => None,
        };
        let depends_on = get_dependencies(resolved_default_value.as_ref().map(NodeRef::from));
        self.resolve_dependencies(input, &depends_on)?;

        let mut resolved = ResolvedInstructionAccount {
            node: account,
            depends_on,
            is_pda: self.bump_accounts.contains(&account.identifier),
            resolved_default_value: None,
            resolved_is_optional: account.is_optional.unwrap_or(false),
            resolved_is_signer: account.is_signer,
        };
        let signer_or_either = match account.is_signer {
            IsSigner::False => IsSigner::False,
            _ => IsSigner::Either,
        };
        match &resolved_default_value {
            Some(InstructionInputValueNode::AccountValue(value)) => {
                // The default account is resolved first, so its resolved flags carry along a chain of
                // defaults, such as `a` defaulting to `b` defaulting to `c`.
                let default_account = self.resolved_account(&value.identifier);
                resolved.resolved_is_signer =
                    match (account.is_signer, default_account.resolved_is_signer) {
                        (IsSigner::False, IsSigner::False) => IsSigner::False,
                        (IsSigner::True, IsSigner::True) => IsSigner::True,
                        _ => IsSigner::Either,
                    };
                resolved.resolved_is_optional = default_account.resolved_is_optional;
            }
            Some(
                InstructionInputValueNode::PublicKeyValue(_)
                | InstructionInputValueNode::ProgramLink(_)
                | InstructionInputValueNode::ProgramIdValue(_),
            ) => {
                resolved.resolved_is_signer = signer_or_either;
                resolved.resolved_is_optional = false;
            }
            Some(InstructionInputValueNode::PdaValue(pda)) => {
                resolved.resolved_is_signer = signer_or_either;
                resolved.resolved_is_optional = false;
                for seed in &pda.seeds {
                    let PdaSeedValueValue::Account(seed_account) = &*seed.value else {
                        continue;
                    };
                    if !self
                        .resolved_account(&seed_account.identifier)
                        .resolved_is_optional
                    {
                        continue;
                    }
                    // A PDA value injected by a provider is not within the account, so its path starts
                    // at the PDA value. The account's own PDA value starts at the account, even when
                    // resolving an injected seed within it rebuilt it.
                    let is_injected = matches!(
                        account.default_value.as_ref(),
                        Some(InstructionInputValueNode::InjectedValue(_))
                    );
                    let mut path = match is_injected {
                        true => vec![],
                        false => {
                            let mut path = self.instruction_path.clone();
                            path.push(account);
                            path.nodes().to_vec()
                        }
                    };
                    path.push(NodeRef::from(pda));
                    path.push(NodeRef::from(seed));
                    return Err(CodamaError::CannotUseOptionalAccountAsPdaSeedValue {
                        seed_value_name: seed_account.identifier.to_string(),
                        path: NodePath::from(path).to_string(),
                    });
                }
            }
            Some(
                InstructionInputValueNode::IdentityValue(_)
                | InstructionInputValueNode::PayerValue(_),
            ) => {
                resolved.resolved_is_optional = false;
            }
            _ => {}
        }
        resolved.resolved_default_value = resolved_default_value;
        Ok(resolved)
    }

    fn resolve_data_field(
        &mut self,
        input: &Input<'a>,
        field: &DataInput<'a>,
    ) -> CodamaResult<ResolvedInstructionDataField<'a>> {
        let resolved_default_value = self.data_defaults[field.key.as_str()].clone();
        let depends_on = get_dependencies(resolved_default_value.as_ref().map(NodeRef::from));
        self.resolve_dependencies(input, &depends_on)?;
        Ok(ResolvedInstructionDataField {
            node: field.node,
            path: field.key.clone(),
            depends_on,
            resolved_default_value,
        })
    }

    fn resolve_dependencies(
        &mut self,
        parent: &Input<'a>,
        dependencies: &[InstructionDependency],
    ) -> CodamaResult<()> {
        for dependency in dependencies {
            let input = match dependency {
                InstructionDependency::Account(account) => self
                    .instruction
                    .accounts
                    .iter()
                    .find(|candidate| candidate.identifier == account.identifier)
                    .map(Input::Account),
                InstructionDependency::Data(data) => self
                    .find_field_by_path(&data.path)
                    .cloned()
                    .map(Input::Data),
            };
            let Some(input) = input else {
                let (dependency_kind, dependency_name) = match dependency {
                    InstructionDependency::Account(account) => {
                        (account.kind(), account.identifier.to_string())
                    }
                    InstructionDependency::Data(data) => (data.kind(), data.path.to_string()),
                };
                return Err(CodamaError::InvalidInstructionDefaultValueDependency {
                    dependency_kind: dependency_kind.to_string(),
                    dependency_name,
                    dependent: parent.key().to_string(),
                    path: self.instruction_path.to_string(),
                });
            };
            self.resolve_input(input)?;
        }
        Ok(())
    }

    /// The data field that a data path refers to. A path deeper than the struct fields, such as an
    /// item of an array like `config.fees[0]`, refers to the deepest field that holds it.
    fn find_field_by_path(&self, path: &Path) -> Option<&DataInput<'a>> {
        let path = path.as_str();
        self.data_fields
            .iter()
            .filter(|field| {
                let key = field.key.as_str();
                path == key
                    || path
                        .strip_prefix(key)
                        .is_some_and(|rest| rest.starts_with('.') || rest.starts_with('['))
            })
            .max_by_key(|field| field.key.as_str().len())
    }

    fn resolved_account(&self, identifier: &Identifier) -> &ResolvedInstructionAccount<'a> {
        let index = self.resolved_accounts[identifier];
        match &self.resolved[index] {
            ResolvedInstructionInput::Account(account) => account,
            ResolvedInstructionInput::DataField(_) => unreachable!("indexed as an account"),
        }
    }
}

/// Resolves every injected value within a default value, such as a PDA seed.
fn resolve_default_value<'a>(
    scope: &ProvidedScope<'a>,
    value: NodeRef<'a>,
) -> CodamaResult<Option<InstructionInputValueNode>> {
    scope.resolve::<InstructionInputValueNode>(value)
}

/// The accounts and data fields that a default value depends on, recursively, each listed once.
/// Dependencies are created from the identifier or path they refer to, without plugins.
fn get_dependencies(value: Option<NodeRef>) -> Vec<InstructionDependency> {
    let account = |identifier: &Identifier| {
        InstructionDependency::Account(AccountValueNode::new(identifier.clone()))
    };
    let data = |path: &Path| InstructionDependency::Data(DataValueNode::new(path.clone()));
    match value {
        Some(NodeRef::AccountValue(value)) => vec![account(&value.identifier)],
        Some(NodeRef::AccountBumpValue(value)) => vec![account(&value.identifier)],
        Some(NodeRef::AccountDataValue(value)) => vec![account(&value.account)],
        Some(NodeRef::DataValue(value)) => vec![data(&value.path)],
        Some(NodeRef::PdaValue(pda)) => {
            let mut dependencies: Vec<_> = pda
                .seeds
                .iter()
                .filter_map(|seed| match &*seed.value {
                    PdaSeedValueValue::Account(value) => Some(account(&value.identifier)),
                    PdaSeedValueValue::Data(value) => Some(data(&value.path)),
                    _ => None,
                })
                .collect();
            match &*pda.program_id {
                Some(PdaValueProgramId::Account(value)) => {
                    dependencies.push(account(&value.identifier))
                }
                Some(PdaValueProgramId::Data(value)) => dependencies.push(data(&value.path)),
                None => {}
            }
            deduplicate_dependencies(dependencies)
        }
        Some(NodeRef::ConditionalValue(conditional)) => {
            let mut dependencies = get_dependencies(Some(NodeRef::from(&*conditional.condition)));
            dependencies.extend(get_dependencies(
                conditional.if_true.as_ref().as_ref().map(NodeRef::from),
            ));
            dependencies.extend(get_dependencies(
                conditional.if_false.as_ref().as_ref().map(NodeRef::from),
            ));
            deduplicate_dependencies(dependencies)
        }
        _ => vec![],
    }
}

/// Lists each account once, then each data field once, in the order they first appear.
fn deduplicate_dependencies(
    dependencies: Vec<InstructionDependency>,
) -> Vec<InstructionDependency> {
    let mut accounts: Vec<InstructionDependency> = vec![];
    let mut data: Vec<InstructionDependency> = vec![];
    for dependency in dependencies {
        let list = match &dependency {
            InstructionDependency::Account(_) => &mut accounts,
            InstructionDependency::Data(_) => &mut data,
        };
        if !list.contains(&dependency) {
            list.push(dependency);
        }
    }
    accounts.extend(data);
    accounts
}
