use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct FixedSizeDirective {
    pub size: usize,
}

impl TryFrom<&Meta> for FixedSizeDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("fixed_size")?.as_path_value()?;
        let size = pv.value.as_expr()?.as_literal_integer()?;
        Ok(Self { size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let meta: Meta = syn::parse_quote! { fixed_size = 42 };
        let directive = FixedSizeDirective::try_from(&meta).unwrap();
        assert_eq!(directive, FixedSizeDirective { size: 42 });
    }
}
