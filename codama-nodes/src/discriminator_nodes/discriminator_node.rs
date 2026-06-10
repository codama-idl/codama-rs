#[cfg(test)]
mod tests {
    use crate::{DiscriminatorNode, HasKind, SizeDiscriminatorNode};

    #[test]
    fn kind() {
        let node: DiscriminatorNode = SizeDiscriminatorNode::new(42).into();
        assert_eq!(node.kind(), "sizeDiscriminatorNode");
    }
}
