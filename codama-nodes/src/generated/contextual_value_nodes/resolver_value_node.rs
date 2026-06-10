use crate::{CamelCaseString, Docs, HasName, ResolverDependency};
use codama_nodes_derive::node;

#[node]
pub struct ResolverValueNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub docs: Docs,

    // Children.
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub depends_on: Vec<ResolverDependency>,
}

impl From<ResolverValueNode> for crate::Node {
    fn from(val: ResolverValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}

impl HasName for ResolverValueNode {
    fn name(&self) -> &CamelCaseString {
        &self.name
    }
}
