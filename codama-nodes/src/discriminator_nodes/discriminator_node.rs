use crate::{ConstantDiscriminatorNode, FieldDiscriminatorNode, SizeDiscriminatorNode};
use codama_nodes_derive::IntoEnum;

#[derive(IntoEnum, Debug, PartialEq, Clone)]
pub enum DiscriminatorNode {
    Constant(ConstantDiscriminatorNode),
    Field(FieldDiscriminatorNode),
    Size(SizeDiscriminatorNode),
}
