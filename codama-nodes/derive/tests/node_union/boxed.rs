use codama_nodes_derive::NodeUnion;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "kind", rename = "numberTypeNode")]
pub struct NumberTypeNode {}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "kind", rename = "stringTypeNode")]
pub struct StringTypeNode {}

#[derive(NodeUnion, PartialEq, Debug)]
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
