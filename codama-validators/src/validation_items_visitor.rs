use crate::{
    identifier_collisions::{
        child_path, get_identifier_collision_items, get_nested_instruction_collision_items,
    },
    validation_item, LogLevel, ValidationItem,
};
use codama_errors::CodamaError;
use codama_nodes::{NodeRef, PdaSeedNode, Text};
use codama_visitors_core::{
    visit, LinkableDictionary, MergeVisitor, NodeStack, ProvidedScope, RecordLinkablesVisitor,
    Visitor, VisitorExt,
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
        // The first node records the links of its tree, so every check can resolve them.
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
        let item = |level, message: String| validation_item(level, message, path.clone());
        let error = |message: &str| vec![item(LogLevel::Error, message.to_string())];
        match node {
            NodeRef::Account(account) if account.identifier.is_empty() => {
                error("Account has no identifier.")
            }
            NodeRef::AccountLink(link) if !self.linkables.has(&path) => vec![item(
                LogLevel::Error,
                format!(
                    "Pointing to a missing account named \"{}\"",
                    link.identifier
                ),
            )],
            NodeRef::DateTimeType(node) => self.get_ticks_per_second_items(node.ticks_per_second),
            NodeRef::DurationType(node) => self.get_ticks_per_second_items(node.ticks_per_second),
            NodeRef::DefinedType(defined_type) if defined_type.identifier.is_empty() => {
                error("Defined type has no identifier.")
            }
            NodeRef::DefinedTypeLink(link) if link.identifier.is_empty() => {
                error("Pointing to a defined type with no identifier.")
            }
            NodeRef::DefinedTypeLink(link) if !self.linkables.has(&path) => vec![item(
                LogLevel::Error,
                format!(
                    "Pointing to a missing defined type named \"{}\"",
                    link.identifier
                ),
            )],
            NodeRef::EnumType(r#enum) => {
                let mut items = vec![];
                if r#enum.variants.is_empty() {
                    items.push(item(LogLevel::Warn, "Enum has no variants.".to_string()));
                }
                items.extend(get_identifier_collision_items(
                    &r#enum.variants,
                    "Enum variant",
                    "",
                    &path,
                ));
                items
            }
            NodeRef::EnumVariantType(variant) if variant.identifier.is_empty() => {
                error("Enum variant has no identifier.")
            }
            NodeRef::Error(node) => {
                let mut items = vec![];
                if node.identifier.is_empty() {
                    items.push(item(
                        LogLevel::Error,
                        "Error has no identifier.".to_string(),
                    ));
                }
                if matches!(&node.message, Text::String(message) if message.is_empty()) {
                    items.push(item(LogLevel::Warn, "Error has no message.".to_string()));
                }
                items
            }
            NodeRef::FixedPointType(node) if node.scale == 0 => vec![item(
                LogLevel::Error,
                format!(
                    "Fixed-point scale must be a positive integer, got {}.",
                    node.scale
                ),
            )],
            NodeRef::Instruction(instruction) => {
                let mut items = vec![];
                if instruction.identifier.is_empty() {
                    items.push(item(
                        LogLevel::Error,
                        "Instruction has no identifier.".to_string(),
                    ));
                }
                for account in &instruction.accounts {
                    if account.identifier.is_empty() {
                        items.push(validation_item(
                            LogLevel::Error,
                            "Instruction account has no identifier.",
                            child_path(&path, account.into()),
                        ));
                    }
                }
                let context = format!(" in instruction \"{}\"", instruction.identifier);
                items.extend(get_identifier_collision_items(
                    &instruction.accounts,
                    "Instruction account",
                    &context,
                    &path,
                ));
                items.extend(get_identifier_collision_items(
                    &instruction.remaining_accounts,
                    "Instruction remaining accounts",
                    &context,
                    &path,
                ));
                items.extend(get_identifier_collision_items(
                    &instruction.provides,
                    "Provided value",
                    &context,
                    &path,
                ));
                items.extend(get_identifier_collision_items(
                    &instruction.sub_instructions,
                    "Sub-instruction",
                    &context,
                    &path,
                ));
                items
            }
            NodeRef::Pda(pda) => {
                let seeds = pda.seeds.iter().filter_map(|seed| match seed {
                    PdaSeedNode::Variable(seed) => Some(seed),
                    _ => None,
                });
                let context = format!(" in PDA \"{}\"", pda.identifier);
                get_identifier_collision_items(seeds, "PDA seed", &context, &path)
            }
            NodeRef::Program(program) => {
                let mut items = vec![];
                if program.identifier.is_empty() {
                    items.push(item(
                        LogLevel::Error,
                        "Program has no identifier.".to_string(),
                    ));
                }
                if program.public_key.is_empty() {
                    items.push(item(
                        LogLevel::Error,
                        "Program has no public key.".to_string(),
                    ));
                }
                if program.version.is_empty() {
                    items.push(item(LogLevel::Warn, "Program has no version.".to_string()));
                }
                let context = format!(" in program \"{}\"", program.identifier);
                let collisions = [
                    get_identifier_collision_items(&program.accounts, "Account", &context, &path),
                    get_identifier_collision_items(
                        &program.instructions,
                        "Instruction",
                        &context,
                        &path,
                    ),
                    get_identifier_collision_items(
                        &program.defined_types,
                        "Defined type",
                        &context,
                        &path,
                    ),
                    get_identifier_collision_items(&program.pdas, "PDA", &context, &path),
                    get_identifier_collision_items(&program.events, "Event", &context, &path),
                    get_identifier_collision_items(&program.errors, "Error", &context, &path),
                    get_identifier_collision_items(&program.constants, "Constant", &context, &path),
                    get_nested_instruction_collision_items(program, &path),
                ];
                items.extend(collisions.into_iter().flatten());
                items
            }
            NodeRef::Root(root) => {
                let programs = std::iter::once(&root.program).chain(&root.additional_programs);
                get_identifier_collision_items(programs, "Program", "", &path)
            }
            NodeRef::StructFieldType(field) if field.identifier.is_empty() => {
                error("Struct field has no identifier.")
            }
            NodeRef::StructType(r#struct) => {
                get_identifier_collision_items(&r#struct.fields, "Struct field", "", &path)
            }
            NodeRef::Text(text) if text.plugins.is_empty() => vec![item(
                LogLevel::Info,
                "Text node has no plugins; use a plain string instead.".to_string(),
            )],
            NodeRef::TupleType(tuple) if tuple.items.is_empty() => {
                vec![item(LogLevel::Warn, "Tuple has no items.".to_string())]
            }
            _ => vec![],
        }
    }

    /// The items of a date-time or a duration whose `ticks_per_second` is not a positive integer.
    fn get_ticks_per_second_items(&self, ticks_per_second: Option<u32>) -> Vec<ValidationItem<'a>> {
        let Some(0) = ticks_per_second else {
            return vec![];
        };
        let path = self.stack.get_path();
        // Keep the error that reading the value would raise, so both report it the same way.
        let cause = CodamaError::InvalidTicksPerSecond {
            path: path.to_string(),
            ticks_per_second: 0,
        };
        vec![validation_item(LogLevel::Error, cause.to_string(), path).with_cause(cause)]
    }
}

/// Marks the next visit as a first visit when dropped, even when the current visit panics.
struct RearmOnDrop<'c>(&'c Cell<bool>);

impl Drop for RearmOnDrop<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}
