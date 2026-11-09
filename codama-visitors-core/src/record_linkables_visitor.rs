use crate::{visit, LinkableDictionary, NodeStack, Visitor, VisitorExt, VoidVisitor};
use codama_nodes::NodeRef;
use std::cell::Cell;

/// Records every node of a tree that links can point to into a [`LinkableDictionary`].
///
/// Most visitors rather use [`VisitorExt::record_linkables_on_first_visit`], which records the whole
/// tree before visiting it, so the links of any node can be resolved during the visit.
#[derive(Debug, Clone, Copy)]
pub struct RecordLinkablesVisitor<'l, 'a> {
    linkables: &'l LinkableDictionary<'a>,
}

impl<'l, 'a> RecordLinkablesVisitor<'l, 'a> {
    /// Records into `linkables`.
    pub fn new(linkables: &'l LinkableDictionary<'a>) -> Self {
        Self { linkables }
    }
}

impl<'a> Visitor<'a, ()> for RecordLinkablesVisitor<'_, 'a> {
    fn visit(&self, node: NodeRef<'a>, _recurse: &dyn Fn(NodeRef<'a>)) {
        let stack = NodeStack::default();
        let visitor = VoidVisitor
            .tap(|_| self.linkables.record_path(&stack.get_path()))
            .record_node_stack(&stack);
        visit(&visitor, node);
    }
}

/// Wraps a visitor so that the first node it visits records the whole tree below it into a
/// [`LinkableDictionary`] before being visited.
///
/// Created by [`VisitorExt::record_linkables_on_first_visit`]. The dictionary is then complete during
/// the whole visit, so the extensions of the wrapped visitor can resolve any link. Once the visit of
/// that first node is over, the next visit records its tree again.
#[derive(Debug, Clone)]
pub struct RecordLinkablesOnFirstVisitVisitor<'l, 'a, V> {
    visitor: V,
    linkables: &'l LinkableDictionary<'a>,
    is_first_visit: Cell<bool>,
}

impl<'l, 'a, V> RecordLinkablesOnFirstVisitVisitor<'l, 'a, V> {
    /// Wraps `visitor` to record into `linkables`.
    /// Prefer [`VisitorExt::record_linkables_on_first_visit`].
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
        visit(&RecordLinkablesVisitor::new(self.linkables), node);
        let value = self.visitor.visit(node, recurse);
        self.is_first_visit.set(true);
        value
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
            ["[rootNode] > [programNode]token > [definedTypeNode]config"]
        );
    }
}
