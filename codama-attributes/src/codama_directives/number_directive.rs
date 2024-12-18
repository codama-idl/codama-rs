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
        let pl = meta.as_path_list()?;
        if !pl.path.is_strict("number") {
            return Err(pl.path.error("expected #[codama(number(...))]"));
        };

        // TODO
        Ok(Self {
            endian: None,
            format: None,
        })
    }
}
