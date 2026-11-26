use crate::utils::{parse_integer_type, FromMeta, SetOnce};
use codama_nodes::{BooleanTypeNode, IntegerTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for BooleanTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("boolean")?;
        let mut size: SetOnce<IntegerTypeNode> = SetOnce::new("size");
        if meta.is_path_or_empty_list() {
            return Ok(BooleanTypeNode::default());
        }

        meta.as_path_list()?
            .each(|ref meta| match meta.path_str().as_str() {
                "size" => size.set(parse_integer_type(meta.as_value()?, "size")?, meta),
                _ => {
                    if meta.is_path_or_list() {
                        return size.set(parse_integer_type(meta, "size")?, meta);
                    }
                    Err(meta.error("unrecognized attribute"))
                }
            })?;

        Ok(BooleanTypeNode::new(size.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{assert_type, assert_type_err};
    use codama_nodes::IntegerFormat::U32;

    #[test]
    fn default() {
        assert_type!({ boolean }, BooleanTypeNode::default().into());
        assert_type!({ boolean() }, BooleanTypeNode::default().into());
    }

    #[test]
    fn implicit() {
        assert_type!(
            { boolean(integer(u32)) },
            BooleanTypeNode::new(IntegerTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { boolean(size = integer(u32)) },
            BooleanTypeNode::new(IntegerTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!({ boolean(unrecognized) }, "unrecognized type");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ boolean(foo = 42) }, "unrecognized attribute");
    }
}
