use crate::{Endian, NumberFormat};
use codama_nodes_derive::{IntoEnum, Node, TypeNode};
use serde::{Deserialize, Serialize};

#[derive(Node, TypeNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
struct NumberTypeNode {
    format: NumberFormat,
    endian: Endian,
}

#[derive(Node, TypeNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AmountTypeNode {
    decimals: u8,
    unit: Option<String>,
    number: NumberTypeNode,
}

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum TypeNode {
    Amount(AmountTypeNode),
    Number(NumberTypeNode),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let node: TypeNode = NumberTypeNode {
            format: NumberFormat::U8,
            endian: Endian::Big,
        }
        .into();

        // Serialize node.
        let expected_json = r#"{"kind":"numberTypeNode","format":"u8","endian":"be"}"#;
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, expected_json);

        // Deserialize node.
        let deserialized_node = serde_json::from_str::<TypeNode>(&json).unwrap();
        assert_eq!(deserialized_node, node.clone());

        // Validate kind.
        let wrong_json = r#"{"kind":"Oops","format":"u8","endian":"be"}"#;
        let wrong_result = serde_json::from_str::<TypeNode>(wrong_json);
        assert_eq!(wrong_result.is_err(), true);
    }
}
