use crate::NodeTrait;

#[derive(Debug)]
pub struct AccountNode {
    // Data.
    pub name: String,
}

impl NodeTrait for AccountNode {
    const KIND: &'static str = "accountNode";
}
