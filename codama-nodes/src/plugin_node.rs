use crate::{Namespace, PluginNode};
use serde_json::Value;

impl PluginNode {
    pub fn new<T: Into<Namespace>>(namespace: T) -> Self {
        Self {
            namespace: namespace.into(),
            payload: None,
            plugins: vec![],
        }
    }

    pub fn with_payload<T: Into<Namespace>>(namespace: T, payload: Value) -> Self {
        Self {
            namespace: namespace.into(),
            payload: Some(payload),
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn namespace(value: &str) -> Namespace {
        value.parse().unwrap()
    }

    #[test]
    fn new() {
        let node = PluginNode::new(namespace("anchor"));
        assert_eq!(node.namespace, namespace("anchor"));
        assert_eq!(node.payload, None);
    }

    #[test]
    fn with_payload() {
        let node = PluginNode::with_payload(namespace("anchor"), json!({ "version": "0.30.0" }));
        assert_eq!(node.namespace, namespace("anchor"));
        assert_eq!(node.payload, Some(json!({ "version": "0.30.0" })));
    }

    #[test]
    fn to_json() {
        let node = PluginNode::new(namespace("anchor"));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(json, r#"{"kind":"pluginNode","namespace":"anchor"}"#);
    }

    #[test]
    fn to_json_with_payload() {
        let node = PluginNode::with_payload(namespace("i18n.es"), json!({ "label": "Cantidad" }));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"pluginNode","namespace":"i18n.es","payload":{"label":"Cantidad"}}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"pluginNode","namespace":"anchor"}"#;
        let node: PluginNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, PluginNode::new(namespace("anchor")));
    }

    #[test]
    fn from_json_with_payload() {
        let json = r#"{"kind":"pluginNode","namespace":"i18n.es","payload":{"label":"Cantidad"}}"#;
        let node: PluginNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            PluginNode::with_payload(namespace("i18n.es"), json!({ "label": "Cantidad" }))
        );
    }

    #[test]
    fn from_json_rejects_an_invalid_namespace() {
        let json = r#"{"kind":"pluginNode","namespace":"codama-rs"}"#;
        let error = serde_json::from_str::<PluginNode>(json).unwrap_err();
        assert_eq!(
            error.to_string(),
            "invalid value: string \"codama-rs\", expected a namespace string at line 1 column 44"
        );
    }
}
