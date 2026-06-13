use codama_nodes::CamelCaseString;
use std::fmt;

/// The identifying information of a node as seen while traversing a tree: its
/// `kind` and, when it has one, its `name`.
///
/// A full node carries much more, but node *selection* only ever looks at the
/// kind and name (mirroring the upstream `NodeSelector`), so a [`NodePath`]
/// keeps these lightweight descriptors instead of cloning whole nodes as it
/// descends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeDescriptor {
    pub kind: &'static str,
    pub name: Option<CamelCaseString>,
}

impl NodeDescriptor {
    /// A descriptor for a node that has no name (e.g. a `numberTypeNode`).
    pub fn kinded(kind: &'static str) -> Self {
        Self { kind, name: None }
    }

    /// A descriptor for a named node (e.g. an `accountNode`).
    pub fn named(kind: &'static str, name: impl Into<CamelCaseString>) -> Self {
        Self {
            kind,
            name: Some(name.into()),
        }
    }
}

/// The ancestry of the node currently being visited, from the root down to (and
/// including) the node itself, which is always the last element.
///
/// This is the Rust counterpart of the TypeScript `NodePath`. It is consumed by
/// [`crate::NodeSelector`] and grown/shrunk by a traversal as it enters and
/// leaves nodes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NodePath(Vec<NodeDescriptor>);

impl NodePath {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, descriptor: NodeDescriptor) {
        self.0.push(descriptor);
    }

    pub fn pop(&mut self) -> Option<NodeDescriptor> {
        self.0.pop()
    }

    pub fn descriptors(&self) -> &[NodeDescriptor] {
        &self.0
    }

    /// The node currently being visited (the tail of the path).
    pub fn current(&self) -> Option<&NodeDescriptor> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The nearest ancestor (or the current node) of the given kind, searching
    /// from the current node upward.
    pub fn find_last_by_kind(&self, kind: &str) -> Option<&NodeDescriptor> {
        self.0.iter().rev().find(|d| d.kind == kind)
    }

    /// The outermost node of the given kind, searching from the root down.
    pub fn find_first_by_kind(&self, kind: &str) -> Option<&NodeDescriptor> {
        self.0.iter().find(|d| d.kind == kind)
    }

    /// The enclosing `programNode`, if any (the Rust counterpart of
    /// `findProgramNodeFromPath`).
    pub fn program(&self) -> Option<&NodeDescriptor> {
        self.find_last_by_kind("programNode")
    }

    /// The enclosing `instructionNode`, if any.
    pub fn instruction(&self) -> Option<&NodeDescriptor> {
        self.find_last_by_kind("instructionNode")
    }
}

impl From<Vec<NodeDescriptor>> for NodePath {
    fn from(descriptors: Vec<NodeDescriptor>) -> Self {
        Self(descriptors)
    }
}

impl FromIterator<NodeDescriptor> for NodePath {
    fn from_iter<I: IntoIterator<Item = NodeDescriptor>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl fmt::Display for NodePath {
    /// Renders the path as `[kindA]name > [kindB] > ...`, matching the upstream
    /// `nodePathToString` format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, d) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str(" > ")?;
            }
            match &d.name {
                Some(name) => write!(f, "[{}]{}", d.kind, name.as_ref())?,
                None => write!(f, "[{}]", d.kind)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> NodePath {
        NodePath::from(vec![
            NodeDescriptor::named("programNode", "myProgram"),
            NodeDescriptor::named("accountNode", "myAccount"),
            NodeDescriptor::kinded("structTypeNode"),
            NodeDescriptor::named("structFieldTypeNode", "amount"),
            NodeDescriptor::kinded("numberTypeNode"),
        ])
    }

    #[test]
    fn current_is_the_tail() {
        assert_eq!(sample().current().unwrap().kind, "numberTypeNode");
    }

    #[test]
    fn find_by_kind_searches_in_both_directions() {
        let path = sample();
        assert_eq!(
            path.find_last_by_kind("structFieldTypeNode").unwrap().name,
            Some("amount".into())
        );
        assert!(path.find_last_by_kind("eventNode").is_none());
    }

    #[test]
    fn program_and_instruction_helpers() {
        let path = sample();
        assert_eq!(path.program().unwrap().name, Some("myProgram".into()));
        assert!(path.instruction().is_none());
    }

    #[test]
    fn push_and_pop() {
        let mut path = NodePath::new();
        path.push(NodeDescriptor::named("programNode", "p"));
        path.push(NodeDescriptor::kinded("numberTypeNode"));
        assert_eq!(path.len(), 2);
        assert_eq!(path.pop().unwrap().kind, "numberTypeNode");
        assert_eq!(path.len(), 1);
    }

    #[test]
    fn display() {
        assert_eq!(
            sample().to_string(),
            "[programNode]myProgram > [accountNode]myAccount > [structTypeNode] > [structFieldTypeNode]amount > [numberTypeNode]"
        );
    }
}
