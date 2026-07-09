use crate::{InjectedValueNode, StringValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InjectableStringValueNode {
    Injected(InjectedValueNode),
    String(StringValueNode),
}
