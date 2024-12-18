use codama_nodes::BytesEncoding;
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct EncodingDirective {
    pub encoding: BytesEncoding,
}

impl TryFrom<&Meta> for EncodingDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let _pv = meta.assert_directive("encoding")?.as_path_value()?;

        // TODO
        Ok(Self {
            encoding: BytesEncoding::Utf8,
        })
    }
}
