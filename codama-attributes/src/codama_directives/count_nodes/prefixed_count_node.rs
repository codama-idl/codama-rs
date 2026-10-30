use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{IntegerTypeNode, PrefixedCountNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PrefixedCountNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("prefixed_count")?.as_path_list()?;
        let mut prefix: SetOnce<IntegerTypeNode> = SetOnce::new("prefix");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "prefix" => prefix.set(parse_prefix(meta.as_value()?)?, meta),
            _ => {
                if meta.is_path_or_list() {
                    return prefix.set(parse_prefix(meta)?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(PrefixedCountNode::new(prefix.take(meta)?))
    }
}

fn parse_prefix(meta: &Meta) -> syn::Result<IntegerTypeNode> {
    let node = TypeNode::from_meta(meta)?;
    IntegerTypeNode::try_from_type_node(node)
        .map_err(|_| meta.error("prefix must be an IntegerTypeNode"))
}
