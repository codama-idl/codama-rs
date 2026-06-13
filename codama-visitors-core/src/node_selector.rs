use crate::{NodeDescriptor, NodePath};
use codama_nodes::CamelCaseString;

/// A string-based pattern for selecting a node within a Codama tree, ported from
/// the upstream `NodeSelectorPath` syntax:
///
/// - `*` matches any node.
/// - `someName` matches a node by its `name`, if it has one.
/// - `[someNode]` matches a node by its `kind`.
/// - `[someNode|otherNode]` matches a node of any of the listed kinds.
/// - `[someNode]someName` matches both the kind and the name.
/// - `a.b.c` matches a node `c` whose ancestry contains `a` then `b` in order
///   (not necessarily contiguous); the final segment must match the node itself.
///
/// Kinds and names are compared case-insensitively via camelCase normalisation,
/// so `[NumberTypeNode]` and `[number_type_node]` both match `numberTypeNode`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeSelector {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Segment {
    /// `*` — matches any node regardless of kind or name.
    any: bool,
    /// camelCased kinds; empty means "any kind".
    kinds: Vec<String>,
    /// camelCased name; `None` means "any name".
    name: Option<String>,
}

fn camel(s: &str) -> String {
    CamelCaseString::new(s).to_string()
}

impl Segment {
    fn parse(raw: &str) -> Self {
        if raw == "*" {
            return Self {
                any: true,
                kinds: Vec::new(),
                name: None,
            };
        }

        // Optional `[kindA|kindB]` prefix followed by an optional name.
        let (kinds, name_part) = match raw
            .strip_prefix('[')
            .and_then(|rest| rest.find(']').map(|end| (&rest[..end], &rest[end + 1..])))
        {
            Some((kinds_str, name)) => (kinds_str.split('|').map(camel).collect(), name),
            None => (Vec::new(), raw),
        };

        let name = if name_part.is_empty() {
            None
        } else {
            Some(camel(name_part))
        };

        Self {
            any: false,
            kinds,
            name,
        }
    }

    fn matches(&self, node: &NodeDescriptor) -> bool {
        if self.any {
            return true;
        }
        if !self.kinds.is_empty() && !self.kinds.iter().any(|k| k == node.kind) {
            return false;
        }
        if let Some(name) = &self.name {
            if node.name.as_ref().map(|n| n.as_ref()) != Some(name.as_str()) {
                return false;
            }
        }
        true
    }
}

impl NodeSelector {
    pub fn parse(selector: &str) -> Self {
        Self {
            segments: selector.split('.').map(Segment::parse).collect(),
        }
    }

    /// Whether this selector matches the node at the tail of `path`.
    pub fn matches(&self, path: &NodePath) -> bool {
        check_initial(path.descriptors(), &self.segments)
    }
}

impl From<&str> for NodeSelector {
    fn from(selector: &str) -> Self {
        Self::parse(selector)
    }
}

impl From<String> for NodeSelector {
    fn from(selector: String) -> Self {
        Self::parse(&selector)
    }
}

/// The current node (path tail) must match the final segment; the remaining
/// segments are then matched against its ancestors.
fn check_initial(path: &[NodeDescriptor], segments: &[Segment]) -> bool {
    match (path.split_last(), segments.split_last()) {
        (Some((node, rest_path)), Some((segment, rest_segments))) => {
            segment.matches(node) && check_ancestors(rest_path, rest_segments)
        }
        _ => false,
    }
}

/// Matches the remaining segments against ancestors, walking upward. Segments
/// must appear in order but need not be contiguous — an ancestor that doesn't
/// match the current segment is simply skipped.
fn check_ancestors(path: &[NodeDescriptor], segments: &[Segment]) -> bool {
    let Some((segment, rest_segments)) = segments.split_last() else {
        return true; // all segments consumed
    };
    let Some((node, rest_path)) = path.split_last() else {
        return false; // segments remain but no ancestors left
    };
    if segment.matches(node) {
        check_ancestors(rest_path, rest_segments)
    } else {
        check_ancestors(rest_path, segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(descriptors: Vec<NodeDescriptor>) -> NodePath {
        NodePath::from(descriptors)
    }

    #[test]
    fn wildcard_matches_any_current_node() {
        assert!(
            NodeSelector::parse("*").matches(&path(vec![NodeDescriptor::kinded("numberTypeNode")]))
        );
    }

    #[test]
    fn name_matches_current_node_name() {
        let selector = NodeSelector::parse("myAccount");
        assert!(selector.matches(&path(vec![NodeDescriptor::named(
            "accountNode",
            "myAccount"
        )])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::named("accountNode", "other")])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::kinded("numberTypeNode")])));
    }

    #[test]
    fn kind_matches_by_kind() {
        let selector = NodeSelector::parse("[numberTypeNode]");
        assert!(selector.matches(&path(vec![NodeDescriptor::kinded("numberTypeNode")])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::kinded("stringTypeNode")])));
    }

    #[test]
    fn kind_union_matches_any_listed_kind() {
        let selector = NodeSelector::parse("[accountNode|definedTypeNode]");
        assert!(selector.matches(&path(vec![NodeDescriptor::named("accountNode", "a")])));
        assert!(selector.matches(&path(vec![NodeDescriptor::named("definedTypeNode", "d")])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::named("eventNode", "e")])));
    }

    #[test]
    fn kind_and_name_together() {
        let selector = NodeSelector::parse("[accountNode]token");
        assert!(selector.matches(&path(vec![NodeDescriptor::named("accountNode", "token")])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::named("accountNode", "mint")])));
        assert!(!selector.matches(&path(vec![NodeDescriptor::named("pdaNode", "token")])));
    }

    #[test]
    fn camel_case_normalisation() {
        let selector = NodeSelector::parse("[NumberTypeNode]my_field");
        assert!(selector.matches(&path(vec![
            NodeDescriptor::kinded("structFieldTypeNode"),
            NodeDescriptor::named("numberTypeNode", "myField"),
        ])));
    }

    #[test]
    fn dotted_path_matches_ancestors_in_order() {
        let selector = NodeSelector::parse("[instructionNode].amount");
        // Directly nested.
        assert!(selector.matches(&path(vec![
            NodeDescriptor::named("instructionNode", "transfer"),
            NodeDescriptor::named("numberTypeNode", "amount"),
        ])));
        // Non-contiguous: an extra node between the ancestor and the current node.
        assert!(selector.matches(&path(vec![
            NodeDescriptor::named("instructionNode", "transfer"),
            NodeDescriptor::named("structFieldTypeNode", "wrapper"),
            NodeDescriptor::named("numberTypeNode", "amount"),
        ])));
    }

    #[test]
    fn dotted_path_requires_the_ancestor_to_exist() {
        let selector = NodeSelector::parse("[instructionNode].amount");
        assert!(!selector.matches(&path(vec![
            NodeDescriptor::named("accountNode", "myAccount"),
            NodeDescriptor::named("numberTypeNode", "amount"),
        ])));
    }

    #[test]
    fn final_segment_must_match_the_current_node_not_an_ancestor() {
        // `[instructionNode]` selects an instruction *as the current node*, so a
        // path where the instruction is only an ancestor must not match.
        let selector = NodeSelector::parse("[instructionNode]");
        assert!(!selector.matches(&path(vec![
            NodeDescriptor::named("instructionNode", "transfer"),
            NodeDescriptor::kinded("numberTypeNode"),
        ])));
        assert!(selector.matches(&path(vec![NodeDescriptor::named(
            "instructionNode",
            "transfer"
        )])));
    }

    #[test]
    fn empty_path_never_matches() {
        assert!(!NodeSelector::parse("*").matches(&NodePath::new()));
    }
}
