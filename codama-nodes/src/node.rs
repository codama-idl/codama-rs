use crate::{HasKind, Node};

impl HasKind for Option<Node> {
    fn kind(&self) -> &'static str {
        match self {
            Some(node) => node.kind(),
            None => "None",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DefinedTypeLinkNode, DefinedTypeNode, FixedSizeTransformNode, IntegerValueNode, NodeRef,
        PayerValueNode, ProgramNode, StringTypeNode, TextNode, TypeNode, ValueNode,
    };

    #[test]
    fn kind() {
        let node: Node = ProgramNode::default().into();
        assert_eq!(node.kind(), "programNode");
    }

    #[test]
    fn type_node_to_json() {
        let node: Node = StringTypeNode::utf8().into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"stringTypeNode","encoding":"utf8"}"#);
    }

    #[test]
    fn type_node_from_json() {
        let json = r#"{"kind":"stringTypeNode","encoding":"utf8"}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(node, Node::StringType(StringTypeNode::utf8()));
    }

    #[test]
    fn transform_node_to_json() {
        let node: Node = FixedSizeTransformNode {
            size: 4,
            plugins: vec![],
        }
        .into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"fixedSizeTransformNode","size":4}"#);
    }

    #[test]
    fn transform_node_from_json() {
        let json = r#"{"kind":"fixedSizeTransformNode","size":4}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::FixedSizeTransform(FixedSizeTransformNode {
                size: 4,
                plugins: vec![],
            })
        );
    }

    #[test]
    fn value_node_to_json() {
        let node: Node = IntegerValueNode::new(42).into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"integerValueNode","value":"42"}"#);
    }

    #[test]
    fn value_node_from_json() {
        let json = r#"{"kind":"integerValueNode","value":"42"}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(node, Node::IntegerValue(IntegerValueNode::new(42)));
    }

    #[test]
    fn text_node_to_json() {
        let node: Node = TextNode {
            content: "Hello".to_string(),
            plugins: vec![],
        }
        .into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"textNode","content":"Hello"}"#);
    }

    #[test]
    fn text_node_from_json() {
        let json = r#"{"kind":"textNode","content":"Hello"}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::Text(TextNode {
                content: "Hello".to_string(),
                plugins: vec![],
            })
        );
    }

    #[test]
    fn defined_type_to_json() {
        let node: Node = DefinedTypeNode {
            identifier: "myType".into(),
            docs: vec!["Hello".to_string()].into(),
            r#type: Box::new(StringTypeNode::utf8().into()),
            plugins: vec![],
        }
        .into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeNode","identifier":"myType","docs":"Hello","type":{"kind":"stringTypeNode","encoding":"utf8"}}"#
        );
    }

    #[test]
    fn defined_type_from_json() {
        let json = r#"{"kind":"definedTypeNode","identifier":"myType","docs":"Hello","type":{"kind":"stringTypeNode","encoding":"utf8"}}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::DefinedType(DefinedTypeNode {
                identifier: "myType".into(),
                docs: vec!["Hello".to_string()].into(),
                r#type: Box::new(StringTypeNode::utf8().into()),
                plugins: vec![],
            })
        );
    }

    #[test]
    fn from_json_rejects_an_unknown_kind() {
        let json = r#"{"kind":"bananaNode"}"#;
        let error = serde_json::from_str::<Node>(json).unwrap_err();
        assert_eq!(error.to_string(), "unknown kind bananaNode for Node");
    }

    #[test]
    fn node_ref_borrows_and_clones_each_variant() {
        let nodes: [Node; 6] = [
            StringTypeNode::utf8().into(),
            IntegerValueNode::new(42).into(),
            DefinedTypeLinkNode::new("myType").into(),
            FixedSizeTransformNode::new(4).into(),
            PayerValueNode::new().into(),
            ProgramNode::default().into(),
        ];
        for node in nodes {
            let node_ref = NodeRef::from(&node);
            assert_eq!(node_ref.kind(), node.kind());
            assert_eq!(node_ref.to_node(), node);
        }
        let node: Node = StringTypeNode::utf8().into();
        assert!(
            matches!(NodeRef::from(&node), NodeRef::StringType(string) if *string == StringTypeNode::utf8())
        );
    }

    #[test]
    fn node_ref_from_a_node_struct() {
        let program = ProgramNode::default();
        assert!(
            matches!(NodeRef::from(&program), NodeRef::Program(p) if std::ptr::eq(p, &program))
        );
    }

    #[test]
    fn node_struct_try_from_node() {
        let node: Node = IntegerValueNode::new(42).into();
        assert_eq!(
            IntegerValueNode::try_from(node.clone()).unwrap(),
            IntegerValueNode::new(42)
        );
        let error = StringTypeNode::try_from(node).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not convert node from `integerValueNode` into `stringTypeNode`"
        );
    }

    #[test]
    fn union_into_node_and_node_ref() {
        let link = DefinedTypeLinkNode::new("myType");
        let r#type = TypeNode::Link(link.clone());
        assert!(matches!(NodeRef::from(&r#type), NodeRef::DefinedTypeLink(l) if *l == link));
        assert_eq!(Node::from(r#type), Node::DefinedTypeLink(link));
    }

    #[test]
    fn union_try_from_node() {
        let node = Node::DefinedTypeLink(DefinedTypeLinkNode::new("myType"));
        assert_eq!(
            TypeNode::try_from(node.clone()).unwrap(),
            TypeNode::Link(DefinedTypeLinkNode::new("myType"))
        );
        let error = ValueNode::try_from(node).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not convert node from `definedTypeLinkNode` into `ValueNode`"
        );
    }

    #[test]
    fn node_kinds() {
        use crate::{NodeKinds, RegisteredValueNode};
        assert_eq!(IntegerValueNode::KINDS, ["integerValueNode"]);
        assert_eq!(Node::KINDS.len(), 94);
        assert!(TypeNode::KINDS.contains(&"definedTypeLinkNode"));
        assert!(!TypeNode::KINDS.contains(&"structFieldTypeNode"));
        assert!(RegisteredValueNode::KINDS.contains(&"structFieldValueNode"));
        assert!(!ValueNode::KINDS.contains(&"structFieldValueNode"));
    }
}
