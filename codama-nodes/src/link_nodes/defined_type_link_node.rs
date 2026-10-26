use crate::{CamelCaseString, DefinedTypeLinkNode, ProgramLinkNode};

impl DefinedTypeLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: None,
            transforms: vec![],
            plugins: vec![],
        }
    }

    pub fn new_from_program<T>(identifier: T, program: ProgramLinkNode) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

impl From<String> for DefinedTypeLinkNode {
    fn from(identifier: String) -> Self {
        Self::new(identifier)
    }
}

impl From<&str> for DefinedTypeLinkNode {
    fn from(identifier: &str) -> Self {
        Self::new(identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedSizeTransformNode, HasTransforms, TransformNode};

    #[test]
    fn new() {
        let node = DefinedTypeLinkNode::new("my_type");
        assert_eq!(node.identifier, CamelCaseString::new("myType"));
    }

    #[test]
    fn new_from_program() {
        let node =
            DefinedTypeLinkNode::new_from_program("my_type", ProgramLinkNode::new("my_program"));
        assert_eq!(node.identifier, CamelCaseString::new("myType"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
    }

    #[test]
    fn from_string() {
        let node: DefinedTypeLinkNode = String::from("my_type").into();
        assert_eq!(node.identifier, CamelCaseString::new("myType"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn from_str() {
        let node: DefinedTypeLinkNode = "my_type".into();
        assert_eq!(node.identifier, CamelCaseString::new("myType"));
        assert_eq!(node.program, None);
    }

    #[test]
    fn to_json() {
        let node = DefinedTypeLinkNode::new("myType");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeLinkNode","identifier":"myType"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"definedTypeLinkNode","identifier":"myType"}"#;
        let node: DefinedTypeLinkNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, DefinedTypeLinkNode::new("myType"));
    }

    #[test]
    fn add_transform() {
        let node =
            DefinedTypeLinkNode::new("myType").add_transform(FixedSizeTransformNode::new(32));
        assert_eq!(
            node.transforms(),
            [TransformNode::FixedSize(FixedSizeTransformNode::new(32))]
        );
    }

    #[test]
    fn to_json_with_transforms() {
        let node =
            DefinedTypeLinkNode::new("myType").add_transform(FixedSizeTransformNode::new(32));
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"definedTypeLinkNode","identifier":"myType","transforms":[{"kind":"fixedSizeTransformNode","size":32}]}"#
        );
    }
}
