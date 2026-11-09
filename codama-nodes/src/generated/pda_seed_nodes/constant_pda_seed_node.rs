use crate::{ConstantPdaSeedValue, PluginNode, TypeNode};
use codama_nodes_derive::node;

#[node]
pub struct ConstantPdaSeedNode {
    pub r#type: Box<TypeNode>,
    pub value: Box<ConstantPdaSeedValue>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
