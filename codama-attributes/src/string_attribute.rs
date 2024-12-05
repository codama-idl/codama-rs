use codama_nodes::BytesEncoding;

#[derive(Debug, PartialEq)]
pub struct StringAttribute<'a> {
    pub ast: &'a syn::Attribute,
    pub encoding: BytesEncoding,
}
