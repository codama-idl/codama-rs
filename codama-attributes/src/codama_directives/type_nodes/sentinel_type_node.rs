use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{ConstantValueNode, SentinelTypeNode, TypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for SentinelTypeNode<TypeNode> {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("sentinel")?.as_path_list()?;
        let mut r#type: SetOnce<TypeNode> = SetOnce::new("type");
        let mut sentinel: SetOnce<ConstantValueNode> = SetOnce::new("sentinel");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "type" => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            "sentinel" => sentinel.set(ConstantValueNode::from_meta(meta.as_value()?)?, meta),
            "constant" => sentinel.set(ConstantValueNode::from_meta(meta)?, meta),
            _ => {
                if meta.is_path_or_list() {
                    return r#type.set(TypeNode::from_meta(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(SentinelTypeNode::new(
            r#type.take(meta)?,
            sentinel.take(meta)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberFormat::U8, NumberTypeNode, NumberValueNode, StringTypeNode};

    fn sentinel_255() -> ConstantValueNode {
        ConstantValueNode::new(NumberTypeNode::le(U8), NumberValueNode::new(255u64))
    }

    #[test]
    fn implicit() {
        assert_type!(
            { sentinel(string, constant(number(u8), 255)) },
            SentinelTypeNode::new(StringTypeNode::utf8(), sentinel_255()).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { sentinel(type = string, sentinel = constant(number(u8), 255)) },
            SentinelTypeNode::new(StringTypeNode::utf8(), sentinel_255()).into()
        );
    }

    #[test]
    fn sentinel_missing() {
        assert_type_err!({ sentinel(string) }, "sentinel is missing");
    }

    #[test]
    fn type_missing() {
        assert_type_err!({ sentinel(constant(number(u8), 255)) }, "type is missing");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ sentinel(foo = 42) }, "unrecognized attribute");
    }
}
