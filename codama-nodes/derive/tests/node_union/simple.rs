use codama_nodes::{NodeTrait, NodeUnionTrait};
use codama_nodes_derive::{node, NodeUnion};

#[node]
pub struct NumberTypeNode {}

#[node]
pub struct StringTypeNode {}

#[derive(NodeUnion, PartialEq, Debug, Clone)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}

fn main() {
    let node = TypeNode::Number(NumberTypeNode {});
    let json = r#"{"kind":"numberTypeNode"}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode>(json).unwrap(), node);

    let node = TypeNode::String(StringTypeNode {});
    let json = r#"{"kind":"stringTypeNode"}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode>(json).unwrap(), node);
}
