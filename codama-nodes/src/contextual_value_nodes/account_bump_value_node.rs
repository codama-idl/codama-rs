use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct AccountBumpValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl AccountBumpValueNode {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self { name: name.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = AccountBumpValueNode::new("my_account");
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
    }
}
