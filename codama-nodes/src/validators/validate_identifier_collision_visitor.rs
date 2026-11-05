use crate::{
    casing::camel_case, walk_enum_type, walk_instruction, walk_pda, walk_program, walk_root,
    walk_struct_type, EnumTypeNode, HasIdentifier, Identifier, InstructionNode, NodePath,
    NodeVisitor, PathSegment, PdaNode, PdaSeedNode, ProgramNode, RootNode, StructTypeNode,
};
use std::collections::{hash_map::Entry, HashMap, HashSet};

/// Node visitor that lists the identifier errors of a root node in `errors`, one message per error.
/// A program without an identifier is an error: an empty identifier only marks a name not set yet while modules merge.
/// Two identifiers of one sibling set collide when their camelCase forms are equal (spec casing-collision rule).
/// Instructions also collide across the sub-instructions of their program, at any depth.
/// The sibling sets are those of the JS validator (`getValidationItemsVisitor`), wherever they occur in the tree.
/// `StructValueNode.fields` and `PdaValueNode.seeds` refer to declared identifiers, so they are not sibling sets here.
#[derive(Default)]
pub struct ValidateIdentifierCollisionVisitor {
    pub errors: Vec<String>,
}

impl ValidateIdentifierCollisionVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one error per collision among the nodes. `label` names the sibling set.
    fn report_collisions<'a, T: HasIdentifier + 'a>(
        &mut self,
        label: &str,
        nodes: impl IntoIterator<Item = &'a T>,
        path: &NodePath,
    ) {
        let identifiers = nodes.into_iter().map(HasIdentifier::identifier);
        for (identifier, existing) in find_collisions(identifiers) {
            self.errors.push(format!(
                "{label} identifier \"{identifier}\" {}{}.",
                collision_message(identifier, existing),
                at_location_message(path),
            ));
        }
    }
}

impl NodeVisitor for ValidateIdentifierCollisionVisitor {
    fn visit_root<'a>(&mut self, node: &'a RootNode, path: &NodePath<'a>) {
        let programs = || std::iter::once(&node.program).chain(&node.additional_programs);
        let unnamed = programs().filter(|program| program.identifier.is_empty());
        self.errors
            .extend(unnamed.map(missing_program_identifier_message));
        self.report_collisions("Program", programs(), path);
        walk_root(self, node, path);
    }

    fn visit_program<'a>(&mut self, node: &'a ProgramNode, path: &NodePath<'a>) {
        self.report_collisions("Account", &node.accounts, path);
        self.report_collisions("Instruction", &node.instructions, path);
        self.report_collisions("Defined type", &node.defined_types, path);
        self.report_collisions("PDA", &node.pdas, path);
        self.report_collisions("Event", &node.events, path);
        self.report_collisions("Error", &node.errors, path);
        self.report_collisions("Constant", &node.constants, path);
        let mut nested = NestedInstructions::new(&node.identifier);
        nested.collect_siblings(&node.instructions, "instructions", path);
        self.errors.extend(nested.errors);
        walk_program(self, node, path);
    }

    fn visit_instruction<'a>(&mut self, node: &'a InstructionNode, path: &NodePath<'a>) {
        self.report_collisions("Instruction account", &node.accounts, path);
        self.report_collisions(
            "Instruction remaining accounts",
            &node.remaining_accounts,
            path,
        );
        self.report_collisions("Provided value", &node.provides, path);
        self.report_collisions("Sub-instruction", &node.sub_instructions, path);
        walk_instruction(self, node, path);
    }

    fn visit_pda<'a>(&mut self, node: &'a PdaNode, path: &NodePath<'a>) {
        let variable_seeds = node.seeds.iter().filter_map(|seed| match seed {
            PdaSeedNode::Variable(seed) => Some(seed),
            _ => None,
        });
        self.report_collisions("PDA seed", variable_seeds, path);
        walk_pda(self, node, path);
    }

    fn visit_enum_type<'a>(&mut self, node: &'a EnumTypeNode, path: &NodePath<'a>) {
        self.report_collisions("Enum variant", &node.variants, path);
        walk_enum_type(self, node, path);
    }

    fn visit_struct_type<'a>(&mut self, node: &'a StructTypeNode, path: &NodePath<'a>) {
        self.report_collisions("Struct field", &node.fields, path);
        walk_struct_type(self, node, path);
    }
}

/// Instructions link by identifier within their program, and renderers flatten sub-instructions into one namespace.
/// A collision within one sibling set is reported once, by `ValidateIdentifierCollisionVisitor::report_collisions`.
struct NestedInstructions<'a> {
    program: &'a Identifier,
    /// The first identifier of each camelCase form, and the sibling sets that hold that form.
    seen: HashMap<String, (&'a Identifier, HashSet<usize>)>,
    sibling_sets: usize,
    errors: Vec<String>,
}

impl<'a> NestedInstructions<'a> {
    fn new(program: &'a Identifier) -> Self {
        Self {
            program,
            seen: HashMap::new(),
            sibling_sets: 0,
            errors: Vec::new(),
        }
    }

    /// Walks parents before their sub-instructions, as the JS `getNestedInstructionCollisionItems` does.
    fn collect_siblings(
        &mut self,
        siblings: &'a [InstructionNode],
        attribute: &'static str,
        parent: &NodePath<'a>,
    ) {
        let sibling_set = self.sibling_sets;
        self.sibling_sets += 1;
        for (index, instruction) in siblings.iter().enumerate() {
            self.record_sibling(&instruction.identifier, sibling_set, parent);
            let path = parent.join_named(PathSegment::Item(attribute, index), instruction);
            self.collect_siblings(&instruction.sub_instructions, "subInstructions", &path);
        }
    }

    /// Records each sibling set of a camelCase form, and reports the identifier when a known form gets a new set.
    fn record_sibling(
        &mut self,
        identifier: &'a Identifier,
        sibling_set: usize,
        parent: &NodePath,
    ) {
        if identifier.is_empty() {
            return;
        }
        match self.seen.entry(camel_case(identifier.as_str())) {
            Entry::Vacant(entry) => {
                entry.insert((identifier, HashSet::from([sibling_set])));
            }
            Entry::Occupied(mut entry) => {
                let (existing, sibling_sets) = entry.get_mut();
                if sibling_sets.insert(sibling_set) {
                    self.errors.push(format!(
                        "Instruction identifier \"{identifier}\"{} {} across the instructions and \
                         sub-instructions of program \"{}\".",
                        at_location_message(parent),
                        collision_message(identifier, existing),
                        self.program,
                    ));
                }
            }
        }
    }
}

/// Pairs each identifier with the earlier sibling that has the same camelCase form.
/// Empty identifiers are skipped.
fn find_collisions<'a>(
    identifiers: impl IntoIterator<Item = &'a Identifier>,
) -> Vec<(&'a Identifier, &'a Identifier)> {
    let mut first_by_camel_case = HashMap::new();
    let mut collisions = Vec::new();
    for identifier in identifiers {
        if identifier.is_empty() {
            continue;
        }
        match first_by_camel_case.entry(camel_case(identifier.as_str())) {
            Entry::Vacant(entry) => {
                entry.insert(identifier);
            }
            Entry::Occupied(entry) => collisions.push((identifier, *entry.get())),
        }
    }
    collisions
}

fn missing_program_identifier_message(program: &ProgramNode) -> String {
    let program = match program.public_key.is_empty() {
        true => "The program".to_string(),
        false => format!("The program with public key \"{}\"", program.public_key),
    };
    format!(
        "{program} has no identifier. Set one with `codama_program!(identifier = \"...\")` \
         or `#[codama(program(identifier = \"...\", address = \"...\"))]`."
    )
}

/// Says whether the identifier repeats the earlier one or only shares its camelCase form, as the JS messages do.
fn collision_message(identifier: &Identifier, existing: &Identifier) -> String {
    match identifier == existing {
        true => "is not unique".to_string(),
        false => format!("collides with \"{existing}\""),
    }
}

fn at_location_message(path: &NodePath) -> String {
    match path.steps().is_empty() {
        true => String::new(),
        false => format!(" in {path}"),
    }
}
