use crate::{
    ArrayTypeNode, BooleanTypeNode, BytesTypeNode, DateTimeTypeNode, DefinedTypeLinkNode,
    DurationTypeNode, EnumTypeNode, EnumVariantTypeNode, FixedPointTypeNode, FloatTypeNode,
    HasKind, HasTransforms, IntegerTypeNode, LinkNode, MapTypeNode, Node, OptionTypeNode,
    PublicKeyTypeNode, RemainderOptionTypeNode, SetTypeNode, StringTypeNode, StructFieldTypeNode,
    StructTypeNode, TransformNode, TupleTypeNode, ZeroableOptionTypeNode,
};
use codama_errors::CodamaError;
use codama_nodes_derive::node_union;

#[node_union]
pub enum RegisteredTypeNode {
    Array(ArrayTypeNode),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Duration(DurationTypeNode),
    Enum(EnumTypeNode),
    FixedPoint(FixedPointTypeNode),
    Float(FloatTypeNode),
    Integer(IntegerTypeNode),
    Map(MapTypeNode),
    Option(OptionTypeNode),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(RemainderOptionTypeNode),
    Set(SetTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(ZeroableOptionTypeNode),

    // Registered only.
    EnumVariant(EnumVariantTypeNode),
    StructField(StructFieldTypeNode),
}

#[node_union]
pub enum TypeNode {
    Array(ArrayTypeNode),
    Boolean(BooleanTypeNode),
    Bytes(BytesTypeNode),
    DateTime(DateTimeTypeNode),
    Duration(DurationTypeNode),
    Enum(EnumTypeNode),
    FixedPoint(FixedPointTypeNode),
    Float(FloatTypeNode),
    Integer(IntegerTypeNode),
    Map(MapTypeNode),
    Option(OptionTypeNode),
    PublicKey(PublicKeyTypeNode),
    RemainderOption(RemainderOptionTypeNode),
    Set(SetTypeNode),
    String(StringTypeNode),
    Struct(StructTypeNode),
    Tuple(TupleTypeNode),
    ZeroableOption(ZeroableOptionTypeNode),

    // Standalone only.
    Link(DefinedTypeLinkNode),
}

impl TryFrom<Node> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Type(node) => Self::try_from(node),
            Node::Link(LinkNode::DefinedType(node)) => Ok(Self::Link(node)),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "TypeNode".to_string(),
            }),
        }
    }
}

impl<T> TryFrom<Option<T>> for TypeNode
where
    T: TryInto<Self, Error = CodamaError>,
{
    type Error = CodamaError;

    fn try_from(node: Option<T>) -> Result<Self, Self::Error> {
        match node {
            Some(t) => t.try_into(),
            _ => Err(CodamaError::InvalidNodeConversion {
                from: "None".to_string(),
                into: "TypeNode".to_string(),
            }),
        }
    }
}

impl TryFrom<RegisteredTypeNode> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: RegisteredTypeNode) -> Result<Self, Self::Error> {
        match node {
            RegisteredTypeNode::Array(node) => Ok(Self::Array(node)),
            RegisteredTypeNode::Boolean(node) => Ok(Self::Boolean(node)),
            RegisteredTypeNode::Bytes(node) => Ok(Self::Bytes(node)),
            RegisteredTypeNode::DateTime(node) => Ok(Self::DateTime(node)),
            RegisteredTypeNode::Duration(node) => Ok(Self::Duration(node)),
            RegisteredTypeNode::Enum(node) => Ok(Self::Enum(node)),
            RegisteredTypeNode::FixedPoint(node) => Ok(Self::FixedPoint(node)),
            RegisteredTypeNode::Float(node) => Ok(Self::Float(node)),
            RegisteredTypeNode::Integer(node) => Ok(Self::Integer(node)),
            RegisteredTypeNode::Map(node) => Ok(Self::Map(node)),
            RegisteredTypeNode::Option(node) => Ok(Self::Option(node)),
            RegisteredTypeNode::PublicKey(node) => Ok(Self::PublicKey(node)),
            RegisteredTypeNode::RemainderOption(node) => Ok(Self::RemainderOption(node)),
            RegisteredTypeNode::Set(node) => Ok(Self::Set(node)),
            RegisteredTypeNode::String(node) => Ok(Self::String(node)),
            RegisteredTypeNode::Struct(node) => Ok(Self::Struct(node)),
            RegisteredTypeNode::Tuple(node) => Ok(Self::Tuple(node)),
            RegisteredTypeNode::ZeroableOption(node) => Ok(Self::ZeroableOption(node)),
            RegisteredTypeNode::EnumVariant(_) | RegisteredTypeNode::StructField(_) => {
                Err(CodamaError::InvalidNodeConversion {
                    from: node.kind().to_string(),
                    into: "TypeNode".to_string(),
                })
            }
        }
    }
}

impl TryFrom<TypeNode> for RegisteredTypeNode {
    type Error = CodamaError;

    fn try_from(node: TypeNode) -> Result<Self, Self::Error> {
        match node {
            TypeNode::Array(node) => Ok(Self::Array(node)),
            TypeNode::Boolean(node) => Ok(Self::Boolean(node)),
            TypeNode::Bytes(node) => Ok(Self::Bytes(node)),
            TypeNode::DateTime(node) => Ok(Self::DateTime(node)),
            TypeNode::Duration(node) => Ok(Self::Duration(node)),
            TypeNode::Enum(node) => Ok(Self::Enum(node)),
            TypeNode::FixedPoint(node) => Ok(Self::FixedPoint(node)),
            TypeNode::Float(node) => Ok(Self::Float(node)),
            TypeNode::Integer(node) => Ok(Self::Integer(node)),
            TypeNode::Map(node) => Ok(Self::Map(node)),
            TypeNode::Option(node) => Ok(Self::Option(node)),
            TypeNode::PublicKey(node) => Ok(Self::PublicKey(node)),
            TypeNode::RemainderOption(node) => Ok(Self::RemainderOption(node)),
            TypeNode::Set(node) => Ok(Self::Set(node)),
            TypeNode::String(node) => Ok(Self::String(node)),
            TypeNode::Struct(node) => Ok(Self::Struct(node)),
            TypeNode::Tuple(node) => Ok(Self::Tuple(node)),
            TypeNode::ZeroableOption(node) => Ok(Self::ZeroableOption(node)),
            TypeNode::Link(_) => Err(CodamaError::InvalidNodeConversion {
                from: node.kind().to_string(),
                into: "RegisteredTypeNode".to_string(),
            }),
        }
    }
}

impl HasTransforms for TypeNode {
    fn transforms(&self) -> &[TransformNode] {
        match self {
            TypeNode::Array(node) => node.transforms(),
            TypeNode::Boolean(node) => node.transforms(),
            TypeNode::Bytes(node) => node.transforms(),
            TypeNode::DateTime(node) => node.transforms(),
            TypeNode::Duration(node) => node.transforms(),
            TypeNode::Enum(node) => node.transforms(),
            TypeNode::FixedPoint(node) => node.transforms(),
            TypeNode::Float(node) => node.transforms(),
            TypeNode::Integer(node) => node.transforms(),
            TypeNode::Map(node) => node.transforms(),
            TypeNode::Option(node) => node.transforms(),
            TypeNode::PublicKey(node) => node.transforms(),
            TypeNode::RemainderOption(node) => node.transforms(),
            TypeNode::Set(node) => node.transforms(),
            TypeNode::String(node) => node.transforms(),
            TypeNode::Struct(node) => node.transforms(),
            TypeNode::Tuple(node) => node.transforms(),
            TypeNode::ZeroableOption(node) => node.transforms(),
            TypeNode::Link(node) => node.transforms(),
        }
    }

    fn transforms_mut(&mut self) -> &mut Vec<TransformNode> {
        match self {
            TypeNode::Array(node) => node.transforms_mut(),
            TypeNode::Boolean(node) => node.transforms_mut(),
            TypeNode::Bytes(node) => node.transforms_mut(),
            TypeNode::DateTime(node) => node.transforms_mut(),
            TypeNode::Duration(node) => node.transforms_mut(),
            TypeNode::Enum(node) => node.transforms_mut(),
            TypeNode::FixedPoint(node) => node.transforms_mut(),
            TypeNode::Float(node) => node.transforms_mut(),
            TypeNode::Integer(node) => node.transforms_mut(),
            TypeNode::Map(node) => node.transforms_mut(),
            TypeNode::Option(node) => node.transforms_mut(),
            TypeNode::PublicKey(node) => node.transforms_mut(),
            TypeNode::RemainderOption(node) => node.transforms_mut(),
            TypeNode::Set(node) => node.transforms_mut(),
            TypeNode::String(node) => node.transforms_mut(),
            TypeNode::Struct(node) => node.transforms_mut(),
            TypeNode::Tuple(node) => node.transforms_mut(),
            TypeNode::ZeroableOption(node) => node.transforms_mut(),
            TypeNode::Link(node) => node.transforms_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FixedSizeTransformNode, SizePrefixTransformNode, U32};

    #[test]
    fn kind_from_standalone() {
        let node: TypeNode = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }

    #[test]
    fn kind_from_registered() {
        let node: RegisteredTypeNode = StringTypeNode::utf8().into();
        assert_eq!(node.kind(), "stringTypeNode");
    }

    #[test]
    fn try_from_node_ok() {
        let node: Node = StringTypeNode::utf8().into();
        let result = TypeNode::try_from(node);
        assert!(matches!(result, Ok(TypeNode::String(_))));
    }

    #[test]
    fn try_from_node_err() {
        let node: Node = StructFieldTypeNode::new("foo", StringTypeNode::utf8()).into();
        let result = TypeNode::try_from(node);
        assert!(matches!(
            result,
            Err(CodamaError::InvalidNodeConversion { from, into }) if from == "structFieldTypeNode" && into == "TypeNode"
        ));
    }

    #[test]
    fn number_like_variants_round_trip() {
        for json in [
            r#"{"kind":"durationTypeNode","number":{"kind":"integerTypeNode","format":"u64"}}"#,
            r#"{"kind":"fixedPointTypeNode","scale":2,"base":2,"number":{"kind":"integerTypeNode","format":"u64"}}"#,
            r#"{"kind":"floatTypeNode","format":"f64"}"#,
            r#"{"kind":"integerTypeNode","format":"u32","endian":"le"}"#,
        ] {
            let node: TypeNode = serde_json::from_str(json).unwrap();
            assert_eq!(serde_json::to_string(&node).unwrap(), json);
        }
    }

    #[test]
    fn transforms() {
        let node: TypeNode = StringTypeNode::utf8()
            .add_transform(FixedSizeTransformNode::new(10))
            .into();
        assert_eq!(
            node.transforms(),
            [TransformNode::FixedSize(FixedSizeTransformNode::new(10))]
        );
    }

    #[test]
    fn add_transform() {
        let node =
            TypeNode::from(IntegerTypeNode::le(U32)).add_transform(FixedSizeTransformNode::new(8));
        assert_eq!(
            node,
            TypeNode::Integer(IntegerTypeNode {
                transforms: vec![FixedSizeTransformNode::new(8).into()],
                ..IntegerTypeNode::le(U32)
            })
        );
    }

    #[test]
    fn link_with_transform() {
        let node = TypeNode::from(DefinedTypeLinkNode::new("myType"))
            .add_transform(FixedSizeTransformNode::new(32));
        assert_eq!(
            serde_json::to_string(&node).unwrap(),
            r#"{"kind":"definedTypeLinkNode","identifier":"myType","transforms":[{"kind":"fixedSizeTransformNode","size":32}]}"#
        );
    }

    #[test]
    fn transforms_mut() {
        let mut node: TypeNode = StringTypeNode::utf8()
            .add_transform(FixedSizeTransformNode::new(10))
            .into();
        node.transforms_mut()[0] = SizePrefixTransformNode::new(IntegerTypeNode::le(U32)).into();
        assert_eq!(
            node,
            TypeNode::String(
                StringTypeNode::utf8()
                    .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U32)))
            )
        );
    }
}
