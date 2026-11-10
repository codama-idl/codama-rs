use crate::{record_node_stack_visitor::PopOnDrop, visit, LinkableDictionary, NodeStack, Visitor};
use codama_nodes::NodeRef;
use std::cell::Cell;

/// Records every node of a tree that links can point to into a [`LinkableDictionary`].
///
/// Most visitors rather use [`VisitorExt::record_linkables_on_first_visit`](crate::VisitorExt::record_linkables_on_first_visit), which records the whole
/// tree before visiting it, so the links of any node can be resolved during the visit. To record a tree
/// once and share the dictionary, use [`get_linkables_from_node`].
///
/// It keeps the path of the visited node in its own [`NodeStack`] and visits children through
/// `recurse`, so it composes with adapters like any other visitor.
#[derive(Debug, Clone)]
pub struct RecordLinkablesVisitor<'l, 'a> {
    linkables: &'l LinkableDictionary<'a>,
    stack: NodeStack<'a>,
}

impl<'l, 'a> RecordLinkablesVisitor<'l, 'a> {
    /// Records into `linkables`.
    pub fn new(linkables: &'l LinkableDictionary<'a>) -> Self {
        Self {
            linkables,
            stack: NodeStack::default(),
        }
    }
}

impl<'a> Visitor<'a, ()> for RecordLinkablesVisitor<'_, 'a> {
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>)) {
        self.stack.push(node);
        let _pop = PopOnDrop(&self.stack);
        self.stack
            .with_current_path(|path| self.linkables.record_path(path));
        node.children().into_iter().for_each(recurse);
    }
}

/// Records the nodes of the tree under `node`, such as a root node, that links can point to into a new
/// [`LinkableDictionary`], like `getLinkablesFromNode` in the JavaScript library.
pub fn get_linkables_from_node<'a>(node: impl Into<NodeRef<'a>>) -> LinkableDictionary<'a> {
    let linkables = LinkableDictionary::default();
    visit(&RecordLinkablesVisitor::new(&linkables), node);
    linkables
}

/// Wraps a visitor so that the first node it visits records the whole tree below it into a
/// [`LinkableDictionary`] before being visited.
///
/// Created by [`VisitorExt::record_linkables_on_first_visit`](crate::VisitorExt::record_linkables_on_first_visit). The dictionary is then complete during
/// the whole visit, so the extensions of the wrapped visitor can resolve any link. The dictionary is
/// cleared before recording, so a visitor used on several trees only resolves the links of the tree
/// it is visiting.
#[derive(Debug, Clone)]
pub struct RecordLinkablesOnFirstVisitVisitor<'l, 'a, V> {
    visitor: V,
    linkables: &'l LinkableDictionary<'a>,
    is_first_visit: Cell<bool>,
}

impl<'l, 'a, V> RecordLinkablesOnFirstVisitVisitor<'l, 'a, V> {
    /// Wraps `visitor` to record into `linkables`.
    /// Prefer [`VisitorExt::record_linkables_on_first_visit`](crate::VisitorExt::record_linkables_on_first_visit).
    pub fn new(visitor: V, linkables: &'l LinkableDictionary<'a>) -> Self {
        Self {
            visitor,
            linkables,
            is_first_visit: Cell::new(true),
        }
    }
}

impl<'a, T, V> Visitor<'a, T> for RecordLinkablesOnFirstVisitVisitor<'_, 'a, V>
where
    V: Visitor<'a, T>,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        if !self.is_first_visit.replace(false) {
            return self.visitor.visit(node, recurse);
        }
        let _rearm = RearmOnDrop(&self.is_first_visit);
        self.linkables.clear();
        visit(&RecordLinkablesVisitor::new(self.linkables), node);
        self.visitor.visit(node, recurse)
    }
}

/// Marks the next visit as a first visit when dropped, even when the current visit panics.
struct RearmOnDrop<'c>(&'c Cell<bool>);

impl Drop for RearmOnDrop<'_> {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, LinkableDictionary, NodeStack, VisitorExt, VoidVisitor};
    use codama_nodes::{
        DefinedTypeLinkNode, DefinedTypeNode, ProgramNode, RootNode, StructFieldTypeNode,
        StructTypeNode,
    };
    use std::cell::RefCell;

    #[test]
    fn links_resolve_before_their_linked_node_is_visited() {
        // The field links to `config`, which comes after it in the program.
        let root = RootNode::new(ProgramNode {
            defined_types: vec![
                DefinedTypeNode::new(
                    "wrapper",
                    StructTypeNode::new(vec![StructFieldTypeNode::new(
                        "config",
                        DefinedTypeLinkNode::new("config"),
                    )]),
                ),
                DefinedTypeNode::new("config", StructTypeNode::new(vec![])),
            ],
            ..ProgramNode::new("token", "1111")
        });
        let stack = NodeStack::default();
        let linkables = LinkableDictionary::default();
        let resolved = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .tap(|node| {
                if let codama_nodes::NodeRef::DefinedTypeLink(_) = node {
                    let path = linkables.get_path_or_err(&stack.get_path()).unwrap();
                    resolved.borrow_mut().push(path.to_string());
                }
            })
            .record_node_stack(&stack)
            .record_linkables_on_first_visit(&linkables);
        visit(&visitor, &root);
        assert_eq!(
            *resolved.borrow(),
            ["[programNode]token.[definedTypeNode]config"]
        );
    }

    #[test]
    fn records_the_tree_of_each_visit_only() {
        let first = RootNode::new(ProgramNode {
            defined_types: vec![DefinedTypeNode::new("first", StructTypeNode::new(vec![]))],
            ..ProgramNode::new("token", "1111")
        });
        let second = RootNode::new(ProgramNode::new("token", "1111"));
        let linkables = LinkableDictionary::default();
        let first_link = DefinedTypeLinkNode::new("first");
        let resolved = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .tap(|node| {
                if let codama_nodes::NodeRef::Program(program) = node {
                    let path = crate::NodePath::from(vec![
                        codama_nodes::NodeRef::from(program),
                        codama_nodes::NodeRef::from(&first_link),
                    ]);
                    resolved.borrow_mut().push(linkables.has(&path));
                }
            })
            .record_linkables_on_first_visit(&linkables);
        visit(&visitor, &first);
        visit(&visitor, &second);
        assert_eq!(*resolved.borrow(), [true, false]);
    }

    #[test]
    fn get_linkables_from_node_records_a_tree() {
        let root = RootNode::new(ProgramNode {
            defined_types: vec![DefinedTypeNode::new("config", StructTypeNode::new(vec![]))],
            ..ProgramNode::new("token", "1111")
        });
        let linkables = crate::get_linkables_from_node(&root);
        let paths = linkables.get_recorded_paths_of_kind::<DefinedTypeNode>();
        assert_eq!(
            paths.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["[programNode]token.[definedTypeNode]config"]
        );
    }

    #[test]
    fn record_linkables_visitor_composes_with_adapters() {
        let root = RootNode::new(ProgramNode::new("token", "1111"));
        let linkables = LinkableDictionary::default();
        let kinds = RefCell::new(vec![]);
        let visitor = crate::RecordLinkablesVisitor::new(&linkables)
            .tap(|node| kinds.borrow_mut().push(codama_nodes::HasKind::kind(&node)));
        visit(&visitor, &root);
        assert_eq!(*kinds.borrow(), ["rootNode", "programNode"]);
        assert_eq!(
            linkables.get_recorded_paths_of_kind::<ProgramNode>().len(),
            1
        );
    }

    #[test]
    fn records_again_after_a_visit_that_panicked() {
        let root = RootNode::new(ProgramNode::new("token", "1111"));
        let linkables = LinkableDictionary::default();
        let should_panic = std::cell::Cell::new(true);
        let visitor = VoidVisitor
            .tap(|_| {
                if should_panic.replace(false) {
                    panic!("visit failed");
                }
            })
            .record_linkables_on_first_visit(&linkables);
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| visit(&visitor, &root)));
        assert!(result.is_err());
        linkables.clear();
        visit(&visitor, &root);
        assert_eq!(
            linkables.get_recorded_paths_of_kind::<ProgramNode>().len(),
            1
        );
    }
}
