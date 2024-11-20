use crate::{Endian, NodeTrait, NumberFormat};
use codama_nodes_derive::{IntoEnum, Node, TypeNode};
use serde::{Deserialize, Serialize};

#[derive(Node, TypeNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "WrappedNumberTypeNode", into = "WrappedNumberTypeNode")]
struct NumberTypeNode {
    format: NumberFormat,
    endian: Endian,
}

#[derive(Node, TypeNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(try_from = "WrappedAmountTypeNode", into = "WrappedAmountTypeNode")]
struct AmountTypeNode {
    decimals: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    number: NumberTypeNode,
}

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum TypeNode {
    Amount(AmountTypeNode),
    Number(NumberTypeNode),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WrappedNumberTypeNode<'a> {
    kind: &'a str,
    format: NumberFormat,
    endian: Endian,
}

impl<'a> From<NumberTypeNode> for WrappedNumberTypeNode<'a> {
    fn from(node: NumberTypeNode) -> Self {
        Self {
            kind: NumberTypeNode::KIND,
            format: node.format.clone(),
            endian: node.endian.clone(),
        }
    }
}

impl<'a> TryFrom<WrappedNumberTypeNode<'a>> for NumberTypeNode {
    type Error = serde::de::value::Error;

    fn try_from(node: WrappedNumberTypeNode<'a>) -> Result<Self, Self::Error> {
        if node.kind != NumberTypeNode::KIND {
            return Err(serde::de::Error::custom(format!(
                "Invalid kind: expected '{}', got '{}'",
                Self::KIND,
                node.kind,
            )));
        }
        Ok(Self {
            format: node.format,
            endian: node.endian,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WrappedAmountTypeNode<'a> {
    kind: &'a str,
    decimals: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    number: NumberTypeNode,
}

impl<'a> From<AmountTypeNode> for WrappedAmountTypeNode<'a> {
    fn from(node: AmountTypeNode) -> Self {
        Self {
            kind: AmountTypeNode::KIND,
            decimals: node.decimals.clone(),
            unit: node.unit.clone(),
            number: node.number.clone(),
        }
    }
}

impl<'a> TryFrom<WrappedAmountTypeNode<'a>> for AmountTypeNode {
    type Error = serde::de::value::Error;

    fn try_from(node: WrappedAmountTypeNode<'a>) -> Result<Self, Self::Error> {
        if node.kind != AmountTypeNode::KIND {
            return Err(serde::de::Error::custom(format!(
                "Invalid kind: expected '{}', got '{}'",
                Self::KIND,
                node.kind,
            )));
        }
        Ok(Self {
            decimals: node.decimals,
            unit: node.unit,
            number: node.number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
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

    #[test]
    fn amount() {
        let node: TypeNode = AmountTypeNode {
            decimals: 9,
            unit: None,
            number: NumberTypeNode {
                format: NumberFormat::U8,
                endian: Endian::Big,
            },
        }
        .into();

        // Serialize node.
        let expected_json = r#"{"kind":"amountTypeNode","decimals":9,"number":{"kind":"numberTypeNode","format":"u8","endian":"be"}}"#;
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, expected_json);

        // Deserialize node.
        let deserialized_node = serde_json::from_str::<TypeNode>(&json).unwrap();
        assert_eq!(deserialized_node, node.clone());

        // Validate kind.
        let wrong_json = r#"{"kind":"Oops","decimals":9,"number":{"kind":"numberTypeNode","format":"u8","endian":"be"}}"#;
        let wrong_result = serde_json::from_str::<TypeNode>(wrong_json);
        assert_eq!(wrong_result.is_err(), true);
    }
}
