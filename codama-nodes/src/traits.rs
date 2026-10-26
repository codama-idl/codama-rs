use crate::{CamelCaseString, TransformNode};
use codama_errors::CodamaResult;
use std::fmt::Debug;

pub trait NodeTrait:
    HasKind + Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    const KIND: &'static str;

    fn to_json(&self) -> CodamaResult<String> {
        serde_json::to_string(&self).map_err(Into::into)
    }

    fn to_json_pretty(&self) -> CodamaResult<String> {
        serde_json::to_string_pretty(&self).map_err(Into::into)
    }

    fn from_json(json: &str) -> CodamaResult<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }
}

pub trait NodeUnionTrait:
    HasKind + Debug + PartialEq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>
{
}

pub trait HasKind {
    fn kind(&self) -> &'static str;
}

pub trait HasIdentifier {
    fn identifier(&self) -> &CamelCaseString;
}

/// Type nodes that carry transforms. Transforms apply in list order, the first is the innermost.
pub trait HasTransforms {
    fn transforms(&self) -> &[TransformNode];
    fn transforms_mut(&mut self) -> &mut Vec<TransformNode>;

    /// Appends a transform, so it applies after the transforms the node already has.
    fn add_transform<T>(mut self, transform: T) -> Self
    where
        Self: Sized,
        T: Into<TransformNode>,
    {
        self.transforms_mut().push(transform.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        FixedSizeTransformNode, IntegerTypeNode, SizePrefixTransformNode, StringTypeNode, U16,
    };

    #[test]
    fn add_transform() {
        let node = StringTypeNode::utf8().add_transform(FixedSizeTransformNode::new(10));
        assert_eq!(
            node.transforms,
            vec![TransformNode::FixedSize(FixedSizeTransformNode::new(10))]
        );
    }

    #[test]
    fn add_transform_appends_after_existing_transforms() {
        let node = StringTypeNode::utf8()
            .add_transform(SizePrefixTransformNode::new(IntegerTypeNode::le(U16)))
            .add_transform(FixedSizeTransformNode::new(10));
        assert_eq!(
            node.transforms(),
            [
                TransformNode::SizePrefix(SizePrefixTransformNode::new(IntegerTypeNode::le(U16))),
                TransformNode::FixedSize(FixedSizeTransformNode::new(10)),
            ]
        );
    }
}
