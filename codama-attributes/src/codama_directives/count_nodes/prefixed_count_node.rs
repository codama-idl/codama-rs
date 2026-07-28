use crate::codama_directives::nested_number::nested_number;
use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{NestedTypeNode, NumberTypeNode, PrefixedCountNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PrefixedCountNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("prefixed_count")?.as_path_list()?;
        let mut prefix: SetOnce<NestedTypeNode<NumberTypeNode>> = SetOnce::new("prefix");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "prefix" => prefix.set(nested_number(meta.as_value()?, "prefix")?, meta),
            _ => {
                if meta.is_path_or_list() {
                    return prefix.set(nested_number(meta, "prefix")?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(PrefixedCountNode::new(prefix.take(meta)?))
    }
}
