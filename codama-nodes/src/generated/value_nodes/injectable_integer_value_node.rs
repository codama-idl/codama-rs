use crate::{InjectedValueNode, IntegerValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum InjectableIntegerValueNode {
    Injected(InjectedValueNode),
    Integer(IntegerValueNode),
}
