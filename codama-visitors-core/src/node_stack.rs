use crate::NodePath;
use codama_nodes::NodeRef;
use std::{cell::RefCell, fmt};

/// Records the path of the node being visited, as the
/// [`VisitorExt::record_node_stack`](crate::VisitorExt::record_node_stack) adapter pushes each node
/// before visiting it and pops it afterwards.
///
/// The stack holds a list of paths, where the last one is the current path. A visitor jumps to
/// another part of the tree, such as the node a link points to, with [`NodeStack::visit_path`] or
/// [`NodeStack::with_path`], which restore the previous path afterwards.
///
/// Its methods take `&self`, so visitors and their extensions can share it by reference.
///
/// ```
/// use codama_nodes::{IntegerTypeNode, NodeRef, StructFieldTypeNode, U8};
/// use codama_visitors_core::{visit, NodeStack, VisitorExt, VoidVisitor};
///
/// // The tree outlives the stack, which holds references to its nodes.
/// let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
/// let stack = NodeStack::default();
/// let visitor = VoidVisitor
///     .tap(|node| {
///         if let NodeRef::IntegerType(_) = node {
///             assert_eq!(stack.to_string(), "[structFieldTypeNode]amount.[integerTypeNode]");
///         }
///     })
///     .record_node_stack(&stack);
///
/// visit(&visitor, &field);
/// assert!(stack.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct NodeStack<'a> {
    /// Never empty: the last path is the current one.
    paths: RefCell<Vec<NodePath<'a>>>,
}

impl Default for NodeStack<'_> {
    fn default() -> Self {
        Self::new(NodePath::default())
    }
}

impl<'a> NodeStack<'a> {
    /// A stack whose current path is `path`.
    pub fn new(path: NodePath<'a>) -> Self {
        Self {
            paths: RefCell::new(vec![path]),
        }
    }

    /// Adds `node` at the end of the current path.
    pub fn push(&self, node: impl Into<NodeRef<'a>>) {
        self.with_current_path(|path| path.push(node));
    }

    /// Removes the last node of the current path and returns it.
    pub fn pop(&self) -> Option<NodeRef<'a>> {
        self.with_current_path(NodePath::pop)
    }

    /// The last node of the current path.
    pub fn peek(&self) -> Option<NodeRef<'a>> {
        self.with_current_path(|path| path.last())
    }

    /// Makes `path` the current path, until [`NodeStack::pop_path`] restores the previous one.
    pub fn push_path(&self, path: NodePath<'a>) {
        self.paths.borrow_mut().push(path);
    }

    /// Removes the current path and returns it, which makes the previous path current again.
    ///
    /// # Panics
    ///
    /// Panics when the current path is the only one, as a stack always has a current path.
    /// It means a `pop_path` has no matching [`NodeStack::push_path`].
    pub fn pop_path(&self) -> NodePath<'a> {
        let mut paths = self.paths.borrow_mut();
        assert!(
            paths.len() > 1,
            "Cannot pop the last path of a node stack: each pop_path needs a matching push_path"
        );
        paths.pop().expect("the stack has more than one path")
    }

    /// Runs `f` with `path` as the current path, then restores the previous path, even if `f` panics.
    pub fn with_path<T>(&self, path: NodePath<'a>, f: impl FnOnce() -> T) -> T {
        self.push_path(path);
        let _restore = PopPathOnDrop(self);
        f()
    }

    /// Visits the last node of `path` with `visit`, with the rest of `path` as the current path, then
    /// restores the previous path, like `visitPath` in the JavaScript library.
    ///
    /// The visit must record its nodes on this stack, as
    /// [`VisitorExt::record_node_stack`](crate::VisitorExt::record_node_stack) does, which adds the
    /// visited node back to the path. Inside an extension, pass `|node| scope.visit(node)`:
    ///
    /// ```
    /// # use codama_nodes::NodeRef;
    /// # use codama_visitors_core::{LinkableDictionary, MergeVisitor, NodeStack, VisitorExt};
    /// # let stack = NodeStack::default();
    /// # let linkables = LinkableDictionary::default();
    /// let visitor = MergeVisitor::new(|_, values: Vec<usize>| 1 + values.iter().sum::<usize>())
    ///     .extend(|node, scope| match node {
    ///         // Counts the nodes of the defined type that a link points to, instead of the link.
    ///         NodeRef::DefinedTypeLink(_) => match linkables.get_path(&stack.get_path()) {
    ///             Some(linked) => stack.visit_path(linked, |linked| scope.visit(linked)),
    ///             None => 0,
    ///         },
    ///         node => scope.next(node),
    ///     })
    ///     .record_node_stack(&stack)
    ///     .record_linkables_on_first_visit(&linkables);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics when `path` is empty, as there is no node to visit.
    pub fn visit_path<T>(&self, mut path: NodePath<'a>, visit: impl FnOnce(NodeRef<'a>) -> T) -> T {
        let node = path.pop().expect("Cannot visit an empty node path");
        self.with_path(path, || visit(node))
    }

    /// A copy of the current path.
    pub fn get_path(&self) -> NodePath<'a> {
        self.with_current_path(|path| path.clone())
    }

    /// Whether the current path has no nodes.
    pub fn is_empty(&self) -> bool {
        self.with_current_path(|path| path.is_empty())
    }

    fn with_current_path<R>(&self, f: impl FnOnce(&mut NodePath<'a>) -> R) -> R {
        let mut paths = self.paths.borrow_mut();
        f(paths.last_mut().expect("a node stack always has a path"))
    }
}

/// Pops the current path when dropped, so a path pushed by [`NodeStack::with_path`] is popped even
/// when its function panics.
struct PopPathOnDrop<'s, 'a>(&'s NodeStack<'a>);

impl Drop for PopPathOnDrop<'_, '_> {
    fn drop(&mut self) {
        self.0.pop_path();
    }
}

impl fmt::Display for NodeStack<'_> {
    /// Displays the current path.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get_path().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{InstructionNode, ProgramNode};

    #[test]
    fn push_peek_and_pop() {
        let program = ProgramNode::default();
        let instruction = InstructionNode::default();
        let stack = NodeStack::default();
        assert!(stack.is_empty());
        stack.push(&program);
        stack.push(&instruction);
        assert_eq!(stack.peek(), Some(NodeRef::from(&instruction)));
        assert_eq!(stack.pop(), Some(NodeRef::from(&instruction)));
        assert_eq!(
            stack.get_path(),
            NodePath::from(vec![NodeRef::from(&program)])
        );
        assert_eq!(stack.pop(), Some(NodeRef::from(&program)));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_path_and_pop_path() {
        let program = ProgramNode::default();
        let instruction = InstructionNode::default();
        let stack = NodeStack::default();
        stack.push(&program);

        let other = NodePath::from(vec![NodeRef::from(&instruction)]);
        stack.push_path(other.clone());
        assert_eq!(stack.get_path(), other);
        stack.pop();
        assert!(stack.is_empty());

        assert_eq!(stack.pop_path(), NodePath::default());
        assert_eq!(
            stack.get_path(),
            NodePath::from(vec![NodeRef::from(&program)])
        );
    }

    #[test]
    fn with_path_restores_the_previous_path() {
        let program = ProgramNode::default();
        let instruction = InstructionNode::default();
        let stack = NodeStack::default();
        stack.push(&program);
        let inner = stack.with_path(NodePath::from(vec![NodeRef::from(&instruction)]), || {
            stack.get_path()
        });
        assert_eq!(inner, NodePath::from(vec![NodeRef::from(&instruction)]));
        assert_eq!(
            stack.get_path(),
            NodePath::from(vec![NodeRef::from(&program)])
        );
    }

    #[test]
    fn with_path_restores_the_previous_path_on_panic() {
        let instruction = InstructionNode::default();
        let stack = NodeStack::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            stack.with_path(NodePath::from(vec![NodeRef::from(&instruction)]), || {
                panic!("visit failed")
            })
        }));
        assert!(result.is_err());
        assert!(stack.is_empty());
    }

    #[test]
    fn visit_path_visits_the_last_node_from_the_rest_of_the_path() {
        let program = ProgramNode::default();
        let instruction = InstructionNode::default();
        let stack = NodeStack::default();
        let path = NodePath::from(vec![NodeRef::from(&program), NodeRef::from(&instruction)]);
        let (visited, current) = stack.visit_path(path, |node| (node, stack.get_path()));
        assert_eq!(visited, NodeRef::from(&instruction));
        assert_eq!(current, NodePath::from(vec![NodeRef::from(&program)]));
        assert!(stack.is_empty());
    }

    #[test]
    #[should_panic(expected = "Cannot pop the last path of a node stack")]
    fn pop_path_panics_on_the_last_path() {
        NodeStack::default().pop_path();
    }
}
