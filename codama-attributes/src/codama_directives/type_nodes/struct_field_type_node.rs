use crate::{
    codama_directives::type_nodes::struct_field_meta_consumer::StructFieldMetaConsumer,
    utils::{FromMeta, MetaConsumer},
    DefaultValueDirective,
};
use codama_nodes::{Docs, StructFieldTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for StructFieldTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("field")?;
        let consumer = StructFieldMetaConsumer::from_meta(meta)?
            .consume_field()?
            .consume_default_value(DefaultValueDirective::parse_value_nodes_only)?
            .assert_fully_consumed()?;

        let default_value_strategy = consumer.default_value_strategy();
        let default_value = consumer
            .default_value_node()
            .map(|r| r.try_into_resolved().map_err(|e| meta.error(e.to_string())))
            .transpose()?;

        // When used as a type node (e.g., `type = field("age", integer(u8))`),
        // unresolved directives must already be resolved.
        let r#type = consumer
            .r#type
            .take(meta)?
            .try_into_resolved()
            .map_err(|e| meta.error(e.to_string()))?;

        Ok(StructFieldTypeNode {
            identifier: consumer.name.take(meta)?,
            r#type: Box::new(r#type),
            default_value: Box::new(default_value),
            default_value_strategy,
            docs: Docs::default(),
            display: None,
            plugins: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        DefaultValueStrategy, IntegerFormat::U32, IntegerTypeNode, IntegerValueNode,
    };

    #[test]
    fn implicit_minimum() {
        assert_type!(
            { field("age", integer(u32)) },
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn explicit_minimum() {
        assert_type!(
            { field(name = "age", type = integer(u32)) },
            StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)).into()
        );
    }

    #[test]
    fn with_default_value() {
        assert_type!(
            { field("age", integer(u32), default_value = 42) },
            StructFieldTypeNode {
                default_value: Box::new(Some(IntegerValueNode::new(42u32).into())),
                ..StructFieldTypeNode::new("age", IntegerTypeNode::le(U32))
            }
            .into()
        );
    }

    #[test]
    fn with_omitted_value() {
        assert_type!(
            { field("age", integer(u32), value = 42) },
            StructFieldTypeNode {
                default_value: Box::new(Some(IntegerValueNode::new(42u32).into())),
                default_value_strategy: Some(DefaultValueStrategy::Omitted),
                ..StructFieldTypeNode::new("age", IntegerTypeNode::le(U32))
            }
            .into()
        );
    }

    #[test]
    fn contextual_default_value() {
        assert_type_err!(
            { field("owner", public_key, default_value = payer) },
            "unrecognized value"
        );
    }

    #[test]
    fn missing_name() {
        assert_type_err!({ field(type = integer(u8)) }, "name is missing");
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
        assert_type_err!({ field("age", unrecognized) }, "unrecognized type");
        assert_type_err!(
            { field("len", number(u8)) },
            "number(...) is removed in v2, use integer(...) or float(...)"
        );
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ field(foo = 42) }, "unrecognized attribute");
    }
}
