use crate::{DateTimeTypeNode, NestedTypeNode, NumberTypeNode};

impl DateTimeTypeNode {
    pub fn new<T>(number: T) -> Self
    where
        T: Into<NestedTypeNode<NumberTypeNode>>,
    {
        Self {
            number: number.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Endianness, NestedTypeNodeTrait, NumberTypeNode, PostOffsetTypeNode, PreOffsetTypeNode, U64,
    };

    #[test]
    fn new() {
        let node = DateTimeTypeNode::new(NumberTypeNode::new(U64, Endianness::Be));
        assert_eq!(
            node.number,
            NestedTypeNode::Value(NumberTypeNode::new(U64, Endianness::Be))
        );
    }

    #[test]
    fn new_with_nested_value() {
        let node = DateTimeTypeNode::new(PostOffsetTypeNode::pre_offset(
            PreOffsetTypeNode::absolute(NumberTypeNode::le(U64), 0),
            0,
        ));
        assert_eq!(
            node.number,
            NestedTypeNode::PostOffset(PostOffsetTypeNode::pre_offset(
                NestedTypeNode::PreOffset(PreOffsetTypeNode::absolute(
                    NestedTypeNode::Value(NumberTypeNode::le(U64)),
                    0
                )),
                0,
            ))
        );
        assert_eq!(
            node.number.get_nested_type_node(),
            &NumberTypeNode::le(U64,)
        );
    }

    #[test]
    fn to_json() {
        let node = DateTimeTypeNode::new(NumberTypeNode::le(U64));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"dateTimeTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"dateTimeTypeNode","number":{"kind":"numberTypeNode","format":"u64","endian":"le"}}"#;
        let node: DateTimeTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DateTimeTypeNode::new(NumberTypeNode::le(U64)));
    }
}
