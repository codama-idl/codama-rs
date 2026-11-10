use codama_nodes::{HasKind, NodeRef};
use std::fmt;

/// The nodes from the start of a visit down to a node, that node included.
///
/// A [`NodeStack`](crate::NodeStack) records the path of the node being visited, and a
/// [`LinkableDictionary`](crate::LinkableDictionary) stores the path of each node that a link can
/// point to. The path only holds references, so it is cheap to clone.
///
/// A path displays like `getNodePathString` in the JavaScript library: the kind and identifier of each
/// node, joined by `.`, without the root node when other nodes follow it, such as
/// `[programNode]token.[instructionNode]transfer`.
///
/// ```
/// use codama_nodes::{InstructionNode, NodeRef, ProgramNode};
/// use codama_visitors_core::NodePath;
///
/// let program = ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
/// let instruction = InstructionNode {
///     identifier: "transfer".into(),
///     ..InstructionNode::default()
/// };
/// let path = NodePath::from(vec![NodeRef::from(&program), NodeRef::from(&instruction)]);
///
/// assert_eq!(path.find_last::<ProgramNode>(), Some(&program));
/// assert_eq!(path.to_string(), "[programNode]token.[instructionNode]transfer");
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodePath<'a>(Vec<NodeRef<'a>>);

impl<'a> NodePath<'a> {
    /// The nodes of the path, from the first one visited to the last one.
    pub fn nodes(&self) -> &[NodeRef<'a>] {
        &self.0
    }

    /// The number of nodes in the path.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the path has no nodes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The last node of the path, which the path leads to.
    pub fn last(&self) -> Option<NodeRef<'a>> {
        self.0.last().copied()
    }

    /// The first node of the path of type `T`, such as `path.find_first::<ProgramNode>()`.
    pub fn find_first<T: 'a>(&self) -> Option<&'a T>
    where
        &'a T: TryFrom<NodeRef<'a>>,
    {
        self.0.iter().find_map(|node| (*node).try_into().ok())
    }

    /// The last node of the path of type `T`, such as `path.find_last::<InstructionNode>()`.
    pub fn find_last<T: 'a>(&self) -> Option<&'a T>
    where
        &'a T: TryFrom<NodeRef<'a>>,
    {
        self.0.iter().rev().find_map(|node| (*node).try_into().ok())
    }

    /// The path up to its last node of type `T`, that node included, or `None` without one.
    pub fn until_last<T: 'a>(&self) -> Option<NodePath<'a>>
    where
        &'a T: TryFrom<NodeRef<'a>>,
    {
        let index = self
            .0
            .iter()
            .rposition(|node| <&'a T>::try_from(*node).is_ok())?;
        Some(Self(self.0[..=index].to_vec()))
    }

    /// Adds `node` at the end of the path.
    pub fn push(&mut self, node: impl Into<NodeRef<'a>>) {
        self.0.push(node.into());
    }

    /// Removes the last node of the path and returns it.
    pub fn pop(&mut self) -> Option<NodeRef<'a>> {
        self.0.pop()
    }
}

impl<'a> From<Vec<NodeRef<'a>>> for NodePath<'a> {
    fn from(nodes: Vec<NodeRef<'a>>) -> Self {
        Self(nodes)
    }
}

impl<'a> FromIterator<NodeRef<'a>> for NodePath<'a> {
    fn from_iter<I: IntoIterator<Item = NodeRef<'a>>>(nodes: I) -> Self {
        Self(nodes.into_iter().collect())
    }
}

impl<'a> IntoIterator for NodePath<'a> {
    type Item = NodeRef<'a>;
    type IntoIter = std::vec::IntoIter<NodeRef<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for NodePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nodes = match self.0.as_slice() {
            [NodeRef::Root(_), rest @ ..] if !rest.is_empty() => rest,
            nodes => nodes,
        };
        for (index, node) in nodes.iter().enumerate() {
            if index > 0 {
                f.write_str(".")?;
            }
            write!(f, "[{}]", node.kind())?;
            if let Some(identifier) = node.identifier() {
                write!(f, "{identifier}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{
        InstructionNode, IntegerTypeNode, ProgramNode, RootNode, StructFieldTypeNode, U8,
    };

    #[test]
    fn find_first_and_last() {
        let first = ProgramNode::new("first", "1111");
        let last = ProgramNode::new("last", "2222");
        let path = NodePath::from(vec![NodeRef::from(&first), NodeRef::from(&last)]);
        assert!(std::ptr::eq(
            path.find_first::<ProgramNode>().unwrap(),
            &first
        ));
        assert!(std::ptr::eq(
            path.find_last::<ProgramNode>().unwrap(),
            &last
        ));
        assert_eq!(path.find_first::<InstructionNode>(), None);
    }

    #[test]
    fn until_last() {
        let root = RootNode::default();
        let program = ProgramNode::new("token", "1111");
        let instruction = InstructionNode {
            identifier: "transfer".into(),
            ..InstructionNode::default()
        };
        let path = NodePath::from(vec![
            NodeRef::from(&root),
            NodeRef::from(&program),
            NodeRef::from(&instruction),
        ]);
        let until_program = path.until_last::<ProgramNode>().unwrap();
        assert_eq!(
            until_program.nodes(),
            [NodeRef::from(&root), NodeRef::from(&program)]
        );
        assert_eq!(path.until_last::<StructFieldTypeNode>(), None);
    }

    #[test]
    fn push_pop_and_last() {
        let program = ProgramNode::default();
        let mut path = NodePath::default();
        assert!(path.is_empty());
        assert_eq!(path.last(), None);
        path.push(&program);
        assert_eq!(path.len(), 1);
        assert_eq!(path.last(), Some(NodeRef::from(&program)));
        assert_eq!(path.pop(), Some(NodeRef::from(&program)));
        assert!(path.is_empty());
    }

    #[test]
    fn display_shows_kinds_and_identifiers() {
        let program = ProgramNode::new("token", "1111");
        let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
        let integer = IntegerTypeNode::le(U8);
        let path = NodePath::from(vec![
            NodeRef::from(&program),
            NodeRef::from(&field),
            NodeRef::from(&integer),
        ]);
        assert_eq!(
            path.to_string(),
            "[programNode]token.[structFieldTypeNode]amount.[integerTypeNode]"
        );
    }

    #[test]
    fn display_drops_the_root_node_unless_it_is_alone() {
        let root = RootNode::default();
        let program = ProgramNode::new("token", "1111");
        let path = NodePath::from(vec![NodeRef::from(&root), NodeRef::from(&program)]);
        assert_eq!(path.to_string(), "[programNode]token");
        let path = NodePath::from(vec![NodeRef::from(&root)]);
        assert_eq!(path.to_string(), "[rootNode]");
        assert_eq!(NodePath::default().to_string(), "");
    }
}
