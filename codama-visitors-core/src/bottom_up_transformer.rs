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
/// an `enumTypeNode`'s size); and top-down traversal.
pub struct BottomUpTransformer {
    rules: Vec<TransformRule>,
    delete: Vec<NodeSelector>,
    path: NodePath,
    pending_delete: bool,
}

impl BottomUpTransformer {
    pub fn new(rules: Vec<TransformRule>) -> Self {
        Self {
            rules,
            delete: Vec::new(),
            path: NodePath::new(),
            pending_delete: false,
        }
    }

    /// Applies every rule whose selector matches the current path, in order.
    fn run_rules(&mut self, mut node: Node) -> Node {
        // Clone the path so the rule closures (which borrow `&mut self.rules`)
        // can also read it without a borrow conflict.
        let path = self.path.clone();
        for rule in self.rules.iter_mut() {
            if rule.selector.matches(&path) {
                node = (rule.transform)(node, &path);
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
}

impl TransformVisitor for BottomUpTransformer {
    fn take_deleted(&mut self) -> bool {
        std::mem::take(&mut self.pending_delete)
    }

    fn visit_program_link(&mut self, node: ProgramLinkNode) -> ProgramLinkNode {
        // A leaf link node; an apply target so transforms can rewrite program
        // references (e.g. on rename). Link nodes are not deletable.
        self.path
            .push(NodeDescriptor::named("programLinkNode", node.name.clone()));
        let result = match self.run_rules(Node::Link(LinkNode::Program(node.clone()))) {
            Node::Link(LinkNode::Program(n)) => n,
            _ => node,
        };
        self.path.pop();
        result
    }

    fn visit_defined_type_link(&mut self, node: DefinedTypeLinkNode) -> DefinedTypeLinkNode {
        self.path.push(NodeDescriptor::named(
            "definedTypeLinkNode",
            node.name.clone(),
        ));
        let node = fold_defined_type_link(self, node);
        let result = match self.run_rules(Node::Link(LinkNode::DefinedType(node.clone()))) {
            Node::Link(LinkNode::DefinedType(n)) => n,
            _ => node,
        };
        self.path.pop();
        result
    }

    fn visit_account_link(&mut self, node: AccountLinkNode) -> AccountLinkNode {
        self.path
            .push(NodeDescriptor::named("accountLinkNode", node.name.clone()));
        let node = fold_account_link(self, node);
        let result = match self.run_rules(Node::Link(LinkNode::Account(node.clone()))) {
            Node::Link(LinkNode::Account(n)) => n,
            _ => node,
        };
        self.path.pop();
        result
    }

    fn visit_pda_link(&mut self, node: PdaLinkNode) -> PdaLinkNode {
        self.path
            .push(NodeDescriptor::named("pdaLinkNode", node.name.clone()));
        let node = fold_pda_link(self, node);
        let result = match self.run_rules(Node::Link(LinkNode::Pda(node.clone()))) {
            Node::Link(LinkNode::Pda(n)) => n,
            _ => node,
        };
        self.path.pop();
        result
    }

    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        self.path.push(NodeDescriptor::kinded(node.kind()));
        let node = fold_type_node(self, node);
        // Type nodes may change kind, so recover through `TryFrom<Node>`.
        let result = TypeNode::try_from(self.run_rules(node.clone().into())).unwrap_or(node);
        self.path.pop();
        result
    }

    fn visit_root(&mut self, node: RootNode) -> RootNode {
        self.path.push(NodeDescriptor::kinded("rootNode"));
        let node = fold_root(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Root(n) => n,
            _ => node,
        };
        self.path.pop();
        result
    }

    fn visit_program(&mut self, node: ProgramNode) -> ProgramNode {
        self.path
            .push(NodeDescriptor::named("programNode", node.name.clone()));
        let node = fold_program(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Program(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_account(&mut self, node: AccountNode) -> AccountNode {
        self.path
            .push(NodeDescriptor::named("accountNode", node.name.clone()));
        let node = fold_account(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Account(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_instruction(&mut self, node: InstructionNode) -> InstructionNode {
        self.path
            .push(NodeDescriptor::named("instructionNode", node.name.clone()));
        let node = fold_instruction(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Instruction(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_instruction_account(
        &mut self,
        node: InstructionAccountNode,
    ) -> InstructionAccountNode {
        self.path.push(NodeDescriptor::named(
            "instructionAccountNode",
            node.name.clone(),
        ));
        let node = fold_instruction_account(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::InstructionAccount(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_instruction_argument(
        &mut self,
        node: InstructionArgumentNode,
    ) -> InstructionArgumentNode {
        self.path.push(NodeDescriptor::named(
            "instructionArgumentNode",
            node.name.clone(),
        ));
        let node = fold_instruction_argument(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::InstructionArgument(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_pda(&mut self, node: PdaNode) -> PdaNode {
        self.path
            .push(NodeDescriptor::named("pdaNode", node.name.clone()));
        let node = fold_pda(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Pda(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_defined_type(&mut self, node: DefinedTypeNode) -> DefinedTypeNode {
        self.path
            .push(NodeDescriptor::named("definedTypeNode", node.name.clone()));
        let node = fold_defined_type(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::DefinedType(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_constant(&mut self, node: ConstantNode) -> ConstantNode {
        self.path
            .push(NodeDescriptor::named("constantNode", node.name.clone()));
        let node = fold_constant(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Constant(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_error(&mut self, node: ErrorNode) -> ErrorNode {
        // `ErrorNode` is a leaf, so there is nothing to recurse into.
        self.path
            .push(NodeDescriptor::named("errorNode", node.name.clone()));
        let result = match self.run_rules(node.clone().into()) {
            Node::Error(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
    }

    fn visit_event(&mut self, node: EventNode) -> EventNode {
        self.path
            .push(NodeDescriptor::named("eventNode", node.name.clone()));
        let node = fold_event(self, node);
        let result = match self.run_rules(node.clone().into()) {
            Node::Event(n) => n,
            _ => node,
        };
        self.mark_if_deleted();
        self.path.pop();
        result
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
    }
}
