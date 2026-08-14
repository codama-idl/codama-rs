use crate::utils::FromMeta;
use codama_nodes::DisplaySkip;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DisplaySkip {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let path = meta.as_path()?;
        if path.is_strict("always") {
            Ok(Self::Always)
        } else if path.is_strict("never") {
            Ok(Self::Never)
        } else if path.is_strict("when_injected") {
            Ok(Self::WhenInjected)
        } else {
            Err(path.error("expected always, never, or when_injected"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_values() {
        for (meta, expected) in [
            (syn::parse_quote! { always }, DisplaySkip::Always),
            (syn::parse_quote! { never }, DisplaySkip::Never),
            (
                syn::parse_quote! { when_injected },
                DisplaySkip::WhenInjected,
            ),
        ] {
            assert_eq!(DisplaySkip::from_meta(&meta).unwrap(), expected);
        }
    }

    #[test]
    fn rejects_unsupported_values() {
        let meta: Meta = syn::parse_quote! { sometimes };
        assert_eq!(
            DisplaySkip::from_meta(&meta).unwrap_err().to_string(),
            "expected always, never, or when_injected"
        );
    }
}
