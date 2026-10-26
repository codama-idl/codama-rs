use crate::{InjectableUnitValueNode, UnitNumberDisplayNode};

impl UnitNumberDisplayNode {
    pub fn new<T>(unit: T) -> Self
    where
        T: Into<InjectableUnitValueNode>,
    {
        Self {
            unit: Box::new(unit.into()),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StringValueNode;

    #[test]
    fn new() {
        let node = UnitNumberDisplayNode::new(StringValueNode::new("lamports"));
        assert_eq!(
            *node.unit,
            InjectableUnitValueNode::String(StringValueNode::new("lamports"))
        );
    }

    #[test]
    fn to_json() {
        let node = UnitNumberDisplayNode::new(StringValueNode::new("lamports"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"unitNumberDisplayNode","unit":{"kind":"stringValueNode","string":"lamports"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"unitNumberDisplayNode","unit":{"kind":"stringValueNode","string":"lamports"}}"#;
        let node: UnitNumberDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            UnitNumberDisplayNode::new(StringValueNode::new("lamports"))
        );
    }
}
