use crate::PluginNode;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct TextNode {
    pub content: String,
    #[serde(default, skip_serializing_if = "crate::is_default")]
    pub plugins: Vec<PluginNode>,
}
