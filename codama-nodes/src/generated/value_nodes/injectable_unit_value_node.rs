use crate::{InjectedValueNode, PublicKeyValueNode, StringValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
pub enum InjectableUnitValueNode {
    Injected(InjectedValueNode),
    PublicKey(PublicKeyValueNode),
    String(StringValueNode),
}
