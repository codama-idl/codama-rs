use crate::{AmountNumberDisplayNode, InjectableNumberValueNode, InjectableStringValueNode};

impl AmountNumberDisplayNode {
    pub fn new<T, U>(decimals: T, unit: U) -> Self
    where
        T: Into<InjectableNumberValueNode>,
        U: Into<InjectableStringValueNode>,
    {
        Self {
            decimals: Box::new(Some(decimals.into())),
            unit: Box::new(Some(unit.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumberValueNode, StringValueNode};

    #[test]
    fn new() {
        let node =
            AmountNumberDisplayNode::new(NumberValueNode::new(6), StringValueNode::new("USDC"));
        assert_eq!(
            *node.decimals,
            Some(InjectableNumberValueNode::Number(NumberValueNode::new(6)))
        );
        assert_eq!(
            *node.unit,
            Some(InjectableStringValueNode::String(StringValueNode::new(
                "USDC"
            )))
        );
    }

    #[test]
    fn default() {
        let node = AmountNumberDisplayNode::default();
        assert_eq!(*node.decimals, None);
        assert_eq!(*node.unit, None);
    }

    #[test]
    fn to_json() {
        let node =
            AmountNumberDisplayNode::new(NumberValueNode::new(6), StringValueNode::new("USDC"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"amountNumberDisplayNode","decimals":{"kind":"numberValueNode","number":6},"unit":{"kind":"stringValueNode","string":"USDC"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"amountNumberDisplayNode","decimals":{"kind":"numberValueNode","number":6},"unit":{"kind":"stringValueNode","string":"USDC"}}"#;
        let node: AmountNumberDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            AmountNumberDisplayNode::new(NumberValueNode::new(6u32), StringValueNode::new("USDC"))
        );
    }
}
