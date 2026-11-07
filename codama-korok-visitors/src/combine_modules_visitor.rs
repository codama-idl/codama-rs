use crate::KorokVisitor;
use codama_attributes::ProgramDirective;
use codama_errors::{CodamaError, CodamaResult};
use codama_koroks::KorokTrait;
use codama_nodes::{HasIdentifier, HasKind, Node, PluginNode, ProgramNode, RootNode};
use std::path::Path;

#[derive(Default)]
pub struct CombineModulesVisitor {
    force: bool,
}

impl CombineModulesVisitor {
    pub fn new() -> Self {
        Self { force: false }
    }

    pub fn force() -> Self {
        Self { force: true }
    }
}

impl KorokVisitor for CombineModulesVisitor {
    fn visit_root(&mut self, korok: &mut codama_koroks::RootKorok) -> CodamaResult<()> {
        // Unless forced, if the root node is already set, do not combine modules.
        if !self.force && korok.node.is_some() {
            return Ok(());
        }

        self.visit_children(korok)?;
        korok.node = combine_koroks(&korok.node, &korok.crates, "the loaded crates")?;
        Ok(())
    }

    fn visit_crate(&mut self, korok: &mut codama_koroks::CrateKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        let location = with_path("the crate root".to_string(), &korok.store.path);
        korok.node = combine_koroks(&korok.node, &korok.items, &location)?;
        Ok(())
    }

    fn visit_file_module(
        &mut self,
        korok: &mut codama_koroks::FileModuleKorok,
    ) -> CodamaResult<()> {
        self.visit_children(korok)?;
        let location = with_path(format!("module `{}`", korok.ast.ident), &korok.store.path);
        korok.node = combine_koroks(&korok.node, &korok.items, &location)?;
        Ok(())
    }

    fn visit_module(&mut self, korok: &mut codama_koroks::ModuleKorok) -> CodamaResult<()> {
        self.visit_children(korok)?;
        let location = format!("module `{}`", korok.ast.ident);
        korok.node = combine_koroks(&korok.node, &korok.items, &location)?;

        // Apply program directive on the module if present.
        if let Some(node) = korok.node.take() {
            korok.node = Some(ProgramDirective::apply(&korok.attributes, node));
        }

        Ok(())
    }
}

/// Create a single RootNode from an initial node and a list of nodes to merge.
/// `location` names the korok being combined in error messages.
fn combine_koroks<T: KorokTrait>(
    initial_node: &Option<Node>,
    koroks: &[T],
    location: &str,
) -> CodamaResult<Option<Node>> {
    // Create the new RootNode to bind all items together from the exisiting node, in any.
    // - If there is already a RootNode or ProgramNode, use this as a starting point.
    // - If there is no existing node, use None and let the merging create a new one if needed.
    // - If there is any other node, return it as-is without combining the nodes.
    let mut this_root_node = match initial_node {
        Some(Node::Root(root)) => Some(root.clone()),
        Some(Node::Program(program)) => Some(RootNode::new(program.clone())),
        None => None,
        _ => return Ok(initial_node.clone()),
    };

    // Get all nodes from the koroks to merge.
    let nodes_to_merge = koroks
        .iter()
        .filter_map(|item| item.node().clone())
        .collect::<Vec<_>>();

    // Convert all nodes into RootNodes and merge them with the binding root node.
    let from_parent = this_root_node.is_some();
    for that_root_node in get_root_nodes_to_merge(nodes_to_merge, location)? {
        merge_root_nodes(&mut this_root_node, that_root_node, from_parent, location)?;
    }

    Ok(this_root_node.map(Into::into))
}

/// Convert all nodes to merge into RootNodes.
fn get_root_nodes_to_merge(nodes: Vec<Node>, location: &str) -> CodamaResult<Vec<RootNode>> {
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
    if let Some(root) = get_scraps_root_node(scraps, location)? {
        roots.push(root)
    }

    Ok(roots)
}

/// Go through all "scraps" nodes and try to get a shared RootNode from them.
fn get_scraps_root_node(nodes: Vec<Node>, location: &str) -> CodamaResult<Option<RootNode>> {
    let mut has_scraps = false;
    let mut root = RootNode::default();
    let location = program_location(&root.program, location);

    for node in nodes {
        match node {
            Node::Account(node) => {
                add_node_with_identifier(&mut root.program.accounts, node, &location)?;
                has_scraps = true
            }
            Node::Constant(node) => {
                add_node_with_identifier(&mut root.program.constants, node, &location)?;
                has_scraps = true
            }
            Node::Instruction(node) => {
                add_node_with_identifier(&mut root.program.instructions, node, &location)?;
                has_scraps = true
            }
            Node::DefinedType(node) => {
                add_node_with_identifier(&mut root.program.defined_types, node, &location)?;
                has_scraps = true
            }
            Node::Error(node) => {
                add_node_with_identifier(&mut root.program.errors, node, &location)?;
                has_scraps = true
            }
            Node::Pda(node) => {
                add_node_with_identifier(&mut root.program.pdas, node, &location)?;
                has_scraps = true
            }
            Node::Event(node) => {
                add_node_with_identifier(&mut root.program.events, node, &location)?;
                has_scraps = true
            }
            _ => (),
        }
    }

    Ok(has_scraps.then_some(root))
}

/// Merge `that` RootNode into `this` RootNode.
fn merge_root_nodes(
    this: &mut Option<RootNode>,
    that: RootNode,
    from_parent: bool,
    location: &str,
) -> CodamaResult<()> {
    // If there is no root node yet, set it to the one provided.
    let Some(this) = this else {
        *this = Some(that);
        return Ok(());
    };

    merge_plugins(&mut this.plugins, that.plugins, "the root node")?;

    // Get an array of all programs to merge.
    let mut those_programs = Vec::new();
    those_programs.push(that.program);
    those_programs.extend(that.additional_programs);

    // For each program to merge.
    for that_program in those_programs {
        // Check if it can be merged with the main root program.
        if should_merge_program_nodes(&this.program, &that_program, from_parent) {
            merge_program_nodes(&mut this.program, that_program, location)?;
            continue;
        }

        // Then, check if it can be merged with any additional program.
        let found = this
            .additional_programs
            .iter_mut()
            .find(|p| should_merge_program_nodes(p, &that_program, from_parent));

        if let Some(additional_program) = found {
            // If so, merge it with the additional program found.
            merge_program_nodes(additional_program, that_program, location)?;
        } else {
            // Otherwise, add it as another additional program.
            this.additional_programs.push(that_program);
        }
    }

    Ok(())
}

/// Check if two ProgramNodes should be merged together.
fn should_merge_program_nodes(this: &ProgramNode, that: &ProgramNode, from_parent: bool) -> bool {
    this.public_key == that.public_key || (from_parent && that.public_key.is_empty())
}

/// Merge `that` ProgramNode into `this` ProgramNode.
fn merge_program_nodes(
    this: &mut ProgramNode,
    that: ProgramNode,
    location: &str,
) -> CodamaResult<()> {
    if this.identifier.is_empty() {
        this.identifier = that.identifier;
    }
    if this.public_key.is_empty() {
        this.public_key = that.public_key;
    }
    if this.version.is_empty() {
        this.version = that.version;
    }
    if this.docs.is_empty() {
        this.docs = that.docs;
    }
    let location = program_location(this, location);
    merge_nodes_with_identifier(&mut this.accounts, that.accounts, &location)?;
    merge_nodes_with_identifier(&mut this.instructions, that.instructions, &location)?;
    merge_nodes_with_identifier(&mut this.defined_types, that.defined_types, &location)?;
    merge_nodes_with_identifier(&mut this.pdas, that.pdas, &location)?;
    merge_nodes_with_identifier(&mut this.events, that.events, &location)?;
    merge_nodes_with_identifier(&mut this.errors, that.errors, &location)?;
    merge_nodes_with_identifier(&mut this.constants, that.constants, &location)?;
    merge_plugins(&mut this.plugins, that.plugins, &location)
}

/// Merge `that` plugins into `this` plugins.
/// A namespace identifies a plugin, so two different plugins with the same namespace are an error.
fn merge_plugins(
    this: &mut Vec<PluginNode>,
    that: Vec<PluginNode>,
    location: &str,
) -> CodamaResult<()> {
    for plugin in that {
        match this
            .iter()
            .find(|known| known.namespace == plugin.namespace)
        {
            None => this.push(plugin),
            Some(known) if *known == plugin => {}
            Some(_) => {
                return Err(CodamaError::PluginMergeConflict {
                    namespace: plugin.namespace.to_string(),
                    location: location.to_string(),
                })
            }
        }
    }
    Ok(())
}

fn merge_nodes_with_identifier<T>(
    nodes: &mut Vec<T>,
    new_nodes: Vec<T>,
    location: &str,
) -> CodamaResult<()>
where
    T: HasIdentifier + HasKind + PartialEq,
{
    for that in new_nodes {
        add_node_with_identifier(nodes, that, location)?;
    }
    Ok(())
}

/// Add `new_node` to `nodes` unless an equal node is already there.
/// Equal duplicates are expected, as a re-run of this visitor merges each child node again.
/// Two different nodes with the same identifier are an error, as keeping one would drop the other.
fn add_node_with_identifier<T>(nodes: &mut Vec<T>, new_node: T, location: &str) -> CodamaResult<()>
where
    T: HasIdentifier + HasKind + PartialEq,
{
    match nodes
        .iter()
        .find(|node| node.identifier() == new_node.identifier())
    {
        None => nodes.push(new_node),
        Some(existing) if *existing == new_node => {}
        Some(_) => {
            return Err(CodamaError::NodeMergeConflict {
                kind: new_node.kind().to_string(),
                identifier: new_node.identifier().to_string(),
                location: location.to_string(),
            })
        }
    }
    Ok(())
}

/// The location of a merge in an error message: the program, or the module or crate being combined.
/// Modules merge their items before the crate or a `program(...)` directive names the program,
/// so a program without an identifier or a public key falls back to `location`.
fn program_location(program: &ProgramNode, location: &str) -> String {
    if !program.identifier.is_empty() {
        format!("program `{}`", program.identifier.as_str())
    } else if !program.public_key.is_empty() {
        format!("program `{}`", program.public_key)
    } else {
        location.to_string()
    }
}

/// Append the source path to a location, unless the korok was built without one (for example from tokens).
fn with_path(location: String, path: &Path) -> String {
    if path.as_os_str().is_empty() {
        location
    } else {
        format!("{location} ({})", path.display())
    }
}
