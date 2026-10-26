use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{DateTimeTypeNode, IntegerTypeNode, TypeNode, TypeNodeTrait};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DateTimeTypeNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("date_time")?.as_path_list()?;
        let mut number: SetOnce<IntegerTypeNode> = SetOnce::new("number");

        pl.each(|ref meta| {
            // Lists and bare paths go to the type parser first, so `number(u64)` gets its hint, not the `number =` field.
            if meta.is_path_or_list() {
                let node = TypeNode::from_meta(meta)?;
                let number_node = match IntegerTypeNode::try_from_type_node(node) {
                    Ok(node) => node,
                    Err(_) => return Err(meta.error("number must be an IntegerTypeNode")),
                };
                return number.set(number_node, meta);
            }
            match meta.path_str().as_str() {
                "number" => {
                    let value = meta.as_value()?;
                    let node = TypeNode::from_meta(value)?;
                    let number_node = match IntegerTypeNode::try_from_type_node(node) {
                        Ok(node) => node,
                        Err(_) => return Err(value.error("number must be an IntegerTypeNode")),
                    };
                    number.set(number_node, meta)
                }
                _ => Err(meta.error("unrecognized attribute")),
            }
        })?;

        Ok(DateTimeTypeNode::new(number.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::IntegerFormat::U64;

    #[test]
    fn parses() {
        for meta in [
            syn::parse_quote! { date_time(integer(u64)) },
            syn::parse_quote! { date_time(number = integer(u64)) },
        ] {
            assert_eq!(
                DateTimeTypeNode::from_meta(&meta).unwrap(),
                DateTimeTypeNode::new(IntegerTypeNode::le(U64))
            );
        }
    }

    #[test]
    fn errors() {
        for (meta, expected) in [
            (
                syn::parse_quote! { date_time(string) },
                "number must be an IntegerTypeNode",
            ),
            (syn::parse_quote! { date_time() }, "number is missing"),
            (
                syn::parse_quote! { date_time(foo = 42) },
                "unrecognized attribute",
            ),
            (
                syn::parse_quote! { date_time(number(u64)) },
                "number(...) is removed in v2, use integer(...) or float(...)",
            ),
        ] {
            let error = DateTimeTypeNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
