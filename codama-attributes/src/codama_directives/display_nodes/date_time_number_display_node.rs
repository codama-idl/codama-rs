use crate::utils::{FromMeta, SetOnce};
use codama_nodes::DateTimeNumberDisplayNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DateTimeNumberDisplayNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        meta.assert_directive("date_time")?;
        if meta.is_path_or_empty_list() {
            return Ok(Self::default());
        }
        let pl = meta.as_path_list()?;
        let mut ticks_per_second: SetOnce<u64> = SetOnce::new("ticks_per_second");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "ticks_per_second" => {
                let value = meta.as_value()?;
                let ticks_per_second_value = value.as_expr()?.as_unsigned_integer()?;
                if ticks_per_second_value == 0 {
                    return Err(value.error("ticks_per_second must be greater than zero"));
                }
                ticks_per_second.set(ticks_per_second_value, meta)
            }
            _ => Err(meta.error("unrecognized date_time display attribute")),
        })?;

        Ok(Self {
            ticks_per_second: ticks_per_second.option(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_display() {
        let meta: Meta = syn::parse_quote! { date_time(ticks_per_second = 1_000) };
        assert_eq!(
            DateTimeNumberDisplayNode::from_meta(&meta).unwrap(),
            DateTimeNumberDisplayNode {
                ticks_per_second: Some(1_000),
            }
        );
    }

    #[test]
    fn parses_empty_display() {
        for meta in [
            syn::parse_quote! { date_time },
            syn::parse_quote! { date_time() },
        ] {
            assert_eq!(
                DateTimeNumberDisplayNode::from_meta(&meta).unwrap(),
                DateTimeNumberDisplayNode::default()
            );
        }
    }

    #[test]
    fn rejects_positional_attributes() {
        let meta: Meta = syn::parse_quote! { date_time(1_000) };
        assert!(DateTimeNumberDisplayNode::from_meta(&meta).is_err());
    }

    #[test]
    fn rejects_zero_ticks_per_second() {
        let meta: Meta = syn::parse_quote! { date_time(ticks_per_second = 0) };
        assert_eq!(
            DateTimeNumberDisplayNode::from_meta(&meta)
                .unwrap_err()
                .to_string(),
            "ticks_per_second must be greater than zero"
        );
    }
}
