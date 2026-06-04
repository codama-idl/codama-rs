#[cfg(test)]
mod tests {
    use crate::{CountNode, HasKind, RemainderCountNode};

    #[test]
    fn kind() {
        let node: CountNode = RemainderCountNode::new().into();
        assert_eq!(node.kind(), "remainderCountNode");
    }
}
