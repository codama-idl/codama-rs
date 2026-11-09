use crate::{DefinedTypeLinkNode, Identifier, PluginNode, ValueNode};
use codama_nodes_derive::node;

#[node]
pub struct EnumValueNode {
    pub variant: Identifier,
    pub r#enum: DefinedTypeLinkNode,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub value: Box<Option<ValueNode>>,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
