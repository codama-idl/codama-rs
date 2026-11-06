use crate::{
    AccountNode, ConstantNode, ContextualValueNode, CountNode, DefinedTypeNode, DiscriminatorNode,
    DisplayNode, ErrorNode, EventNode, HasKind, InstructionAccountNode, InstructionByteDeltaNode,
    InstructionNode, InstructionRemainingAccountsNode, InstructionStatusNode, LinkNode,
    NodeUnionTrait, PdaNode, PdaSeedNode, PluginNode, ProgramNode, ProvidedNode,
    RegisteredContextualValueNode, RegisteredTypeNode, RegisteredValueNode, RootNode, TextNode,
    TransformNode, TypeNode, ValueNode,
};
use derive_more::derive::From;
use serde::{Deserialize, Serialize};

#[derive(From, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
    // Node unions.
    ContextualValue(RegisteredContextualValueNode),
    Count(CountNode),
    Discriminator(DiscriminatorNode),
    Display(DisplayNode),
    Link(LinkNode),
    PdaSeed(PdaSeedNode),
    Transform(TransformNode),
    Type(RegisteredTypeNode),
    Value(RegisteredValueNode),

    // Nodes.
    Account(AccountNode),
    Constant(ConstantNode),
    DefinedType(DefinedTypeNode),
    Error(ErrorNode),
    Event(EventNode),
    Instruction(InstructionNode),
    InstructionAccount(InstructionAccountNode),
    InstructionByteDelta(InstructionByteDeltaNode),
    InstructionRemainingAccounts(InstructionRemainingAccountsNode),
    InstructionStatus(InstructionStatusNode),
    Pda(PdaNode),
    Plugin(PluginNode),
    Program(ProgramNode),
    Provided(ProvidedNode),
    Root(RootNode),
    Text(TextNode),
}

impl From<ContextualValueNode> for Node {
    fn from(node: ContextualValueNode) -> Self {
        Node::ContextualValue(node.into())
    }
}

impl From<TypeNode> for Node {
    fn from(node: TypeNode) -> Self {
        match node {
            TypeNode::Link(link) => link.into(),
            _ => Node::Type(node.try_into().unwrap()),
        }
    }
}

impl From<ValueNode> for Node {
    fn from(node: ValueNode) -> Self {
        Node::Value(node.into())
    }
}

impl NodeUnionTrait for Node {}
impl HasKind for Node {
    fn kind(&self) -> &'static str {
        match self {
            Node::ContextualValue(node) => node.kind(),
            Node::Count(node) => node.kind(),
            Node::Discriminator(node) => node.kind(),
            Node::Display(node) => node.kind(),
            Node::Link(node) => node.kind(),
            Node::PdaSeed(node) => node.kind(),
            Node::Transform(node) => node.kind(),
            Node::Type(node) => node.kind(),
            Node::Value(node) => node.kind(),
            Node::Account(node) => node.kind(),
            Node::Constant(node) => node.kind(),
            Node::DefinedType(node) => node.kind(),
            Node::Error(node) => node.kind(),
            Node::Event(node) => node.kind(),
            Node::Instruction(node) => node.kind(),
            Node::InstructionAccount(node) => node.kind(),
            Node::InstructionByteDelta(node) => node.kind(),
            Node::InstructionRemainingAccounts(node) => node.kind(),
            Node::InstructionStatus(node) => node.kind(),
            Node::Pda(node) => node.kind(),
            Node::Plugin(node) => node.kind(),
            Node::Program(node) => node.kind(),
            Node::Provided(node) => node.kind(),
            Node::Root(node) => node.kind(),
            Node::Text(node) => node.kind(),
        }
    }
}

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
    use crate::{CamelCaseString, FixedSizeTransformNode, IntegerValueNode, StringTypeNode};

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
        assert_eq!(
            node,
            Node::Type(RegisteredTypeNode::String(StringTypeNode::utf8()))
        );
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
            Node::Transform(TransformNode::FixedSize(FixedSizeTransformNode {
                size: 4,
                plugins: vec![],
            }))
        );
    }

    #[test]
    fn value_node_to_json() {
        let node: Node = IntegerValueNode {
            value: 42.into(),
            plugins: vec![],
        }
        .into();
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"integerValueNode","value":"42"}"#);
    }

    #[test]
    fn value_node_from_json() {
        let json = r#"{"kind":"integerValueNode","value":"42"}"#;
        let node: Node = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            Node::Value(RegisteredValueNode::Integer(IntegerValueNode {
                value: 42.into(),
                plugins: vec![],
            }))
        );
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
            identifier: CamelCaseString::new("myType"),
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
                identifier: CamelCaseString::new("myType"),
                docs: vec!["Hello".to_string()].into(),
                r#type: Box::new(StringTypeNode::utf8().into()),
                plugins: vec![],
            })
        );
    }
}
