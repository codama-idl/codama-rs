use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{DisplaySkip, InstructionAccountDisplayNode};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for InstructionAccountDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("display")?.as_path_list()?;
        let mut label: SetOnce<String> = SetOnce::new("label");
        let mut skip: SetOnce<DisplaySkip> = SetOnce::new("skip");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "label" => label.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            "skip" => skip.set(DisplaySkip::from_meta(meta.as_value()?)?, meta),
            _ => Err(meta.error("unrecognized instruction account display attribute")),
        })?;

        let display = Self {
            label: label.option(),
            skip: skip.option(),
        };
        if display == Self::default() {
            return Err(meta.error("instruction account display requires at least one attribute"));
        }
        Ok(display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_display() {
        let meta: Meta = syn::parse_quote! { display(label = "Payer", skip = always) };
        assert_eq!(
            InstructionAccountDisplayNode::from_meta(&meta).unwrap(),
            InstructionAccountDisplayNode {
                label: Some("Payer".to_string()),
                skip: Some(DisplaySkip::Always),
            }
        );
    }

    #[test]
    fn rejects_empty_display() {
        let meta: Meta = syn::parse_quote! { display() };
        assert_eq!(
            InstructionAccountDisplayNode::from_meta(&meta)
                .unwrap_err()
                .to_string(),
            "instruction account display requires at least one attribute"
        );
    }
}
