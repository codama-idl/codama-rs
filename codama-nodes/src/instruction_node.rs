#[cfg(test)]
mod tests {
    use crate::{
        AccountBumpValueNode, CamelCaseString, Docs, InjectedValueNode, InstructionNode,
        IntegerTypeNode, ProvidedNode, StructFieldTypeNode, StructTypeNode, U64, U8,
    };

    fn data() -> StructTypeNode {
        StructTypeNode::new(vec![StructFieldTypeNode::new(
            "amount",
            IntegerTypeNode::le(U64),
        )])
    }

    #[test]
    fn empty_instruction() {
        let node = InstructionNode {
            identifier: "myInstruction".into(),
            ..InstructionNode::default()
        };
        assert_eq!(node.identifier, CamelCaseString::new("myInstruction"));
        assert_eq!(node.docs, Docs::default());
        assert_eq!(node.optional_account_strategy, None);
        assert_eq!(node.accounts, vec![]);
        assert_eq!(*node.data, None);
        assert_eq!(*node.return_data, None);
        assert_eq!(node.remaining_accounts, vec![]);
        assert_eq!(node.byte_deltas, vec![]);
        assert_eq!(node.discriminators, vec![]);
        assert_eq!(node.status, None);
        assert_eq!(node.sub_instructions, vec![]);
    }

    #[test]
    fn instruction_with_sub_instructions() {
        let node = InstructionNode {
            identifier: "myInstruction".into(),
            sub_instructions: vec![
                InstructionNode {
                    identifier: "mySubInstructionA".into(),
                    ..InstructionNode::default()
                },
                InstructionNode {
                    identifier: "mySubInstructionB".into(),
                    ..InstructionNode::default()
                },
            ],
            ..InstructionNode::default()
        };
        assert_eq!(
            node.sub_instructions,
            vec![
                InstructionNode {
                    identifier: "mySubInstructionA".into(),
                    ..InstructionNode::default()
                },
                InstructionNode {
                    identifier: "mySubInstructionB".into(),
                    ..InstructionNode::default()
                },
            ]
        );
    }

    #[test]
    fn to_json() {
        let node = InstructionNode {
            identifier: "myInstruction".into(),
            ..InstructionNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionNode","identifier":"myInstruction"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"instructionNode","identifier":"myInstruction"}"#;
        let node: InstructionNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionNode {
                identifier: "myInstruction".into(),
                ..InstructionNode::default()
            }
        );
    }

    #[test]
    fn to_json_with_data_and_return_data() {
        let node = InstructionNode {
            identifier: "transfer".into(),
            data: Box::new(Some(data().into())),
            return_data: Box::new(Some(IntegerTypeNode::le(U64).into())),
            ..InstructionNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionNode","identifier":"transfer","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"amount","type":{"kind":"integerTypeNode","format":"u64","endian":"le"}}]},"returnData":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#
        );
    }

    #[test]
    fn from_json_with_data_and_return_data() {
        let json = r#"{"kind":"instructionNode","identifier":"transfer","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"amount","type":{"kind":"integerTypeNode","format":"u64","endian":"le"}}]},"returnData":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: InstructionNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            InstructionNode {
                identifier: "transfer".into(),
                data: Box::new(Some(data().into())),
                return_data: Box::new(Some(IntegerTypeNode::le(U64).into())),
                ..InstructionNode::default()
            }
        );
    }

    #[test]
    fn to_json_with_provides() {
        let bump = StructFieldTypeNode {
            default_value: Box::new(Some(InjectedValueNode::new("bump").into())),
            ..StructFieldTypeNode::new("bump", IntegerTypeNode::le(U8))
        };
        let node = InstructionNode {
            identifier: "createCounter".into(),
            data: Box::new(Some(StructTypeNode::new(vec![bump]).into())),
            provides: vec![ProvidedNode::new(
                "bump",
                AccountBumpValueNode::new("counter"),
            )],
            ..InstructionNode::default()
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"instructionNode","identifier":"createCounter","data":{"kind":"structTypeNode","fields":[{"kind":"structFieldTypeNode","identifier":"bump","type":{"kind":"integerTypeNode","format":"u8","endian":"le"},"defaultValue":{"kind":"injectedValueNode","key":"bump"}}]},"provides":[{"kind":"providedNode","identifier":"bump","node":{"kind":"accountBumpValueNode","identifier":"counter"}}]}"#
        );
    }

    #[test]
    fn from_json_with_provides() {
        let json = r#"{"kind":"instructionNode","identifier":"createCounter","provides":[{"kind":"providedNode","identifier":"bump","node":{"kind":"accountBumpValueNode","identifier":"counter"}}]}"#;
        let node: InstructionNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node.provides,
            vec![ProvidedNode::new(
                "bump",
                AccountBumpValueNode::new("counter")
            )]
        );
    }
}
