#[cfg(test)]
mod tests {
    use crate::{HasKind, NoneValueNode, RegisteredValueNode, ValueNode};

    #[test]
    fn kind_from_standalone() {
        let node: ValueNode = NoneValueNode::new().into();
        assert_eq!(node.kind(), "noneValueNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredValueNode = NoneValueNode::new().into();
        assert_eq!(node.kind(), "noneValueNode");
    }
}
