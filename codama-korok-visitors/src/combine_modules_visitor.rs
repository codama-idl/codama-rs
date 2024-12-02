use crate::KorokVisitor;
use codama_nodes::{Node, ProgramNode, RootNode};

#[derive(Default)]
pub struct CombineModulesVisitor {}

impl CombineModulesVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for CombineModulesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) {
        for crate_korok in &mut korok.crates {
            self.visit_crate(crate_korok);
        }
        korok.node = combine_crates(&korok.node, &korok.crates);
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
        korok.node = combine_items(&korok.node, &korok.items);
    }

    fn visit_file_module(&mut self, korok: &mut codama_koroks::FileModuleKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
        korok.node = combine_items(&korok.node, &korok.items);
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
        korok.node = combine_items(&korok.node, &korok.items);
    }
}

fn combine_crates(
    initial_node: &Option<Node>,
    crates: &Vec<codama_koroks::CrateKorok>,
) -> Option<Node> {
    // Get all available nodes from the items.
    let crate_nodes = crates
        .iter()
        .filter_map(|item| item.node.clone())
        .collect::<Vec<_>>();

    combine_nodes(initial_node, crate_nodes)
}

fn combine_items(
    initial_node: &Option<Node>,
    items: &Vec<codama_koroks::ItemKorok>,
) -> Option<Node> {
    // Get all available nodes from the items.
    let item_nodes = items
        .iter()
        .filter_map(|item| item.node())
        .collect::<Vec<_>>();

    combine_nodes(initial_node, item_nodes)
}

fn combine_nodes(initial_node: &Option<Node>, nodes_to_merge: Vec<Node>) -> Option<Node> {
    // Create the new RootNode to bind all items together.
    // If there is already a node that is not a RootNode nor a ProgramNode,
    // return it without combining the items.
    let mut root_node = match initial_node {
        Some(Node::Root(root)) => root.clone(),
        Some(Node::Program(program)) => RootNode::new(program.clone()),
        None => RootNode::default(),
        _ => return initial_node.clone(),
    };

    // Merge each node into the root node one by one.
    for item_node in nodes_to_merge {
        merge_into_root_node(&mut root_node, item_node);
    }

    Some(root_node.into())
}

fn merge_into_root_node(root: &mut RootNode, node: Node) {
    match node {
        Node::Root(node) => merge_root_nodes(root, node),
        Node::Program(node) => merge_root_nodes(root, RootNode::new(node)),
        Node::Account(node) => root.program.accounts.push(node),
        // TODO: Check if instruction needs merging instead of pushing.
        // E.g. InstructionNode with accounts only and InstructionNode with arguments only, with the same name.
        Node::Instruction(node) => root.program.instructions.push(node),
        Node::DefinedType(node) => root.program.defined_types.push(node),
        Node::Error(node) => root.program.errors.push(node),
        Node::Pda(node) => root.program.pdas.push(node),
        _ => (),
    }
}

fn merge_root_nodes(this: &mut RootNode, that: RootNode) {
    // Get an array of all programs to merge.
    let mut those_programs = Vec::new();
    those_programs.push(that.program);
    those_programs.extend(that.additional_programs);

    // For each program to merge.
    for that_program in those_programs {
        // Check if it can be merged with the main root program.
        if is_same_program(&this.program, &that_program) {
            merge_program_nodes(&mut this.program, that_program);
            continue;
        }

        // Then, check if it can be merged with any additional program.
        let found = this
            .additional_programs
            .iter_mut()
            .find(|p| is_same_program(p, &that_program));

        if let Some(additional_program) = found {
            // If so, merge it with the additional program found.
            merge_program_nodes(additional_program, that_program);
        } else {
            // Otherwise, add it as another additional program.
            this.additional_programs.push(that_program);
        }
    }
}

fn is_same_program(this: &ProgramNode, that: &ProgramNode) -> bool {
    this.public_key == that.public_key
}

fn merge_program_nodes(this: &mut ProgramNode, that: ProgramNode) {
    if this.name.is_empty() {
        this.name = that.name;
    }
    if this.public_key.is_empty() {
        this.public_key = that.public_key;
    }
    if this.version.is_empty() {
        this.version = that.version;
    }
    if this.origin.is_none() {
        this.origin = that.origin;
    }
    if this.docs.is_empty() {
        this.docs = that.docs;
    }
    this.accounts.extend(that.accounts);
    this.instructions.extend(that.instructions);
    this.defined_types.extend(that.defined_types);
    this.errors.extend(that.errors);
    this.pdas.extend(that.pdas);
}
