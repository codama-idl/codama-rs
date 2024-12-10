use codama_nodes::Node;

pub trait NodeAttributeParse {
    fn from_meta(path: &syn::Path, meta: &syn::meta::ParseNestedMeta) -> syn::Result<Node>;
}
