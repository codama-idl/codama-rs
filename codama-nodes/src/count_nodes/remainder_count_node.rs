use codama_nodes_derive::node;

#[node]
pub struct RemainderCountNode {}

impl RemainderCountNode {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = RemainderCountNode::new();
        assert_eq!(node, RemainderCountNode {});
    }
}
