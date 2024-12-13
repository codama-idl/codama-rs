use crate::ConstantValueNode;
use codama_nodes_derive::node;

#[node]
pub struct ConstantDiscriminatorNode {
    // Data.
    pub offset: usize,

    // Children.
    pub constant: ConstantValueNode,
}

impl From<ConstantDiscriminatorNode> for crate::Node {
    fn from(val: ConstantDiscriminatorNode) -> Self {
        crate::Node::Discriminator(val.into())
    }
}

impl ConstantDiscriminatorNode {
    pub fn new<T>(constant: T, offset: usize) -> Self
    where
        T: Into<ConstantValueNode>,
    {
        Self {
            constant: constant.into(),
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Base16, NumberTypeNode, NumberValueNode, U32};

    use super::*;

    #[test]
    fn new() {
        let node = ConstantDiscriminatorNode::new(
            ConstantValueNode::new(NumberTypeNode::le(U32), NumberValueNode::new(42u32)),
            0,
        );
        assert_eq!(
            node.constant,
            ConstantValueNode::new(NumberTypeNode::le(U32), NumberValueNode::new(42u32))
        );
        assert_eq!(node.offset, 0);
    }

    #[test]
    fn to_json() {
        let node =
            ConstantDiscriminatorNode::new(ConstantValueNode::bytes(Base16, "deadb0d1e5"), 0);
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"constantDiscriminatorNode","offset":0,"constant":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"constantDiscriminatorNode","offset":0,"constant":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},"value":{"kind":"bytesValueNode","data":"deadb0d1e5","encoding":"base16"}}}"#;
        let node: ConstantDiscriminatorNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            ConstantDiscriminatorNode::new(ConstantValueNode::bytes(Base16, "deadb0d1e5"), 0)
        );
    }
}
