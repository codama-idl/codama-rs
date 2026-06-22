use crate::DateTimeNumberDisplayNode;

impl DateTimeNumberDisplayNode {
    pub fn new(ticks_per_second: Option<u64>) -> Self {
        Self { ticks_per_second }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = DateTimeNumberDisplayNode::new(Some(1000));
        assert_eq!(node.ticks_per_second, Some(1000));
    }

    #[test]
    fn default() {
        let node = DateTimeNumberDisplayNode::default();
        assert_eq!(node.ticks_per_second, None);
    }

    #[test]
    fn to_json() {
        let node = DateTimeNumberDisplayNode::new(Some(1000));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"dateTimeNumberDisplayNode","ticksPerSecond":1000}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"dateTimeNumberDisplayNode","ticksPerSecond":1000}"#;
        let node: DateTimeNumberDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DateTimeNumberDisplayNode::new(Some(1000)));
    }
}
