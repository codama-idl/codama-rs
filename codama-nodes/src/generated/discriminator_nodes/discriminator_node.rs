use crate::{ConstantDiscriminatorNode, FieldDiscriminatorNode, SizeDiscriminatorNode};
use codama_nodes_derive::node_union;

#[node_union]
pub enum DiscriminatorNode {
    Constant(ConstantDiscriminatorNode),
    Field(FieldDiscriminatorNode),
    Size(SizeDiscriminatorNode),
}
