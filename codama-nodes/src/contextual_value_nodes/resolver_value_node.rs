use crate::{AccountValueNode, ArgumentValueNode, CamelCaseString, Docs};
use codama_nodes_derive::{node, IntoEnum};
use serde::{Deserialize, Serialize};

#[node]
pub struct ResolverValueNode {
    // Data.
    pub name: CamelCaseString,
    pub docs: Docs,

    // Children.
    pub depends_on: Option<Vec<ResolverDependency>>,
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

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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
}
