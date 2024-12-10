use codama_nodes::Node;
use codama_syn_helpers::AttributeMeta;

pub trait NodeAttributeParse {
    fn from_meta(meta: &AttributeMeta) -> syn::Result<Node>;
}
