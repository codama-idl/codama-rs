use crate::{InjectedValueNode, PublicKeyValueNode, StringValueNode};
use codama_nodes_derive::{node_union, NodeConversions};

#[derive(NodeConversions)]
#[node_union]
#[non_exhaustive]
pub enum InjectableUnitValueNode {
    InjectedValue(InjectedValueNode),
    PublicKeyValue(PublicKeyValueNode),
    StringValue(StringValueNode),
}
