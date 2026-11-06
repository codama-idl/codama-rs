use crate::{InjectedValueNode, PublicKeyValueNode, StringValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InjectableUnitValueNode {
    Injected(InjectedValueNode),
    PublicKey(PublicKeyValueNode),
    String(StringValueNode),
}
