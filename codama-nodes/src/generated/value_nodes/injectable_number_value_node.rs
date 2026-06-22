use crate::{InjectedValueNode, NumberValueNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InjectableNumberValueNode {
    Injected(InjectedValueNode),
    Number(NumberValueNode),
}
