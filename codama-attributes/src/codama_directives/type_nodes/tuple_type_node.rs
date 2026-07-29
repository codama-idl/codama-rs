use crate::utils::FromMeta;
use codama_errors::IteratorCombineErrors;
use codama_nodes::{TupleTypeNode, TypeNode};
use codama_syn_helpers::Meta;

impl FromMeta for TupleTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("tuple")?;
        if meta.is_path_or_empty_list() {
            return Ok(TupleTypeNode::new(vec![]));
        }

        let items = meta
            .as_path_list()?
            .parse_metas()?
            .iter()
            .map(TypeNode::from_meta)
            .collect_and_combine_errors()?;

        Ok(TupleTypeNode::new(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::{
        NumberFormat::{U32, U8},
        NumberTypeNode, PublicKeyTypeNode, StringTypeNode,
    };

    #[test]
    fn empty() {
        assert_type!({ tuple }, TupleTypeNode::new(vec![]).into());
        assert_type!({ tuple() }, TupleTypeNode::new(vec![]).into());
    }

    #[test]
    fn single_item() {
        assert_type!(
            { tuple(number(u32)) },
            TupleTypeNode::new(vec![NumberTypeNode::le(U32).into()]).into()
        );
    }

    #[test]
    fn multiple_items() {
        assert_type!(
            { tuple(number(u32), string, public_key) },
            TupleTypeNode::new(vec![
                NumberTypeNode::le(U32).into(),
                StringTypeNode::utf8().into(),
                PublicKeyTypeNode::new().into(),
            ])
            .into()
        );
    }

    #[test]
    fn nested() {
        assert_type!(
            { tuple(option(number(u8)), tuple(string)) },
            TupleTypeNode::new(vec![
                codama_nodes::OptionTypeNode::new(NumberTypeNode::le(U8)).into(),
                TupleTypeNode::new(vec![StringTypeNode::utf8().into()]).into(),
            ])
            .into()
        );
    }

    #[test]
    fn unrecognized_type() {
        assert_type_err!({ tuple(unrecognized) }, "unrecognized type");
    }
}
