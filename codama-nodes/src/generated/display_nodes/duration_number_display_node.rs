use codama_nodes_derive::node;

#[node]
#[derive(Copy, Default)]
pub struct DurationNumberDisplayNode {
    // Data.
    #[serde(skip_serializing_if = "crate::is_default")]
    pub ticks_per_second: Option<u64>,
}

impl From<DurationNumberDisplayNode> for crate::Node {
    fn from(val: DurationNumberDisplayNode) -> Self {
        crate::Node::Display(val.into())
    }
}
