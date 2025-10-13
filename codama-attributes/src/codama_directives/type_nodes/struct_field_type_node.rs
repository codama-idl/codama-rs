use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, Docs, StructFieldTypeNode, TypeNode, ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

// TODO: impl { partial_meta(meta) -> Result<Self, Vec<Meta>> } to reuse in other directives.

impl FromMeta for StructFieldTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("field")?.as_path_list()?;
        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut r#type = SetOnce::<TypeNode>::new("type");
        let mut default_value = SetOnce::<ValueNode>::new("default_value");
        let mut default_value_strategy =
            SetOnce::<DefaultValueStrategy>::new("default_value_strategy");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(String::from_meta(meta)?.into(), meta),
            "type" => {
                let node = TypeNode::from_meta(&meta.as_path_value()?.value)?;
                r#type.set(node, meta)
            }
            "default_value" => {
                let node = ValueNode::from_meta(&meta.as_path_value()?.value)?;
                default_value.set(node, meta)
            }
            "default_value_omitted" => {
                meta.as_path()?;
                default_value_strategy.set(DefaultValueStrategy::Omitted, meta)
            }
            _ => {
                if let Ok(value) = String::from_meta(meta) {
                    return name.set(value.into(), meta);
                }
                if let Ok(node) = TypeNode::from_meta(meta) {
                    return r#type.set(node, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;
        Ok(StructFieldTypeNode {
            name: name.take(meta)?,
            r#type: r#type.take(meta)?,
            default_value: default_value.option(),
            default_value_strategy: default_value_strategy.option(),
            docs: Docs::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use codama_nodes::{NumberFormat::U32, NumberTypeNode, NumberValueNode};

    use super::*;
    use crate::{assert_type, assert_type_err};

    #[test]
    fn implicit_minimum() {
        assert_type!(
            { field("age", number(u32)) },
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn explicit_minimum() {
        assert_type!(
            { field(name = "age", type = number(u32)) },
            StructFieldTypeNode::new("age", NumberTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn with_default_value() {
        assert_type!(
            { field("age", number(u32), default_value = 42) },
            StructFieldTypeNode {
                default_value: Some(NumberValueNode::new(42u32).into()),
                ..StructFieldTypeNode::new("age", NumberTypeNode::le(U32))
            }
            .into()
        );
    }

    #[test]
    fn with_default_value_strategy() {
        assert_type!(
            {
                field(
                    "age",
                    number(u32),
                    default_value = 42,
                    default_value_omitted,
                )
            },
            StructFieldTypeNode {
                default_value: Some(NumberValueNode::new(42u32).into()),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                ..StructFieldTypeNode::new("age", NumberTypeNode::le(U32))
            }
            .into()
        );
    }

    #[test]
    fn missing_name() {
        assert_type_err!({ field(type = number(u8)) }, "name is missing");
    }

    #[test]
    fn missing_type() {
        assert_type_err!({ field(name = "age") }, "type is missing");
    }

    #[test]
    fn invalid_name() {
        assert_type_err!({ field(name = unrecognized) }, "expected a string");
    }

    #[test]
    fn invalid_type() {
        assert_type_err!({ field(type = unrecognized) }, "unrecognized type");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ field(unrecognized) }, "unrecognized attribute");
        assert_type_err!({ field(foo = 42) }, "unrecognized attribute");
    }
}
