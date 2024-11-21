use crate::{CamelCaseString, DefinedTypeLinkNode, StructValueNode, TupleValueNode};
use codama_nodes_derive::{node, node_union};

#[node]
pub struct EnumValueNode {
    // Data.
    pub variant: CamelCaseString,

    // Children.
    pub r#enum: DefinedTypeLinkNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EnumVariantData>,
}

impl EnumValueNode {
    pub fn new<T, U>(r#enum: T, variant: U, value: Option<EnumVariantData>) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value,
        }
    }

    pub fn empty<T, U>(r#enum: T, variant: U) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: None,
        }
    }

    pub fn fields<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
        V: Into<StructValueNode>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Some(EnumVariantData::Struct(value.into())),
        }
    }

    pub fn tuple<T, U, V>(r#enum: T, variant: U, value: V) -> Self
    where
        T: Into<DefinedTypeLinkNode>,
        U: Into<CamelCaseString>,
        V: Into<TupleValueNode>,
    {
        Self {
            variant: variant.into(),
            r#enum: r#enum.into(),
            value: Some(EnumVariantData::Tuple(value.into())),
        }
    }
}

#[node_union]
pub enum EnumVariantData {
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}

#[cfg(test)]
mod tests {
    use crate::{NumberValueNode, StringValueNode, StructFieldValueNode};

    use super::*;

    #[test]
    fn new() {
        let node = EnumValueNode::new("direction", "north", None);
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("direction"));
        assert_eq!(node.variant, CamelCaseString::from("north"));
        assert_eq!(node.value, None);
    }

    #[test]
    fn empty() {
        let node = EnumValueNode::empty("command", "exit");
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("exit"));
        assert_eq!(node.value, None);
    }

    #[test]
    fn fields() {
        let node = EnumValueNode::fields(
            "command",
            "move",
            vec![
                StructFieldValueNode::new("x", NumberValueNode::new(10)),
                StructFieldValueNode::new("y", NumberValueNode::new(20)),
            ],
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("move"));
        assert_eq!(
            node.value,
            Some(EnumVariantData::Struct(StructValueNode::new(vec![
                StructFieldValueNode::new("x", NumberValueNode::new(10)),
                StructFieldValueNode::new("y", NumberValueNode::new(20)),
            ])))
        );
    }

    #[test]
    fn tuple() {
        let node = EnumValueNode::tuple(
            "command",
            "write",
            vec![StringValueNode::new("Hello World").into()],
        );
        assert_eq!(node.r#enum, DefinedTypeLinkNode::new("command"));
        assert_eq!(node.variant, CamelCaseString::from("write"));
        assert_eq!(
            node.value,
            Some(EnumVariantData::Tuple(TupleValueNode::new(vec![
                StringValueNode::new("Hello World").into()
            ])))
        );
    }

    #[test]
    fn to_json() {
        let node = EnumValueNode::empty("command", "exit");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumValueNode","variant":"exit","enum":{"kind":"definedTypeLinkNode","name":"command"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumValueNode","variant":"exit","enum":{"kind":"definedTypeLinkNode","name":"command"}}"#;
        let node: EnumValueNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumValueNode::empty("command", "exit"));
    }
}
