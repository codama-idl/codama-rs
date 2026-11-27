use crate::{
    contextual_value_items::{get_contextual_value_items, is_linked_public_key, ContextualValue},
    identifier_collisions::{
        get_identifier_collision_items, get_nested_instruction_collision_items,
    },
    validation_error_item, validation_item, LogLevel, ValidationItem,
};
use codama_errors::CodamaError;
use codama_nodes::{
    parse_interpolated_intent, FieldDiscriminatorNode, HasKind, Identifier, InjectedValueNode,
    InstructionAccountLinkNode, InstructionInputValueNode, InstructionNode, InterpolatedIntentPart,
    IsSigner, Node, NodeRef, PdaSeedNode, ProgramLinkNode, ProgramNode, StructFieldTypeNode, Text,
    ValueNode,
};
use codama_visitors_core::{
    get_resolved_instruction_inputs, has_defined_type_finite_value, is_defined_type_cyclic,
    is_value_node_of_type, resolve_type_path, visit, InstructionDependency, LinkableDictionary,
    MergeVisitor, NodePath, NodeStack, ProvidedScope, RecordLinkablesVisitor,
    ResolveInstructionInputsError, ResolveTypePathError, ResolvedInstructionInput, Visitor,
    VisitorExt,
};
use std::cell::Cell;

/// Creates a visitor that lists the [`ValidationItem`]s of a tree, like `getValidationItemsVisitor` in the
/// JavaScript library.
///
/// ```
/// use codama_nodes::{ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, StringTypeNode};
/// use codama_nodes::{DefinedTypeNode};
/// use codama_validators::get_validation_items_visitor;
/// use codama_visitors_core::visit;
///
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![DefinedTypeNode::new(
///         "config",
///         StructTypeNode::new(vec![
///             StructFieldTypeNode::new("label", StringTypeNode::utf8()),
///             StructFieldTypeNode::new("Label", StringTypeNode::utf8()),
///         ]),
///     )],
///     version: "1.0.0".into(),
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
///
/// let items = visit(&get_validation_items_visitor(), &root);
/// assert_eq!(
///     items.iter().map(ToString::to_string).collect::<Vec<_>>(),
///     [concat!(
///         "[error] Struct field identifier \"Label\" collides with \"label\" once converted to camelCase. ",
///         "at [programNode]token.[definedTypeNode]config.[structTypeNode].[structFieldTypeNode]Label",
///     )]
/// );
/// ```
pub fn get_validation_items_visitor<'a>() -> ValidationItemsVisitor<'a> {
    ValidationItemsVisitor::default()
}

/// Lists the [`ValidationItem`]s of the visited tree. Created by [`get_validation_items_visitor`].
///
/// It owns the [`NodeStack`], [`LinkableDictionary`] and [`ProvidedScope`] that its checks read, and keeps
/// them in sync with the visit: the first node it visits records the links of its whole tree, so it can
/// visit several trees in turn.
#[derive(Debug)]
pub struct ValidationItemsVisitor<'a> {
    stack: NodeStack<'a>,
    linkables: LinkableDictionary<'a>,
    /// The `provides` of the instructions enclosing the node being visited, to resolve injected values.
    scope: ProvidedScope<'a>,
    is_first_visit: Cell<bool>,
}

impl Default for ValidationItemsVisitor<'_> {
    fn default() -> Self {
        Self {
            stack: NodeStack::default(),
            linkables: LinkableDictionary::default(),
            scope: ProvidedScope::default(),
            is_first_visit: Cell::new(true),
        }
    }
}

impl<'a> Visitor<'a, Vec<ValidationItem<'a>>> for ValidationItemsVisitor<'a> {
    fn visit(
        &self,
        node: NodeRef<'a>,
        recurse: &dyn Fn(NodeRef<'a>) -> Vec<ValidationItem<'a>>,
    ) -> Vec<ValidationItem<'a>> {
        // The first node records the links of its tree, so every check can resolve them. This is what
        // `record_linkables_on_first_visit` does, but the chain below is rebuilt for every node, so that
        // adapter would see each node as a first visit. The flag lives on `self` instead.
        let _rearm = match self.is_first_visit.replace(false) {
            true => {
                self.linkables.clear();
                visit(&RecordLinkablesVisitor::new(&self.linkables), node);
                Some(RearmOnDrop(&self.is_first_visit))
            }
            false => None,
        };

        // The stack and the scope hold the node while its checks run, and the children go through
        // `recurse`, so they run through this visitor too.
        MergeVisitor::new(|_, items: Vec<Vec<ValidationItem<'a>>>| {
            items.into_iter().flatten().collect()
        })
        .extend(|node, scope| {
            let mut items = self.get_node_items(node);
            items.extend(scope.next(node));
            items
        })
        .record_node_stack(&self.stack)
        .record_provided_scope(&self.scope)
        .visit(node, recurse)
    }
}

impl<'a> ValidationItemsVisitor<'a> {
    /// The items of `node` itself, whose path is the current path of the stack.
    fn get_node_items(&self, node: NodeRef<'a>) -> Vec<ValidationItem<'a>> {
        let path = self.stack.get_path();
        let Some(item) = get_missing_identifier_item(node, &path) else {
            return self.get_node_kind_items(node, &path);
        };
        // A node naming another by an empty identifier names nothing, so its other checks would only
        // repeat this item, such as a link "pointing to a missing PDA named """.
        if names_another_node(node) {
            return vec![item];
        }
        let mut items = vec![item];
        items.extend(self.get_node_kind_items(node, &path));
        items
    }

    /// The items of `node` that are specific to its kind, sorted by kind.
    fn get_node_kind_items(
        &self,
        node: NodeRef<'a>,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let item = |level, message: &str| vec![validation_item(level, message, path.clone())];
        match node {
            NodeRef::AccountBumpValue(value) => get_contextual_value_items(
                ContextualValue::Account(&value.identifier),
                path,
                &self.linkables,
            ),
            NodeRef::AccountDataValue(value) => get_contextual_value_items(
                ContextualValue::AccountData(&value.account, value.path.as_ref()),
                path,
                &self.linkables,
            ),
            NodeRef::AccountLink(link) => self.get_missing_link_items(
                "account",
                &link.identifier,
                link.program.as_ref(),
                path,
            ),
            NodeRef::AccountValue(value) => get_contextual_value_items(
                ContextualValue::Account(&value.identifier),
                path,
                &self.linkables,
            ),
            NodeRef::DataValue(value) => get_contextual_value_items(
                ContextualValue::Data(&value.path),
                path,
                &self.linkables,
            ),
            NodeRef::DateTimeType(node) => {
                self.get_ticks_per_second_items(node.ticks_per_second, path)
            }
            NodeRef::DefinedType(_) => self.get_finite_value_items(path),
            NodeRef::DefinedTypeLink(link) => self.get_missing_link_items(
                "defined type",
                &link.identifier,
                link.program.as_ref(),
                path,
            ),
            NodeRef::DurationType(node) => {
                self.get_ticks_per_second_items(node.ticks_per_second, path)
            }
            NodeRef::EnumType(r#enum) => {
                let mut items = vec![];
                if r#enum.variants.is_empty() {
                    items.extend(item(LogLevel::Warn, "Enum has no variants."));
                }
                items.extend(get_identifier_collision_items(
                    &r#enum.variants,
                    "Enum variant",
                    "",
                    path,
                ));
                items
            }
            NodeRef::Error(node) => match &node.message {
                Text::String(message) if message.is_empty() => {
                    item(LogLevel::Warn, "Error has no message.")
                }
                _ => vec![],
            },
            NodeRef::FieldDiscriminator(discriminator) => {
                self.get_field_discriminator_items(discriminator, path)
            }
            NodeRef::FixedPointType(node) if node.scale == 0 => item(
                LogLevel::Error,
                &format!(
                    "Fixed-point scale must be a positive integer, got {}.",
                    node.scale
                ),
            ),
            NodeRef::InjectedValue(injected) => self.get_injected_value_items(injected, path),
            NodeRef::Instruction(instruction) => self.get_instruction_items(instruction, path),
            NodeRef::InstructionAccountLink(link) => {
                self.get_missing_instruction_account_link_items(link, path)
            }
            NodeRef::InstructionDisplay(display) => match &display.interpolated_intent {
                Some(intent) => self.get_interpolated_intent_items(intent, path),
                None => vec![],
            },
            NodeRef::InstructionLink(link) => self.get_missing_link_items(
                "instruction",
                &link.identifier,
                link.program.as_ref(),
                path,
            ),
            NodeRef::Pda(pda) => {
                let seeds = pda.seeds.iter().filter_map(|seed| match seed {
                    PdaSeedNode::Variable(seed) => Some(seed),
                    _ => None,
                });
                let context = format!(" in PDA \"{}\"", pda.identifier);
                get_identifier_collision_items(seeds, "PDA seed", &context, path)
            }
            NodeRef::PdaLink(link) => {
                self.get_missing_link_items("PDA", &link.identifier, link.program.as_ref(), path)
            }
            NodeRef::Program(program) => get_program_items(program, path),
            NodeRef::Root(root) => {
                let programs = std::iter::once(&root.program).chain(&root.additional_programs);
                get_identifier_collision_items(programs, "Program", "", path)
            }
            NodeRef::StructType(r#struct) => {
                get_identifier_collision_items(&r#struct.fields, "Struct field", "", path)
            }
            NodeRef::Text(text) if text.plugins.is_empty() => item(
                LogLevel::Info,
                "Text node has no plugins; use a plain string instead.",
            ),
            NodeRef::TupleType(tuple) if tuple.items.is_empty() => {
                item(LogLevel::Warn, "Tuple has no items.")
            }
            NodeRef::ValueLabelDisplay(label) => self.get_value_label_items(&label.value, path),
            _ => vec![],
        }
    }

    /// The items of an instruction: collisions between the identifiers of its collections, and the
    /// items of the default values of its inputs.
    fn get_instruction_items(
        &self,
        instruction: &'a InstructionNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let context = format!(" in instruction \"{}\"", instruction.identifier);
        let collisions = [
            get_identifier_collision_items(
                &instruction.accounts,
                "Instruction account",
                &context,
                path,
            ),
            get_identifier_collision_items(
                &instruction.remaining_accounts,
                "Instruction remaining accounts",
                &context,
                path,
            ),
            get_identifier_collision_items(&instruction.provides, "Provided value", &context, path),
            get_identifier_collision_items(
                &instruction.sub_instructions,
                "Sub-instruction",
                &context,
                path,
            ),
        ];
        let mut items: Vec<_> = collisions.into_iter().flatten().collect();
        items.extend(self.get_instruction_input_items(instruction, path));
        items
    }

    /// The items of a link that points to no node, such as a PDA link with a typo.
    ///
    /// A link naming a program that is not in the tree points outside of it, e.g. to the PDA of another
    /// program whose client is generated separately, so it cannot be checked and is skipped. Links to
    /// programs are never reported for the same reason. A typo in the name of the program, such as
    /// `program = "tokne"`, therefore cannot be told apart from an external program and is not reported.
    fn get_missing_link_items(
        &self,
        kind: &str,
        identifier: &Identifier,
        program: Option<&'a ProgramLinkNode>,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        if self.is_external_program(program) || self.linkables.has(path) {
            return vec![];
        }
        let message = format!("Pointing to a missing {kind} named \"{identifier}\"");
        vec![validation_item(LogLevel::Error, message, path.clone())]
    }

    /// The items of an instruction account link that points to no account. When the instruction it
    /// names is missing, only that instruction link is reported, as it is visited on its own.
    fn get_missing_instruction_account_link_items(
        &self,
        link: &'a InstructionAccountLinkNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        if let Some(instruction) = &link.instruction {
            let instruction_path = path.child(instruction);
            if self.is_external_program(instruction.program.as_ref())
                || !self.linkables.has(&instruction_path)
            {
                return vec![];
            }
        }
        self.get_missing_link_items("instruction account", &link.identifier, None, path)
    }

    /// Whether `program` names a program that is not in the tree.
    fn is_external_program(&self, program: Option<&'a ProgramLinkNode>) -> bool {
        program.is_some_and(|program| {
            !self
                .linkables
                .has(&NodePath::from(vec![NodeRef::from(program)]))
        })
    }

    /// The items of the default values of the inputs of an instruction: cycles and invalid dependencies,
    /// and data fields defaulting to the bump of an account that may be a signer.
    fn get_instruction_input_items(
        &self,
        instruction: &InstructionNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let inputs =
            match get_resolved_instruction_inputs(path, &self.linkables, Default::default()) {
                Ok(inputs) => inputs,
                Err(cause) => {
                    // An invalid dependency stems from a contextual value, such as an unknown account or an
                    // unresolvable data path, which reports it on its own where it is written: within this
                    // instruction, or within the parent instruction providing it. Like the JavaScript
                    // library, a value provided by a parent whose dependency only holds in the parent is
                    // not reported at all; sub-instructions are on their way out, so this is left as is.
                    let is_reported_by_dependency = match &cause {
                        ResolveInstructionInputsError::InvalidDependency { dependency, .. } => {
                            let value = match dependency.as_ref() {
                                InstructionDependency::Account(account) => {
                                    ContextualValue::Account(&account.identifier)
                                }
                                InstructionDependency::Data(data) => {
                                    ContextualValue::Data(&data.path)
                                }
                            };
                            !get_contextual_value_items(value, path, &self.linkables).is_empty()
                        }
                        _ => false,
                    };
                    return match is_reported_by_dependency {
                        true => vec![],
                        false => vec![validation_error_item(cause, path.clone())],
                    };
                }
            };

        // A bump can only be derived from an account that is not a signer.
        inputs
            .iter()
            .filter_map(|input| match input {
                ResolvedInstructionInput::DataField(field) => match &field.resolved_default_value {
                    Some(InstructionInputValueNode::AccountBumpValue(bump)) => Some((field, bump)),
                    _ => None,
                },
                ResolvedInstructionInput::Account(_) => None,
            })
            .filter(|(_, bump)| {
                instruction.accounts.iter().any(|account| {
                    account.identifier == bump.identifier && account.is_signer != IsSigner::False
                })
            })
            .map(|(field, bump)| {
                let message = format!(
                    "Data field \"{}\" cannot default to the bump of the \"{}\" account as it may be a signer.",
                    field.path, bump.identifier
                );
                validation_item(LogLevel::Error, message, path.clone())
            })
            .collect()
    }

    /// The items of an injected value that is neither provided nor given a fallback.
    ///
    /// Only injections consumed within an instruction have a known final context to resolve against.
    /// Injections within a provided node or the fallback of another injection are resolved as part of
    /// the value that consumes them, and value labels report injections on their own.
    fn get_injected_value_items(
        &self,
        injected: &InjectedValueNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let ancestors = &path.nodes()[..path.len() - 1];
        let is_within_instruction = ancestors
            .iter()
            .any(|ancestor| matches!(ancestor, NodeRef::Instruction(_)));
        let is_nested = ancestors.iter().any(|ancestor| {
            matches!(
                ancestor,
                NodeRef::Provided(_) | NodeRef::InjectedValue(_) | NodeRef::ValueLabelDisplay(_)
            )
        });
        if !is_within_instruction || is_nested {
            return vec![];
        }
        match self.scope.resolve::<Node>(injected) {
            Ok(Some(_)) => vec![],
            Ok(None) => vec![validation_item(
                LogLevel::Error,
                format!(
                    "Injected value \"{}\" is not provided and has no fallback.",
                    injected.key
                ),
                path.clone(),
            )],
            Err(cause) => vec![validation_error_item(cause, path.clone())],
        }
    }

    /// The items of a field discriminator whose path does not point to a field of the data it identifies.
    /// Identification only reads the bytes being identified, so the path must not cross account links.
    fn get_field_discriminator_items(
        &self,
        discriminator: &FieldDiscriminatorNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let owner_index = path.nodes().iter().rposition(|node| {
            matches!(
                node,
                NodeRef::Account(_) | NodeRef::Event(_) | NodeRef::Instruction(_)
            )
        });
        let Some(owner_index) = owner_index else {
            return vec![];
        };
        let data = match path.nodes()[owner_index] {
            NodeRef::Account(account) => Some(&*account.data),
            NodeRef::Event(event) => Some(&*event.data),
            NodeRef::Instruction(instruction) => instruction.data.as_ref().as_ref(),
            _ => unreachable!("the owner is an account, an event or an instruction"),
        };
        let error = |message: String| validation_item(LogLevel::Error, message, path.clone());
        let Some(data) = data else {
            return vec![error(format!(
                "Field discriminator path \"{}\" cannot be resolved as there is no data.",
                discriminator.path
            ))];
        };
        let mut source: NodePath<'a> = path.nodes()[..=owner_index].iter().copied().collect();
        source.push(data);
        match resolve_type_path(
            &source,
            &discriminator.path,
            &self.linkables,
            Default::default(),
        ) {
            Ok(resolved) if matches!(resolved.last(), Some(NodeRef::StructFieldType(_))) => vec![],
            Ok(_) => vec![error(format!(
                "Field discriminator path \"{}\" must point to a struct field.",
                discriminator.path
            ))],
            Err(ResolveTypePathError::LinkedNodeNotFound { .. }) => vec![],
            Err(cause) => {
                let crosses_account_link = matches!(
                    &cause,
                    ResolveTypePathError::CannotResolvePath { path, segment, .. }
                        if segment == "data" && is_linked_public_key(path.last())
                );
                let cause = CodamaError::from(cause);
                let message = match crosses_account_link {
                    true => format!(
                        "Field discriminator path \"{}\" must not cross an account link.",
                        discriminator.path
                    ),
                    false => cause.to_string(),
                };
                vec![error(message).with_cause(cause)]
            }
        }
    }

    /// The items of an interpolated intent whose placeholders are invalid or do not resolve.
    fn get_interpolated_intent_items(
        &self,
        intent: &Text,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let parts = match parse_interpolated_intent(intent.content()) {
            Ok(parts) => parts,
            Err(cause) => return vec![validation_error_item(cause, path.clone())],
        };
        parts
            .iter()
            .flat_map(|part| {
                let value = match part {
                    InterpolatedIntentPart::Text(_) => return vec![],
                    InterpolatedIntentPart::Data(value) => ContextualValue::Data(&value.path),
                    InterpolatedIntentPart::Account(value) => {
                        ContextualValue::Account(&value.identifier)
                    }
                    InterpolatedIntentPart::AccountData(value) => {
                        ContextualValue::AccountData(&value.account, value.path.as_ref())
                    }
                };
                get_contextual_value_items(value, path, &self.linkables)
            })
            .collect()
    }

    /// The items of a value label that is not a concrete value of the type of its field.
    fn get_value_label_items(
        &self,
        value: &ValueNode,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let Some((field_path, field)) = path.until_last_with::<StructFieldTypeNode>() else {
            return vec![];
        };
        let error = |message: String| vec![validation_item(LogLevel::Error, message, path.clone())];
        let kind = match value {
            ValueNode::Injected(_) => Some("an injected"),
            ValueNode::Constant(_) => Some("a constant"),
            _ => None,
        };
        if let Some(kind) = kind {
            return error(format!(
                "Value labels must be concrete values, got {kind} value."
            ));
        }
        let field_path = field_path.child(&*field.r#type);
        match is_value_node_of_type(value, &field_path, &self.linkables) {
            Ok(true) | Err(CodamaError::LinkedNodeNotFound { .. }) => vec![],
            Ok(false) => error(format!(
                "Value label does not match the type of field \"{}\".",
                field.identifier
            )),
            Err(cause) => vec![validation_error_item(cause, path.clone())],
        }
    }

    /// The items of a cyclic defined type whose every value nests another one, which can be neither
    /// encoded nor decoded.
    ///
    /// The cyclic check matches the codecs of the JavaScript library, which only fail for links back to
    /// their own type: an enum without variants alone is reported by its own warning. Links can only be
    /// resolved within a program, so types outside one are skipped.
    fn get_finite_value_items(&self, path: &NodePath<'a>) -> Vec<ValidationItem<'a>> {
        if path.find_last::<ProgramNode>().is_none() {
            return vec![];
        }
        let is_cyclic = is_defined_type_cyclic(path, &self.linkables)
            .expect("the path ends with a defined type within a program");
        if !is_cyclic
            || has_defined_type_finite_value(path, &self.linkables)
                .expect("the path ends with a defined type within a program")
        {
            return vec![];
        }
        // Keep the error that creating its codec raises in the JavaScript library as the cause, so both
        // report it the same way.
        let cause = CodamaError::DefinedTypeHasNoFiniteValue {
            path: path.to_string(),
        };
        vec![validation_error_item(cause, path.clone())]
    }

    /// The items of a date-time or a duration whose `ticks_per_second` is not a positive integer.
    fn get_ticks_per_second_items(
        &self,
        ticks_per_second: Option<u32>,
        path: &NodePath<'a>,
    ) -> Vec<ValidationItem<'a>> {
        let Some(0) = ticks_per_second else {
            return vec![];
        };
        // Keep the error as the cause, like the JavaScript library, where creating the codec of the
        // type raises it.
        let cause = CodamaError::InvalidTicksPerSecond {
            path: path.to_string(),
            ticks_per_second: 0,
        };
        vec![validation_error_item(cause, path.clone())]
    }
}

/// The item of a node whose identifier is empty, which a node with an identifier must not have. An
/// account data value names its account under `account` rather than `identifier`, so it is checked too.
fn get_missing_identifier_item<'a>(
    node: NodeRef<'a>,
    path: &NodePath<'a>,
) -> Option<ValidationItem<'a>> {
    if let NodeRef::AccountDataValue(value) = node {
        return value.account.is_empty().then(|| {
            validation_item(
                LogLevel::Error,
                "Account data value has no account.",
                path.clone(),
            )
        });
    }
    if !node.identifier()?.is_empty() {
        return None;
    }
    let message = match node {
        NodeRef::Account(_) => "Account has no identifier.",
        NodeRef::AccountBumpValue(_) => "Account bump value has no identifier.",
        NodeRef::AccountLink(_) => "Pointing to an account with no identifier.",
        NodeRef::AccountValue(_) => "Account value has no identifier.",
        NodeRef::Constant(_) => "Constant has no identifier.",
        NodeRef::DefinedType(_) => "Defined type has no identifier.",
        NodeRef::DefinedTypeLink(_) => "Pointing to a defined type with no identifier.",
        NodeRef::EnumVariantType(_) => "Enum variant has no identifier.",
        NodeRef::Error(_) => "Error has no identifier.",
        NodeRef::Event(_) => "Event has no identifier.",
        NodeRef::Instruction(_) => "Instruction has no identifier.",
        NodeRef::InstructionAccount(_) => "Instruction account has no identifier.",
        NodeRef::InstructionAccountLink(_) => {
            "Pointing to an instruction account with no identifier."
        }
        NodeRef::InstructionLink(_) => "Pointing to an instruction with no identifier.",
        NodeRef::InstructionRemainingAccounts(_) => {
            "Instruction remaining accounts have no identifier."
        }
        NodeRef::Pda(_) => "PDA has no identifier.",
        NodeRef::PdaLink(_) => "Pointing to a PDA with no identifier.",
        NodeRef::PdaSeedValue(_) => "PDA seed value has no identifier.",
        NodeRef::Program(_) => "Program has no identifier.",
        NodeRef::ProgramLink(_) => "Pointing to a program with no identifier.",
        NodeRef::Provided(_) => "Provided value has no identifier.",
        NodeRef::PublicKeyValue(_) => "Public key value has an empty identifier.",
        NodeRef::StructFieldType(_) => "Struct field has no identifier.",
        NodeRef::StructFieldValue(_) => "Struct field value has no identifier.",
        NodeRef::VariablePdaSeed(_) => "PDA seed has no identifier.",
        node => {
            return Some(validation_item(
                LogLevel::Error,
                format!("Node of kind [{}] has no identifier.", node.kind()),
                path.clone(),
            ))
        }
    };
    Some(validation_item(LogLevel::Error, message, path.clone()))
}

/// Whether `node` names another node by its identifier, such as a link or an account value, rather than
/// defining one.
fn names_another_node(node: NodeRef) -> bool {
    matches!(
        node,
        NodeRef::AccountBumpValue(_)
            | NodeRef::AccountDataValue(_)
            | NodeRef::AccountLink(_)
            | NodeRef::AccountValue(_)
            | NodeRef::DefinedTypeLink(_)
            | NodeRef::InstructionAccountLink(_)
            | NodeRef::InstructionLink(_)
            | NodeRef::PdaLink(_)
            | NodeRef::PdaSeedValue(_)
            | NodeRef::ProgramLink(_)
            | NodeRef::StructFieldValue(_)
    )
}

/// The items of a program: a missing public key or version, and collisions between the identifiers
/// of its collections.
fn get_program_items<'a>(program: &'a ProgramNode, path: &NodePath<'a>) -> Vec<ValidationItem<'a>> {
    let mut items = vec![];
    if program.public_key.is_empty() {
        items.push(validation_item(
            LogLevel::Error,
            "Program has no public key.",
            path.clone(),
        ));
    }
    if program.version.is_empty() {
        items.push(validation_item(
            LogLevel::Warn,
            "Program has no version.",
            path.clone(),
        ));
    }
    let context = format!(" in program \"{}\"", program.identifier);
    let collisions = [
        get_identifier_collision_items(&program.accounts, "Account", &context, path),
        get_identifier_collision_items(&program.instructions, "Instruction", &context, path),
        get_identifier_collision_items(&program.defined_types, "Defined type", &context, path),
        get_identifier_collision_items(&program.pdas, "PDA", &context, path),
        get_identifier_collision_items(&program.events, "Event", &context, path),
        get_identifier_collision_items(&program.errors, "Error", &context, path),
        get_identifier_collision_items(&program.constants, "Constant", &context, path),
        get_nested_instruction_collision_items(program, path),
    ];
    items.extend(collisions.into_iter().flatten());
    items
}

/// Marks the next visit as a first visit when dropped, even when the current visit panics.
struct RearmOnDrop<'c>(&'c Cell<bool>);

impl Drop for RearmOnDrop<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}
