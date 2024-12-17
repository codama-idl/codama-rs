use codama_nodes::{Endian, NumberFormat};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct NumberDirective {
    pub endian: Option<Endian>,
    pub format: Option<NumberFormat>,
}

impl TryFrom<&Meta> for NumberDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let list = meta.as_list()?;
        if !list.path.is_strict("number") {
            return Err(list.path.error("expected #[codama(number(...))]"));
        };

        // TODO
        Ok(Self {
            endian: None,
            format: None,
        })
    }
}
