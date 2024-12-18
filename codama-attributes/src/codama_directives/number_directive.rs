use codama_nodes::{Endian, NumberFormat};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct NumberDirective {
    pub endian: Option<Endian>,
    pub format: Option<NumberFormat>,
}

impl TryFrom<&Meta> for NumberDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let _pl = meta.assert_directive("number")?.as_path_list()?;

        // TODO
        Ok(Self {
            endian: None,
            format: None,
        })
    }
}
