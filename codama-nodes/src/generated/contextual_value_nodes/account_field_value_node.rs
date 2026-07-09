use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
#[derive(Default)]
pub struct AccountFieldValueNode {
    // Data.
    pub account: CamelCaseString,
    #[serde(skip_serializing_if = "crate::is_default")]
    pub path: Option<CamelCaseString>,
}

impl From<AccountFieldValueNode> for crate::Node {
    fn from(val: AccountFieldValueNode) -> Self {
        crate::Node::ContextualValue(val.into())
    }
}
