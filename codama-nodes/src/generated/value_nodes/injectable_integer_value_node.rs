use crate::{InjectedValueNode, IntegerValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InjectableIntegerValueNode {
    Injected(InjectedValueNode),
    Integer(IntegerValueNode),
}
