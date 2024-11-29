use crate::{FixedCountNode, PrefixedCountNode, RemainderCountNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum CountNode {
    Fixed(FixedCountNode),
    Prefixed(PrefixedCountNode),
    Remainder(RemainderCountNode),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeUnionTrait;

    #[test]
    fn kind() {
        let node: CountNode = RemainderCountNode::new().into();
        assert_eq!(node.kind(), "remainderCountNode");
    }
}
