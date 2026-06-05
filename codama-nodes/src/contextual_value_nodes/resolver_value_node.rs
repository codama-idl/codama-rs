use crate::{CamelCaseString, Docs, ResolverValueNode};

impl ResolverValueNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            name: name.into(),
            docs: Docs::default(),
            depends_on: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountValueNode, ArgumentValueNode, ResolverDependency};

    #[test]
    fn new() {
        let node = ResolverValueNode::new("my_resolver");
        assert_eq!(node.name, CamelCaseString::new("myResolver"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.depends_on, vec![]);
    }

    #[test]
    fn direct_instantiation() {
        let node = ResolverValueNode {
            name: "myResolver".into(),
            docs: vec!["I am some resolver docs.".to_string()].into(),
            depends_on: vec![
                AccountValueNode::new("myDependentAccount").into(),
                ArgumentValueNode::new("myDependentArgument").into(),
            ],
        };
        assert_eq!(node.name, CamelCaseString::new("myResolver"));
        assert_eq!(
            node.docs,
            Docs::from(vec!["I am some resolver docs.".to_string()])
        );
        assert_eq!(
            node.depends_on,
            vec![
                ResolverDependency::Account(AccountValueNode::new("myDependentAccount")),
                ResolverDependency::Argument(ArgumentValueNode::new("myDependentArgument")),
            ]
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
