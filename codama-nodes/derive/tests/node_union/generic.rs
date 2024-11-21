use codama_nodes_derive::NodeUnion;

pub trait SomeTrait {}
impl SomeTrait for u32 {}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "kind", rename = "numberTypeNode")]
pub struct NumberTypeNode<T: SomeTrait> {
    pub value: T,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(tag = "kind", rename = "stringTypeNode")]
pub struct StringTypeNode<T: SomeTrait> {
    pub value: T,
}

#[derive(NodeUnion, PartialEq, Debug)]
pub enum TypeNode<T: SomeTrait> {
    Number(NumberTypeNode<T>),
    String(StringTypeNode<T>),
}

fn main() {
    let node = TypeNode::Number(NumberTypeNode { value: 42 });
    let json = r#"{"kind":"numberTypeNode","value":42}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode<u32>>(json).unwrap(), node);

    let node = TypeNode::String(StringTypeNode { value: 42 });
    let json = r#"{"kind":"stringTypeNode","value":42}"#;
    assert_eq!(serde_json::to_string(&node).unwrap(), json);
    assert_eq!(serde_json::from_str::<TypeNode<u32>>(json).unwrap(), node);
}
