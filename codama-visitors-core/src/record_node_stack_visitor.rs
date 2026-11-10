use crate::{NodeStack, Visitor};
use codama_nodes::NodeRef;

/// Wraps a visitor so that a [`NodeStack`] holds the path of the node being visited.
///
/// Created by [`VisitorExt::record_node_stack`](crate::VisitorExt::record_node_stack). Each node is
/// pushed onto the stack before the wrapped visitor visits it, and popped afterwards. Adapters that
/// wrap this one run outside of the push, so wrap a visitor with it after the extensions that read
/// the stack, as they read it while the node is on it.
#[derive(Debug, Clone, Copy)]
pub struct RecordNodeStackVisitor<'s, 'a, V> {
    visitor: V,
    stack: &'s NodeStack<'a>,
}

impl<'s, 'a, V> RecordNodeStackVisitor<'s, 'a, V> {
    /// Wraps `visitor` to record into `stack`.
    /// Prefer [`VisitorExt::record_node_stack`](crate::VisitorExt::record_node_stack).
    pub fn new(visitor: V, stack: &'s NodeStack<'a>) -> Self {
        Self { visitor, stack }
    }
}

impl<'a, T, V> Visitor<'a, T> for RecordNodeStackVisitor<'_, 'a, V>
where
    V: Visitor<'a, T>,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        self.stack.push(node);
        let _pop = PopOnDrop(self.stack);
        self.visitor.visit(node, recurse)
    }
}

/// Pops the current node when dropped, so a visited node leaves the stack even when its visit panics.
pub(crate) struct PopOnDrop<'s, 'a>(pub(crate) &'s NodeStack<'a>);

impl Drop for PopOnDrop<'_, '_> {
    fn drop(&mut self) {
        self.0.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, NodeStack, VisitorExt, VoidVisitor};
    use codama_nodes::{
        IntegerTypeNode, NodeRef, StringTypeNode, StructFieldTypeNode, StructTypeNode, U8,
    };
    use std::cell::RefCell;

    #[test]
    fn records_the_path_of_each_node() {
        let stack = NodeStack::default();
        let paths = RefCell::new(vec![]);
        let visitor = VoidVisitor
            .tap(|_| paths.borrow_mut().push(stack.to_string()))
            .record_node_stack(&stack);
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("a", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("b", StringTypeNode::utf8()),
        ]);
        visit(&visitor, &r#struct);
        assert_eq!(
            *paths.borrow(),
            [
                "[structTypeNode]",
                "[structTypeNode].[structFieldTypeNode]a",
                "[structTypeNode].[structFieldTypeNode]a.[integerTypeNode]",
                "[structTypeNode].[structFieldTypeNode]b",
                "[structTypeNode].[structFieldTypeNode]b.[stringTypeNode]",
            ]
        );
        assert!(stack.is_empty());
    }

    #[test]
    fn records_onto_the_current_path_of_the_stack() {
        let parent = StructTypeNode::new(vec![]);
        let integer = IntegerTypeNode::le(U8);
        let stack = NodeStack::default();
        stack.push(&parent);
        let last = RefCell::new(None);
        let visitor = VoidVisitor
            .tap(|node| {
                if let NodeRef::IntegerType(_) = node {
                    *last.borrow_mut() = Some(stack.to_string());
                }
            })
            .record_node_stack(&stack);
        visit(&visitor, &integer);
        assert_eq!(
            last.borrow().as_deref(),
            Some("[structTypeNode].[integerTypeNode]")
        );
        assert_eq!(stack.get_path().len(), 1);
    }
}
