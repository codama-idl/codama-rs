use crate::{
    fold_account, fold_account_link, fold_constant, fold_defined_type, fold_defined_type_link,
    fold_event, fold_instruction, fold_instruction_account, fold_instruction_argument, fold_pda,
    fold_pda_link, fold_program, fold_root, fold_type_node, NodeDescriptor, NodePath, NodeSelector,
    TransformVisitor,
};
use codama_nodes::{
    AccountLinkNode, AccountNode, ConstantNode, DefinedTypeLinkNode, DefinedTypeNode, ErrorNode,
    EventNode, HasKind, InstructionAccountNode, InstructionArgumentNode, InstructionNode, LinkNode,
    Node, PdaLinkNode, PdaNode, ProgramLinkNode, ProgramNode, RootNode, TypeNode,
};

/// A transform callback: given the node currently being visited (as a [`Node`])
/// and its [`NodePath`], return the node to replace it with. Returning the node
/// unchanged is a no-op. The callback is `FnMut`, so it may accumulate state
/// across the traversal.
pub type NodeTransform = Box<dyn FnMut(Node, &NodePath) -> Node>;

/// A single `(selector, transform)` rule: when `selector` matches the path of
/// the node being visited, `transform` is applied to it.
pub struct TransformRule {
    pub selector: NodeSelector,
    pub transform: NodeTransform,
}

impl TransformRule {
    pub fn new(
        selector: impl Into<NodeSelector>,
        transform: impl FnMut(Node, &NodePath) -> Node + 'static,
    ) -> Self {
        Self {
            selector: selector.into(),
            transform: Box::new(transform),
        }
    }
}

/// Walks an IDL tree bottom-up (children before their parent), maintaining a
/// [`NodePath`], and applies the matching [`TransformRule`]s to each node.
///
/// The Rust counterpart of `@codama/visitors-core`'s `bottomUpTransformerVisitor`.
/// Run it with [`TransformVisitor::visit_root`] (or any other `visit_*` entry).
///
/// ```
/// use codama_nodes::{AccountNode, Node, RootNode, ProgramNode, StructTypeNode};
/// use codama_visitors_core::{bottom_up_transformer, TransformRule, TransformVisitor};
///
/// let mut program = ProgramNode::new("myProgram", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("oldName", StructTypeNode::new(vec![])));
/// let root = RootNode::new(program);
///
/// let mut transformer = bottom_up_transformer(vec![TransformRule::new(
///     "[accountNode]oldName",
///     |node, _path| match node {
///         Node::Account(mut a) => { a.name = "newName".into(); Node::Account(a) }
///         other => other,
///     },
/// )]);
/// let root = transformer.visit_root(root);
/// assert_eq!(root.program.accounts[0].name.as_ref(), "newName");
/// ```
///
/// ## Scope of this first iteration
///
/// Rules can target **type nodes** (where a kind-changing transform is allowed,
/// e.g. an `arrayTypeNode` becoming a `bytesTypeNode`) and the **named
/// top-level nodes** (`rootNode`, `programNode`, `accountNode`, `instructionNode`,
/// `instructionAccountNode`, `instructionArgumentNode`, `pdaNode`,
/// `definedTypeNode`, `constantNode`, `errorNode`, `eventNode`), which are
/// kind-preserving. These are also the only kinds recorded in the [`NodePath`],
/// so selector ancestry is expressed in terms of them.
///
/// Deletion is supported for top-level named nodes via [`delete_nodes`] (a node
/// returned unchanged stays; matching a delete selector drops it from its `Vec`
/// slot). Not yet covered (deliberate follow-ups): deleting type/value nodes or
/// required children (no bubbling); selecting
/// **value/count/discriminator/pdaSeed/link** nodes, **struct fields**, and the
/// **`NestedTypeNode` leaves** reached through wrappers (e.g. `accountNode.data`,
/// an `enumTypeNode`'s size). Top-down traversal is available via
/// [`top_down_transformer`].
pub struct BottomUpTransformer {
    rules: Vec<TransformRule>,
    delete: Vec<NodeSelector>,
    path: NodePath,
    pending_delete: bool,
    /// When `true`, rules are applied to a node *before* recursing into its
    /// children (top-down) rather than after (bottom-up).
    top_down: bool,
}

impl BottomUpTransformer {
    pub fn new(rules: Vec<TransformRule>) -> Self {
        Self {
            rules,
            delete: Vec::new(),
            path: NodePath::new(),
            pending_delete: false,
            top_down: false,
        }
    }

    /// Also deletes every node matching the given selectors, in addition to
    /// applying the configured rules — combining [`bottom_up_transformer`] with
    /// the behavior of [`delete_nodes`]. Used by the `update_*` visitors to honor
    /// a `delete` update.
    pub fn also_delete<S: Into<NodeSelector>>(
        mut self,
        selectors: impl IntoIterator<Item = S>,
    ) -> Self {
        self.delete.extend(selectors.into_iter().map(Into::into));
        self
    }

    /// Applies every rule whose selector matches the current path, in order.
    fn run_rules(&mut self, mut node: Node) -> Node {
        // Disjoint field borrows let the rule closures read the path while
        // `rules` is borrowed mutably -- no need to clone the path per node.
        let Self { rules, path, .. } = self;
        for rule in rules.iter_mut() {
            if rule.selector.matches(path) {
                node = (rule.transform)(node, path);
            }
        }
        node
    }

    /// Marks the current node (path tail) for deletion if it matches a delete
    /// selector. The parent fold removes it via [`TransformVisitor::take_deleted`].
    fn mark_if_deleted(&mut self) {
        if self
            .delete
            .iter()
            .any(|selector| selector.matches(&self.path))
        {
            self.pending_delete = true;
        }
    }

    /// Visits one node: pushes its descriptor, then applies rules and recurses
    /// into children in the configured order (bottom-up vs top-down), recovers
    /// the typed node, optionally marks it for deletion, and pops the path.
    ///
    /// `fold` recurses into children, `wrap`/`unwrap` bridge the typed node and
    /// the [`Node`] union that rules operate on (a kind-changing transform that
    /// `unwrap` cannot recover falls back to the pre-rule node).
    #[allow(clippy::too_many_arguments)]
    fn apply<N: Clone>(
        &mut self,
        descriptor: NodeDescriptor,
        node: N,
        deletable: bool,
        fold: fn(&mut Self, N) -> N,
        wrap: fn(N) -> Node,
        unwrap: fn(Node) -> Option<N>,
    ) -> N {
        self.path.push(descriptor);
        let result = if self.top_down {
            let node = unwrap(self.run_rules(wrap(node.clone()))).unwrap_or(node);
            fold(self, node)
        } else {
            let node = fold(self, node);
            unwrap(self.run_rules(wrap(node.clone()))).unwrap_or(node)
        };
        if deletable {
            self.mark_if_deleted();
        }
        self.path.pop();
        result
    }
}

impl TransformVisitor for BottomUpTransformer {
    fn take_deleted(&mut self) -> bool {
        std::mem::take(&mut self.pending_delete)
    }

    fn visit_program_link(&mut self, node: ProgramLinkNode) -> ProgramLinkNode {
        // A leaf link node; an apply target so transforms can rewrite program
        // references (e.g. on rename). Link nodes are not deletable.
        let descriptor = NodeDescriptor::named("programLinkNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            false,
            |_, n| n,
            |n| Node::Link(LinkNode::Program(n)),
            |node| match node {
                Node::Link(LinkNode::Program(n)) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_defined_type_link(&mut self, node: DefinedTypeLinkNode) -> DefinedTypeLinkNode {
        let descriptor = NodeDescriptor::named("definedTypeLinkNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            false,
            fold_defined_type_link,
            |n| Node::Link(LinkNode::DefinedType(n)),
            |node| match node {
                Node::Link(LinkNode::DefinedType(n)) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_account_link(&mut self, node: AccountLinkNode) -> AccountLinkNode {
        let descriptor = NodeDescriptor::named("accountLinkNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            false,
            fold_account_link,
            |n| Node::Link(LinkNode::Account(n)),
            |node| match node {
                Node::Link(LinkNode::Account(n)) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_pda_link(&mut self, node: PdaLinkNode) -> PdaLinkNode {
        let descriptor = NodeDescriptor::named("pdaLinkNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            false,
            fold_pda_link,
            |n| Node::Link(LinkNode::Pda(n)),
            |node| match node {
                Node::Link(LinkNode::Pda(n)) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        // Type nodes may change kind, so recover through `TryFrom<Node>`.
        let descriptor = NodeDescriptor::kinded(node.kind());
        self.apply(
            descriptor,
            node,
            false,
            fold_type_node,
            |n| n.into(),
            |node| TypeNode::try_from(node).ok(),
        )
    }

    fn visit_root(&mut self, node: RootNode) -> RootNode {
        self.apply(
            NodeDescriptor::kinded("rootNode"),
            node,
            false,
            fold_root,
            |n| n.into(),
            |node| match node {
                Node::Root(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_program(&mut self, node: ProgramNode) -> ProgramNode {
        let descriptor = NodeDescriptor::named("programNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_program,
            |n| n.into(),
            |node| match node {
                Node::Program(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_account(&mut self, node: AccountNode) -> AccountNode {
        let descriptor = NodeDescriptor::named("accountNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_account,
            |n| n.into(),
            |node| match node {
                Node::Account(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_instruction(&mut self, node: InstructionNode) -> InstructionNode {
        let descriptor = NodeDescriptor::named("instructionNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_instruction,
            |n| n.into(),
            |node| match node {
                Node::Instruction(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_instruction_account(
        &mut self,
        node: InstructionAccountNode,
    ) -> InstructionAccountNode {
        let descriptor = NodeDescriptor::named("instructionAccountNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_instruction_account,
            |n| n.into(),
            |node| match node {
                Node::InstructionAccount(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_instruction_argument(
        &mut self,
        node: InstructionArgumentNode,
    ) -> InstructionArgumentNode {
        let descriptor = NodeDescriptor::named("instructionArgumentNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_instruction_argument,
            |n| n.into(),
            |node| match node {
                Node::InstructionArgument(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_pda(&mut self, node: PdaNode) -> PdaNode {
        let descriptor = NodeDescriptor::named("pdaNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_pda,
            |n| n.into(),
            |node| match node {
                Node::Pda(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_defined_type(&mut self, node: DefinedTypeNode) -> DefinedTypeNode {
        let descriptor = NodeDescriptor::named("definedTypeNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_defined_type,
            |n| n.into(),
            |node| match node {
                Node::DefinedType(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_constant(&mut self, node: ConstantNode) -> ConstantNode {
        let descriptor = NodeDescriptor::named("constantNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_constant,
            |n| n.into(),
            |node| match node {
                Node::Constant(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_error(&mut self, node: ErrorNode) -> ErrorNode {
        // `ErrorNode` is a leaf, so there is nothing to recurse into.
        let descriptor = NodeDescriptor::named("errorNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            |_, n| n,
            |n| n.into(),
            |node| match node {
                Node::Error(n) => Some(n),
                _ => None,
            },
        )
    }

    fn visit_event(&mut self, node: EventNode) -> EventNode {
        let descriptor = NodeDescriptor::named("eventNode", node.name.clone());
        self.apply(
            descriptor,
            node,
            true,
            fold_event,
            |n| n.into(),
            |node| match node {
                Node::Event(n) => Some(n),
                _ => None,
            },
        )
    }
}

/// Convenience constructor mirroring the upstream `bottomUpTransformerVisitor`.
pub fn bottom_up_transformer(rules: Vec<TransformRule>) -> BottomUpTransformer {
    BottomUpTransformer::new(rules)
}

/// Deletes every node matching any of the given selectors. The Rust counterpart
/// of `@codama/visitors-core`'s `deleteNodesVisitor`.
///
/// Deletion is supported for the top-level named nodes that live in a `Vec`
/// slot — accounts, instructions (and sub-instructions), defined types, errors,
/// events, PDAs, constants, and instruction accounts/arguments. Matching any
/// other kind (a type/value node, the root program, a required child) is a
/// no-op, since dropping it would orphan or invalidate its parent (upstream
/// bubbles such deletions; that is a follow-up here).
///
/// ```
/// use codama_nodes::{AccountNode, ProgramNode, RootNode, StructTypeNode};
/// use codama_visitors_core::{delete_nodes, TransformVisitor};
///
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("keep", StructTypeNode::new(vec![])));
/// program.accounts.push(AccountNode::new("drop", StructTypeNode::new(vec![])));
/// let root = RootNode::new(program);
///
/// let root = delete_nodes(["[accountNode]drop"]).visit_root(root);
/// assert_eq!(root.program.accounts.len(), 1);
/// assert_eq!(root.program.accounts[0].name.as_ref(), "keep");
/// ```
pub fn delete_nodes<S: Into<NodeSelector>>(
    selectors: impl IntoIterator<Item = S>,
) -> BottomUpTransformer {
    BottomUpTransformer {
        rules: Vec::new(),
        delete: selectors.into_iter().map(Into::into).collect(),
        path: NodePath::new(),
        pending_delete: false,
        top_down: false,
    }
}

/// Like [`bottom_up_transformer`] but applies rules top-down: a node is
/// transformed *before* its children are visited. The Rust counterpart of
/// `@codama/visitors-core`'s `topDownTransformerVisitor`. (Returns the same
/// [`BottomUpTransformer`] type, configured for top-down traversal.)
pub fn top_down_transformer(rules: Vec<TransformRule>) -> BottomUpTransformer {
    let mut transformer = BottomUpTransformer::new(rules);
    transformer.top_down = true;
    transformer
}
