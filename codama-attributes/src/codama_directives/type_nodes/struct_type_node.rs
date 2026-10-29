use crate::utils::FromMeta;
use codama_errors::IteratorCombineErrors;
use codama_nodes::{StructFieldTypeNode, StructTypeNode};
use codama_syn_helpers::{extensions::*, Meta};
use std::collections::HashSet;

impl FromMeta for StructTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("struct")?;
        if meta.is_path_or_empty_list() {
            return Ok(StructTypeNode::default());
        }

        let metas = meta.as_path_list()?.parse_metas()?;
        let fields = metas
            .iter()
            .map(|meta| match meta {
                Meta::PathList(pl) if pl.path.is_strict("field") => {
                    StructFieldTypeNode::from_meta(meta)
                }
                _ => Err(meta.error("expected field(...) attribute")),
            })
            .collect_and_combine_errors()?;
        check_duplicate_identifiers(&metas, &fields)?;

        Ok(StructTypeNode::new(fields))
    }
}

/// The spec forbids two fields of one struct with one identifier, also when only their casing differs.
fn check_duplicate_identifiers(metas: &[Meta], fields: &[StructFieldTypeNode]) -> syn::Result<()> {
    let mut identifiers = HashSet::new();
    let duplicate = metas
        .iter()
        .zip(fields)
        .find(|(_, field)| !identifiers.insert(&field.identifier));
    match duplicate {
        Some((meta, field)) => Err(meta.error(format!(
            "Field `{}` is not unique in `struct(...)`",
            field.identifier.as_str()
        ))),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{IntegerFormat::U32, IntegerTypeNode, StringTypeNode};

    #[test]
    fn empty() {
        assert_type!(
            { struct },
            StructTypeNode::default().into()
        );
    }

    #[test]
    fn empty_with_braces() {
        assert_type!(
            { struct() },
            StructTypeNode::default().into()
        );
    }

    #[test]
    fn with_one_field() {
        assert_type!(
            { struct(field("age", integer(u32))) },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U32))
            ]).into()
        );
    }

    #[test]
    fn with_multiple_fields() {
        assert_type!(
            { struct(field("age", integer(u32)), field("name", string(utf8))) },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8())
            ]).into()
        );
    }

    #[test]
    fn with_brackets() {
        assert_type!(
            { struct[field("age", integer(u32)), field("name", string(utf8))] },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", IntegerTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8())
            ]).into()
        );
    }

    #[test]
    fn invalid_fields_inside() {
        assert_type_err!({ struct(boolean, integer(u32)) }, "expected field(...) attribute");
    }

    #[test]
    fn duplicate_field_identifiers() {
        assert_type_err!(
            { struct(field("owner", public_key), field("owner", integer(u64))) },
            "Field `owner` is not unique in `struct(...)`"
        );
        assert_type_err!(
            { struct(field("owner_key", public_key), field("ownerKey", integer(u64))) },
            "Field `ownerKey` is not unique in `struct(...)`"
        );
    }
}
