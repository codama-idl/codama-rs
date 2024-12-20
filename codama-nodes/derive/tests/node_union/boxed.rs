use codama_nodes::{HasKind, NodeTrait, NodeUnionTrait};
use codama_nodes_derive::{node, NodeUnion};

#[node]
pub struct NumberTypeNode {}

#[node]
pub struct StringTypeNode {}

#[derive(NodeUnion, PartialEq, Debug, Clone)]
pub enum TypeNode {
    Number(Box<NumberTypeNode>),
    String(Box<StringTypeNode>),
}

fn main() {
    let node = TypeNode::Number(Box::new(NumberTypeNode {}));
    let json = r#"{"kind":"numberTypeNode"}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode>(json).unwrap(), node);

    let node = TypeNode::String(Box::new(StringTypeNode {}));
    let json = r#"{"kind":"stringTypeNode"}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode>(json).unwrap(), node);
}
