use codama_nodes::Node;
use codama_syn_helpers::Meta;

pub trait NodeAttributeParse {
    fn from_meta(meta: &Meta) -> syn::Result<Node>;
}