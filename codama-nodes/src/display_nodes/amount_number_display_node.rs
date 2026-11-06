use crate::{AmountNumberDisplayNode, InjectableIntegerValueNode, InjectableUnitValueNode};

impl AmountNumberDisplayNode {
    pub fn new<T, U>(decimals: T, unit: U) -> Self
    where
        T: Into<InjectableIntegerValueNode>,
        U: Into<InjectableUnitValueNode>,
    {
        Self {
            decimals: Box::new(decimals.into()),
            unit: Box::new(Some(unit.into())),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegerValueNode, StringValueNode};

    #[test]
    fn new() {
        let node =
            AmountNumberDisplayNode::new(IntegerValueNode::new(6u8), StringValueNode::new("USDC"));
        assert_eq!(
            *node.decimals,
            InjectableIntegerValueNode::Integer(IntegerValueNode::new(6u8))
        );
        assert_eq!(
            *node.unit,
            Some(InjectableUnitValueNode::String(StringValueNode::new(
                "USDC"
            )))
        );
    }

    #[test]
    fn to_json() {
        let node =
            AmountNumberDisplayNode::new(IntegerValueNode::new(6u8), StringValueNode::new("USDC"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"amountNumberDisplayNode","decimals":{"kind":"integerValueNode","value":"6"},"unit":{"kind":"stringValueNode","string":"USDC"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"amountNumberDisplayNode","decimals":{"kind":"integerValueNode","value":"6"},"unit":{"kind":"stringValueNode","string":"USDC"}}"#;
        let node: AmountNumberDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AmountNumberDisplayNode::new(IntegerValueNode::new(6u8), StringValueNode::new("USDC"))
        );
    }

    #[test]
    fn from_json_without_unit() {
        let json = r#"{"kind":"amountNumberDisplayNode","decimals":{"kind":"integerValueNode","value":"9"}}"#;
        let node: AmountNumberDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(*node.unit, None);
        assert_eq!(serde_json::to_string(&node).unwrap(), json);
    }

    #[test]
    fn from_json_without_decimals() {
        let json = r#"{"kind":"amountNumberDisplayNode"}"#;
        let error = serde_json::from_str::<AmountNumberDisplayNode>(json).unwrap_err();
        assert!(error.to_string().contains("missing field `decimals`"));
    }
}
