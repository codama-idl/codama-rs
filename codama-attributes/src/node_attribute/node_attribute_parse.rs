use codama_errors::CodamaResult;
use codama_nodes::Node;

pub trait NodeAttributeParse {
    fn from_meta(meta: &syn::meta::ParseNestedMeta) -> CodamaResult<Node>;
}
