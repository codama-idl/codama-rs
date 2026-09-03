use codama_nodes::{HasKind, NodeTrait, NodeUnionTrait};
use codama_nodes_derive::{node, NodeUnion};

#[node]
pub struct NumberTypeNode {
    pub value: u8,
}

#[node]
pub struct StringTypeNode {}

#[derive(NodeUnion, PartialEq, Debug, Clone)]
pub enum TypeNode {
    Number(NumberTypeNode),
    String(StringTypeNode),
}

fn main() {
    // The error names the union being deserialized, not an unrelated node.
    let error = serde_json::from_str::<TypeNode>(r#"{"kind":"numberTypeNode"}"#).unwrap_err();
    assert_eq!(
        error.to_string(),
        "failed to deserialize TypeNode: missing field `value`"
    );
}
