use crate::{CamelCaseString, PluginNode};
use serde_json::Value;

impl PluginNode {
    pub fn new<T: Into<CamelCaseString>>(name: T) -> Self {
        Self {
            name: name.into(),
            payload: None,
        }
    }

    pub fn with_payload<T: Into<CamelCaseString>>(name: T, payload: Value) -> Self {
        Self {
            name: name.into(),
            payload: Some(payload),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new() {
        let node = PluginNode::new("anchor");
        assert_eq!(node.name, CamelCaseString::new("anchor"));
        assert_eq!(node.payload, None);
    }

    #[test]
    fn with_payload() {
        let node = PluginNode::with_payload("anchor", json!({ "version": "0.30.0" }));
        assert_eq!(node.name, CamelCaseString::new("anchor"));
        assert_eq!(node.payload, Some(json!({ "version": "0.30.0" })));
    }

    #[test]
    fn to_json() {
        let node = PluginNode::new("anchor");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pluginNode","name":"anchor"}"#);
    }

    #[test]
    fn to_json_with_payload() {
        let node = PluginNode::with_payload("anchor", json!({ "version": "0.30.0" }));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pluginNode","name":"anchor","payload":{"version":"0.30.0"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pluginNode","name":"anchor"}"#;
        let node: PluginNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PluginNode::new("anchor"));
    }

    #[test]
    fn from_json_with_payload() {
        let json = r#"{"kind":"pluginNode","name":"anchor","payload":{"version":"0.30.0"}}"#;
        let node: PluginNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PluginNode::with_payload("anchor", json!({ "version": "0.30.0" }))
        );
    }
}
