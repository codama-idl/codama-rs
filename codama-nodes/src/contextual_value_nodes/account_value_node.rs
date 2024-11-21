use crate::CamelCaseString;
use codama_nodes_derive::node;

#[node]
pub struct AccountValueNode {
    // Data.
    pub name: CamelCaseString,
}

impl AccountValueNode {
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
        let node = AccountValueNode::new("my_account");
        assert_eq!(node.name, CamelCaseString::new("myAccount"));
    }
}
