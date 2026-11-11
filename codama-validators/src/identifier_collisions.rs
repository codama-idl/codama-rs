use crate::{validation_item, LogLevel, ValidationItem};
use codama_nodes::{casing::camel_case, HasIdentifier, InstructionNode, NodeRef, ProgramNode};
use codama_visitors_core::NodePath;
use std::collections::HashMap;

/// Reports the identifiers of a sibling set that are not unique: exact duplicates, and distinct
/// identifiers sharing the same camelCase form, which is the casing-collision rule of the spec. Each
/// duplicate or colliding entry is reported once, against that entry, whose path is `parent` and the
/// entry.
///
/// `label` names the entries, such as `Struct field`, and `context` is appended to the message to name
/// the scope, such as ` in instruction "transfer"`.
pub(crate) fn get_identifier_collision_items<'a, T>(
    entries: impl IntoIterator<Item = &'a T>,
    label: &str,
    context: &str,
    parent: &NodePath<'a>,
) -> Vec<ValidationItem<'a>>
where
    T: HasIdentifier + 'a,
    &'a T: Into<NodeRef<'a>>,
{
    let mut items = vec![];
    let mut identifiers_by_camel_case = HashMap::<String, &str>::new();
    for entry in entries {
        let identifier = entry.identifier().as_str();
        if identifier.is_empty() {
            continue;
        }
        let Some(existing) = identifiers_by_camel_case.get(&camel_case(identifier)) else {
            identifiers_by_camel_case.insert(camel_case(identifier), identifier);
            continue;
        };
        let message = match *existing == identifier {
            true => format!("{label} identifier \"{identifier}\" is not unique{context}."),
            false => format!(
                "{label} identifier \"{identifier}\" collides with \"{existing}\" once converted to camelCase{context}."
            ),
        };
        items.push(validation_item(
            LogLevel::Error,
            message,
            child_path(parent, entry.into()),
        ));
    }
    items
}

/// Reports the instructions whose identifier collides with another instruction of the same program, at
/// any nesting level: exact duplicates, and distinct identifiers sharing the same camelCase form.
/// Instructions are linked by identifier within their program, so instruction links would otherwise
/// resolve to the wrong one, and renderers flatten sub-instructions into one file per instruction.
///
/// Instructions are walked depth first, parents before their sub-instructions, and each colliding
/// instruction is reported once, against itself and the first identifier sharing its camelCase form.
/// Collisions with an earlier sibling are left to [`get_identifier_collision_items`], so they are not
/// reported twice.
pub(crate) fn get_nested_instruction_collision_items<'a>(
    program: &'a ProgramNode,
    program_path: &NodePath<'a>,
) -> Vec<ValidationItem<'a>> {
    let context = format!(
        " across the instructions and sub-instructions of program \"{}\"",
        program.identifier
    );
    let mut collisions = NestedCollisions {
        context: &context,
        items: vec![],
        identifiers_by_camel_case: HashMap::new(),
    };
    collisions.visit(&program.instructions, program_path);
    collisions.items
}

struct NestedCollisions<'c, 'a> {
    context: &'c str,
    items: Vec<ValidationItem<'a>>,
    /// The first identifier of each camelCase form, with the sibling sets that hold it, by address.
    identifiers_by_camel_case: HashMap<String, (&'a str, Vec<*const InstructionNode>)>,
}

impl<'a> NestedCollisions<'_, 'a> {
    fn visit(&mut self, siblings: &'a [InstructionNode], parent: &NodePath<'a>) {
        for instruction in siblings {
            let path = child_path(parent, NodeRef::from(instruction));
            let identifier = instruction.identifier.as_str();
            if !identifier.is_empty() {
                let siblings_id = siblings.as_ptr();
                match self
                    .identifiers_by_camel_case
                    .get_mut(&camel_case(identifier))
                {
                    None => {
                        self.identifiers_by_camel_case
                            .insert(camel_case(identifier), (identifier, vec![siblings_id]));
                    }
                    Some((existing, sibling_sets)) => {
                        if !sibling_sets.contains(&siblings_id) {
                            let context = self.context;
                            let message = match *existing == identifier {
                                true => format!(
                                    "Instruction identifier \"{identifier}\" is not unique{context}."
                                ),
                                false => format!(
                                    "Instruction identifier \"{identifier}\" collides with \"{existing}\" once converted to camelCase{context}."
                                ),
                            };
                            self.items.push(validation_item(
                                LogLevel::Error,
                                message,
                                path.clone(),
                            ));
                            sibling_sets.push(siblings_id);
                        }
                    }
                }
            }
            self.visit(&instruction.sub_instructions, &path);
        }
    }
}

/// The path of `child` within the node at the end of `parent`.
pub(crate) fn child_path<'a>(parent: &NodePath<'a>, child: NodeRef<'a>) -> NodePath<'a> {
    let mut path = parent.clone();
    path.push(child);
    path
}
