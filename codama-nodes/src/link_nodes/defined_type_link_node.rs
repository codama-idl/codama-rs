use crate::{DefinedTypeLinkNode, Identifier, ProgramLinkNode};

impl DefinedTypeLinkNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<Identifier>,
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
        T: Into<Identifier>,
    {
        Self {
            identifier: identifier.into(),
            program: Some(program),
            transforms: vec![],
            plugins: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedSizeTransformNode, HasTransforms, Identifier, TransformNode};

    #[test]
    fn new() {
        let node = DefinedTypeLinkNode::new("myType");
        assert_eq!(node.identifier, Identifier::from("myType"));
    }

    #[test]
    fn new_from_program() {
        let node =
            DefinedTypeLinkNode::new_from_program("myType", ProgramLinkNode::new("myProgram"));
        assert_eq!(node.identifier, Identifier::from("myType"));
        assert_eq!(node.program, Some(ProgramLinkNode::new("myProgram")));
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
