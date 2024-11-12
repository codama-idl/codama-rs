use crate::Node;

#[derive(Debug)]
pub struct AccountNode {
    // Data.
    pub name: String,
}

impl Node for AccountNode {
    const KIND: &'static str = "accountNode";
}
