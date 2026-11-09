use crate::{HasIdentifier, HasKind, Identifier};
use std::fmt;

/// The step from a node to one of its children: the spec name of the attribute, plus the index for a list item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathSegment {
    /// A child that the attribute holds directly, such as `data`.
    Attribute(&'static str),
    /// A child at an index of a list attribute, such as `accounts[0]`.
    Item(&'static str, usize),
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathSegment::Attribute(attribute) => write!(f, "{attribute}"),
            PathSegment::Item(attribute, index) => write!(f, "{attribute}[{index}]"),
        }
    }
}

/// One step of a path: the segment from the parent, and the kind and identifier of the node it leads to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathStep<'a> {
    pub segment: PathSegment,
    pub kind: &'static str,
    /// The identifier of a node kind that implements `HasIdentifier`.
    pub identifier: Option<&'a Identifier>,
}

/// The steps of a walk from its start node to the visited node.
/// The start node is not a step, so the path is empty at the start node and ends with the visited node elsewhere.
/// The kinds and identifiers of the steps let a check find an enclosing node, such as the closest program of a link.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodePath<'a>(Vec<PathStep<'a>>);

impl<'a> NodePath<'a> {
    /// A new path with one more step, to a node of a kind without an identifier.
    pub fn join(&self, segment: PathSegment, node: &impl HasKind) -> Self {
        self.with(PathStep {
            segment,
            kind: node.kind(),
            identifier: None,
        })
    }

    /// A new path with one more step, to a node of a kind with an identifier.
    pub fn join_named(
        &self,
        segment: PathSegment,
        node: &'a (impl HasKind + HasIdentifier),
    ) -> Self {
        self.with(PathStep {
            segment,
            kind: node.kind(),
            identifier: Some(node.identifier()),
        })
    }

    fn with(&self, step: PathStep<'a>) -> Self {
        let mut steps = self.0.clone();
        steps.push(step);
        Self(steps)
    }

    pub fn steps(&self) -> &[PathStep<'a>] {
        &self.0
    }
}

/// Joins the steps with ` > `, such as `program "token" > instruction "transfer" > data`.
/// A step to a node with an identifier shows the node kind without its `Node` suffix and the identifier.
/// Any other step shows its segment.
impl fmt::Display for NodePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, step) in self.0.iter().enumerate() {
            if position > 0 {
                f.write_str(" > ")?;
            }
            match step.identifier {
                Some(identifier) => {
                    let label = step.kind.strip_suffix("Node").unwrap_or(step.kind);
                    write!(f, "{label} \"{identifier}\"")?;
                }
                None => write!(f, "{}", step.segment)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::id;
    use crate::{DefinedTypeNode, FixedSizeTransformNode, ProgramNode, StringTypeNode};
    use PathSegment::{Attribute, Item};

    #[test]
    fn to_string() {
        let program = ProgramNode::new(id("token"), "");
        let defined_type = DefinedTypeNode::new(id("config"), StringTypeNode::utf8());
        let path = NodePath::default()
            .join_named(Attribute("program"), &program)
            .join_named(Item("definedTypes", 0), &defined_type)
            .join(Attribute("type"), &StringTypeNode::utf8())
            .join(Item("transforms", 1), &FixedSizeTransformNode::new(4));
        assert_eq!(
            path.to_string(),
            "program \"token\" > definedType \"config\" > type > transforms[1]"
        );
    }
}
