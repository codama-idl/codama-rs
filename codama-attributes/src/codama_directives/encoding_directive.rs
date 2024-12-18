use codama_nodes::BytesEncoding;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct EncodingDirective {
    pub encoding: BytesEncoding,
}

impl TryFrom<&Meta> for EncodingDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.as_path_value()?;
        if !pv.path.is_strict("encoding") {
            return Err(pv.path.error("expected #[codama(encoding = ...)]"));
        };

        // TODO
        Ok(Self {
            encoding: BytesEncoding::Utf8,
        })
    }
}
