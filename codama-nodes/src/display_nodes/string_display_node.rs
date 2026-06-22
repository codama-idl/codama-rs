use crate::StringDisplayNode;

impl StringDisplayNode {
    pub fn new(slice_start: Option<u64>, slice_end: Option<u64>) -> Self {
        Self {
            slice_start,
            slice_end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let node = StringDisplayNode::new(Some(2), Some(6));
        assert_eq!(node.slice_start, Some(2));
        assert_eq!(node.slice_end, Some(6));
    }

    #[test]
    fn default() {
        let node = StringDisplayNode::default();
        assert_eq!(node.slice_start, None);
        assert_eq!(node.slice_end, None);
    }

    #[test]
    fn to_json() {
        let node = StringDisplayNode::new(Some(2), Some(6));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"stringDisplayNode","sliceStart":2,"sliceEnd":6}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"stringDisplayNode","sliceStart":2,"sliceEnd":6}"#;
        let node: StringDisplayNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, StringDisplayNode::new(Some(2), Some(6)));
    }
}
