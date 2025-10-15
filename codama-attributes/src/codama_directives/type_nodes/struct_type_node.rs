use crate::utils::FromMeta;
use codama_errors::IteratorCombineErrors;
use codama_nodes::{StructFieldTypeNode, StructTypeNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for StructTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("struct")?;
        if meta.is_path_or_empty_list() {
            return Ok(StructTypeNode::default());
        }

        let fields = meta
            .as_path_list()?
            .parse_metas()?
            .into_iter()
            .map(|ref meta| match meta {
                Meta::PathList(pl) if pl.path.is_strict("field") => {
                    StructFieldTypeNode::from_meta(meta)
                }
                _ => Err(meta.error("expected field(...) attribute")),
            })
            .collect_and_combine_errors()?;

        Ok(StructTypeNode { fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{NumberFormat::U32, NumberTypeNode, StringTypeNode};

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
            { struct(field("age", number(u32))) },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32))
            ]).into()
        );
    }

    #[test]
    fn with_multiple_fields() {
        assert_type!(
            { struct(field("age", number(u32)), field("name", string(utf8))) },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8())
            ]).into()
        );
    }

    #[test]
    fn with_brackets() {
        assert_type!(
            { struct[field("age", number(u32)), field("name", string(utf8))] },
            StructTypeNode::new(vec![
                StructFieldTypeNode::new("age", NumberTypeNode::le(U32)),
                StructFieldTypeNode::new("name", StringTypeNode::utf8())
            ]).into()
        );
    }

    #[test]
    fn invalid_fields_inside() {
        assert_type_err!({ struct(boolean, number(u32)) }, "expected field(...) attribute");
    }
}
