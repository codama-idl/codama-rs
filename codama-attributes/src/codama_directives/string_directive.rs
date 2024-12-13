use crate::ApplyToNode;
use codama_nodes::BytesEncoding;
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct StringDirective {
    pub encoding: BytesEncoding,
}

impl ApplyToNode for StringDirective {}

impl TryFrom<&Meta> for StringDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let list = meta.as_list()?;
        if !list.path.is_strict("string") {
            return Err(list.path.error("expected #[string(...)]"));
        };

        // TODO
        Ok(Self {
            encoding: BytesEncoding::Utf8,
        })
    }
}
