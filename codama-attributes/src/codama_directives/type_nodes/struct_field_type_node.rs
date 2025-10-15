use crate::{
    codama_directives::type_nodes::struct_field_meta_consumer::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
};
use codama_nodes::{Docs, StructFieldTypeNode};
use codama_syn_helpers::Meta;

impl FromMeta for StructFieldTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("field")?;
        let consumer = StructFieldMetaConsumer::from_meta(meta)?
            .consume_field()?
            .consume_default_value()?
            .assert_fully_consumed()?;

        Ok(StructFieldTypeNode {
            name: consumer.name.take(meta)?,
            r#type: consumer.r#type.take(meta)?,
            default_value: consumer.default_value.option(),
            default_value_strategy: consumer.default_value_strategy.option(),
            docs: Docs::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{DefaultValueStrategy, NumberFormat::U32, NumberTypeNode, NumberValueNode};

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
