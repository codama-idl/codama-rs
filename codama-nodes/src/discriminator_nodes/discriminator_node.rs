use crate::{ConstantDiscriminatorNode, FieldDiscriminatorNode, SizeDiscriminatorNode};
use codama_nodes_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(IntoEnum, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiscriminatorNode {
    Constant(ConstantDiscriminatorNode),
    Field(FieldDiscriminatorNode),
    Size(SizeDiscriminatorNode),
}
