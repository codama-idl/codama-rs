use crate::KorokVisitor;
use codama_nodes::{Node, ProgramNode};

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
        korok.node = merge_items(&korok.items);
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) {
        for item_korok in &mut korok.items {
            self.visit_item(item_korok);
        }
        korok.node = merge_items(&korok.items);
    }
}

fn merge_items(items: &Vec<codama_koroks::ItemKorok>) -> Option<Node> {
    let mut program_node = ProgramNode::default();
    let nodes = items
        .iter()
        .filter_map(|item| item.node())
        .collect::<Vec<_>>();

    for node in nodes {
        merge_into_program_node(&mut program_node, node);
    }

    Some(program_node.into())
}

fn merge_into_program_node(program_node: &mut ProgramNode, node: Node) {
    match node {
        Node::Account(node) => program_node.accounts.push(node),
        Node::DefinedType(node) => program_node.defined_types.push(node),
        Node::Error(node) => program_node.errors.push(node),
        // TODO: Check if instruction needs merging instead of pushing.
        // E.g. InstructionNode with accounts only and InstructionNode with arguments only, with the same name.
        Node::Instruction(node) => program_node.instructions.push(node),
        Node::Pda(node) => program_node.pdas.push(node),
        // TODO: Node::Program(node) => ...
        // TODO: Node::Root(node) => ...
        _ => (),
    }
}
