use codama_nodes::{HasKind, NodeTrait, NodeUnionTrait};
use codama_nodes_derive::{node, NodeUnion};
use std::fmt::Debug;

pub trait SomeTrait:
    Clone + PartialEq + Debug + serde::Serialize + for<'de> serde::Deserialize<'de>
{
}
impl SomeTrait for u32 {}

#[node]
pub struct NumberTypeNode<T: SomeTrait> {
    #[serde(bound(serialize = "T: SomeTrait", deserialize = "T: SomeTrait"))]
    pub value: T,
}

#[node]
pub struct StringTypeNode<T: SomeTrait> {
    #[serde(bound(serialize = "T: SomeTrait", deserialize = "T: SomeTrait"))]
    pub value: T,
}

#[derive(NodeUnion, PartialEq, Debug, Clone)]
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
