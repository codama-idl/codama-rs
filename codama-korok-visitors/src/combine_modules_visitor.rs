use crate::KorokVisitor;
use codama_errors::CodamaResult;
use codama_koroks::KorokTrait;
use codama_nodes::{Node, ProgramNode, RootNode};

#[derive(Default)]
pub struct CombineModulesVisitor;

impl CombineModulesVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl KorokVisitor for CombineModulesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        korok.node = combine_koroks(&korok.node, &korok.crates);
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        korok.node = combine_koroks(&korok.node, &korok.items);
        Ok(())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        korok.node = combine_koroks(&korok.node, &korok.items);
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        korok.node = combine_koroks(&korok.node, &korok.items);
        Ok(())
    }
}

/// Create a single RootNode from an initial node and a list of nodes to merge.
fn combine_koroks<T: KorokTrait>(initial_node: &Option<Node>, koroks: &[T]) -> Option<Node> {
    // Create the new RootNode to bind all items together from the exisiting node, in any.
    // - If there is already a RootNode or ProgramNode, use this as a starting point.
    // - If there is no existing node, use None and let the merging create a new one if needed.
    // - If there is any other node, return it as-is without combining the nodes.
    let mut this_root_node = match initial_node {
        Some(Node::Root(root)) => Some(root.clone()),
        Some(Node::Program(program)) => Some(RootNode::new(program.clone())),
        None => None,
        _ => return initial_node.clone(),
    };

    // Get all nodes from the koroks to merge.
    let nodes_to_merge = koroks
        .iter()
        .filter_map(|item| item.node().clone())
        .collect::<Vec<_>>();

    // Convert all nodes into RootNodes and merge them with the binding root node.
    let from_parent = this_root_node.is_some();
    for that_root_node in get_root_nodes_to_merge(nodes_to_merge) {
        merge_root_nodes(&mut this_root_node, that_root_node, from_parent);
    }

    this_root_node.map(Into::into)
}

/// Convert all nodes to merge into RootNodes.
fn get_root_nodes_to_merge(nodes: Vec<Node>) -> Vec<RootNode> {
    // Split the nodes into:
    // - Nodes that can be converted into RootNodes (Root, Program).
    // - All other nodes that we will refer to as scraps.
    let (roots_and_programs, scraps) = nodes
        .into_iter()
        .partition::<Vec<Node>, _>(|node| matches!(node, Node::Root(_) | Node::Program(_)));

    // Convert all "rootable" nodes into RootNodes.
    let mut roots = roots_and_programs
        .into_iter()
        .filter_map(|node| match node {
            Node::Root(node) => Some(node),
            Node::Program(node) => Some(RootNode::new(node)),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Try to get a RootNode from all the scraps.
    if let Some(root) = get_scraps_root_node(scraps) {
        roots.push(root)
    }

    roots
}

/// Go through all "scraps" nodes and try to get a shared RootNode from them.
fn get_scraps_root_node(nodes: Vec<Node>) -> Option<RootNode> {
    let mut has_scraps = false;
    let mut root = RootNode::default();

    for node in nodes {
        match node {
            Node::Account(node) => {
                root.program.accounts.push(node);
                has_scraps = true
            }
            Node::Instruction(node) => {
                root.program.instructions.push(node);
                has_scraps = true
            }
            Node::DefinedType(node) => {
                root.program.defined_types.push(node);
                has_scraps = true
            }
            Node::Error(node) => {
                root.program.errors.push(node);
                has_scraps = true
            }
            Node::Pda(node) => {
                root.program.pdas.push(node);
                has_scraps = true
            }
            _ => (),
        }
    }

    has_scraps.then_some(root)
}

/// Merge `that` RootNode into `this` RootNode.
fn merge_root_nodes(this: &mut Option<RootNode>, that: RootNode, from_parent: bool) {
    // If there is no root node yet, set it to the one provided.
    let Some(this) = this else {
        *this = Some(that);
        return;
    };

    // Get an array of all programs to merge.
    let mut those_programs = Vec::new();
    those_programs.push(that.program);
    those_programs.extend(that.additional_programs);

    // For each program to merge.
    for that_program in those_programs {
        // Check if it can be merged with the main root program.
        if should_merge_program_nodes(&this.program, &that_program, from_parent) {
            merge_program_nodes(&mut this.program, that_program);
            continue;
        }

        // Then, check if it can be merged with any additional program.
        let found = this
            .additional_programs
            .iter_mut()
            .find(|p| should_merge_program_nodes(p, &that_program, from_parent));

        if let Some(additional_program) = found {
            // If so, merge it with the additional program found.
            merge_program_nodes(additional_program, that_program);
        } else {
            // Otherwise, add it as another additional program.
            this.additional_programs.push(that_program);
        }
    }
}

/// Check if two ProgramNodes should be merged together.
fn should_merge_program_nodes(this: &ProgramNode, that: &ProgramNode, from_parent: bool) -> bool {
    this.public_key == that.public_key || (from_parent && that.public_key.is_empty())
}

/// Merge `that` ProgramNode into `this` ProgramNode.
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
