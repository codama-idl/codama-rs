use crate::KorokVisitor;
use codama_nodes::{Node, RootNode};

#[derive(Default)]
pub struct CombineModulesVisitor {}

impl CombineModulesVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl KorokVisitor for CombineModulesVisitor {
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

fn combine_items(node: &Option<Node>, items: &Vec<codama_koroks::ItemKorok>) -> Option<Node> {
    // Create the new RootNode to bind all items together.
    // If there is already a node that is not a RootNode nor a ProgramNode,
    // return it without combining the items.
    let mut root_node = match node {
        Some(Node::Root(root)) => root.clone(),
        Some(Node::Program(program)) => RootNode::new(program.clone()),
        None => RootNode::default(),
        _ => return node.clone(),
    };

    // Get all available nodes from the items.
    let item_nodes = items
        .iter()
        .filter_map(|item| item.node())
        .collect::<Vec<_>>();

    // Merge them one by one into the root node.
    for item_node in item_nodes {
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

fn merge_root_nodes(_this: &mut RootNode, _that: RootNode) {
    unimplemented!() // TODO
}
