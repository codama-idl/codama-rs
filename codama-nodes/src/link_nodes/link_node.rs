#[cfg(test)]
mod tests {
    use crate::{HasKind, LinkNode, ProgramLinkNode};

    #[test]
    fn kind() {
        let node: LinkNode = ProgramLinkNode::new("myProgram").into();
        assert_eq!(node.kind(), "programLinkNode");
    }
}
