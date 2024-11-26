use crate::{AccountValueNode, ArgumentValueNode, CamelCaseString, Docs};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct ResolverValueNode {
    // Data.
    pub name: CamelCaseString,
    #[serde(default)]
    #[serde(skip_serializing_if = "Docs::is_empty")]
    pub docs: Docs,

    // Children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<ResolverDependency>>,
}

impl Into<crate::Node> for ResolverValueNode {
    fn into(self) -> crate::Node {
        crate::Node::ContextualValue(self.into())
    }
}

impl ResolverValueNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            depends_on: None,
        }
    }
}

#[node_union]
pub enum ResolverDependency {
    Account(AccountValueNode),
    Argument(ArgumentValueNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = ResolverValueNode::new("my_resolver");
        assert_eq!(node.name, CamelCaseString::new("myResolver"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.depends_on, None);
    }

    #[test]
    fn direct_instantiation() {
        let node = ResolverValueNode {
            name: "myResolver".into(),
            docs: vec!["I am some resolver docs.".to_string()].into(),
            depends_on: Some(vec![
                AccountValueNode::new("myDependentAccount").into(),
                ArgumentValueNode::new("myDependentArgument").into(),
            ]),
        };
        assert_eq!(node.name, CamelCaseString::new("myResolver"));
        assert_eq!(
            node.docs,
            Docs::from(vec!["I am some resolver docs.".to_string()])
        );
        assert_eq!(
            node.depends_on,
            Some(vec![
                ResolverDependency::Account(AccountValueNode::new("myDependentAccount")),
                ResolverDependency::Argument(ArgumentValueNode::new("myDependentArgument")),
            ])
        );
    }

    #[test]
    fn to_json() {
        let node = ResolverValueNode::new("myResolver");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"resolverValueNode","name":"myResolver"}"#);
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"resolverValueNode","name":"myResolver"}"#;
        let node: ResolverValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, ResolverValueNode::new("myResolver"));
    }
}
