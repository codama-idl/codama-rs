use crate::{
    CamelCaseString, Docs, EnumVariantTypeNode, HasKind, Node, NodeTrait, RegisteredTypeNode,
    TypeNode,
};
use codama_errors::CodamaError;

impl EnumVariantTypeNode {
    pub fn new<T>(identifier: T) -> Self
    where
        T: Into<CamelCaseString>,
    {
        Self {
            identifier: identifier.into(),
            discriminator: None,
            docs: Docs::default(),
            data: Box::new(None),
            display: None,
            plugins: vec![],
        }
    }

    pub fn with_data<T, U>(identifier: T, data: U) -> Self
    where
        T: Into<CamelCaseString>,
        U: Into<TypeNode>,
    {
        Self {
            identifier: identifier.into(),
            discriminator: None,
            docs: Docs::default(),
            data: Box::new(Some(data.into())),
            display: None,
            plugins: vec![],
        }
    }
}

impl TryFrom<Node> for EnumVariantTypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Type(RegisteredTypeNode::EnumVariant(node)) => Ok(node),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: Self::KIND.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        IntegerTypeNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, TupleTypeNode, U32,
        U64,
    };

    #[test]
    fn new() {
        let node = EnumVariantTypeNode::new("my_variant");
        assert_eq!(node.identifier, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, None);
        assert_eq!(*node.data, None);
    }

    #[test]
    fn with_data_struct() {
        let r#struct = StructTypeNode::new(vec![
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
            StructFieldTypeNode::new("name", StringTypeNode::utf8()),
        ]);
        let node = EnumVariantTypeNode::with_data("my_variant", r#struct.clone());
        assert_eq!(node.identifier, CamelCaseString::new("myVariant"));
        assert_eq!(*node.data, Some(TypeNode::Struct(r#struct)));
    }

    #[test]
    fn with_data_tuple() {
        let tuple = TupleTypeNode::new(vec![
            IntegerTypeNode::le(U32).into(),
            StringTypeNode::utf8().into(),
        ]);
        let node = EnumVariantTypeNode::with_data("my_variant", tuple.clone());
        assert_eq!(*node.data, Some(TypeNode::Tuple(tuple)));
    }

    #[test]
    fn with_data_any_type() {
        let node = EnumVariantTypeNode::with_data("amount", IntegerTypeNode::le(U64));
        assert_eq!(
            *node.data,
            Some(TypeNode::Integer(IntegerTypeNode::le(U64)))
        );
    }

    #[test]
    fn direct_instantiation() {
        let node = EnumVariantTypeNode {
            identifier: "my_variant".into(),
            discriminator: Some(42),
            docs: Docs::default(),
            data: Box::new(Some(TupleTypeNode::new(vec![]).into())),
            display: None,
            plugins: vec![],
        };
        assert_eq!(node.identifier, CamelCaseString::new("myVariant"));
        assert_eq!(node.discriminator, Some(42));
        assert_eq!(
            *node.data,
            Some(TypeNode::Tuple(TupleTypeNode::new(vec![])))
        );
    }

    #[test]
    fn to_json() {
        let node = EnumVariantTypeNode::new("myVariant");
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumVariantTypeNode","identifier":"myVariant"}"#
        );
    }

    #[test]
    fn from_json() {
        let json = r#"{"kind":"enumVariantTypeNode","identifier":"myVariant"}"#;
        let node: EnumVariantTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(node, EnumVariantTypeNode::new("myVariant"));
    }

    #[test]
    fn to_json_with_data() {
        let node = EnumVariantTypeNode {
            discriminator: Some(3),
            ..EnumVariantTypeNode::with_data("myVariant", StructTypeNode::new(vec![]))
        };
        let json = serde_json::to_string(&node).unwrap();
        assert_eq!(
            json,
            r#"{"kind":"enumVariantTypeNode","identifier":"myVariant","discriminator":3,"data":{"kind":"structTypeNode"}}"#
        );
    }

    #[test]
    fn from_json_with_data() {
        let json = r#"{"kind":"enumVariantTypeNode","identifier":"myVariant","data":{"kind":"integerTypeNode","format":"u64","endian":"le"}}"#;
        let node: EnumVariantTypeNode = serde_json::from_str(json).unwrap();
        assert_eq!(
            node,
            EnumVariantTypeNode::with_data("myVariant", IntegerTypeNode::le(U64))
        );
    }

    #[test]
    fn try_from_node() {
        let node: Node = EnumVariantTypeNode::new("myVariant").into();
        assert_eq!(
            EnumVariantTypeNode::try_from(node).unwrap(),
            EnumVariantTypeNode::new("myVariant")
        );
    }

    #[test]
    fn try_from_node_err() {
        let node: Node = StringTypeNode::utf8().into();
        let result = EnumVariantTypeNode::try_from(node);
        assert!(matches!(
            result,
            Err(CodamaError::InvalidNodeConversion { from, into }) if from == "stringTypeNode" && into == "enumVariantTypeNode"
        ));
    }
}
