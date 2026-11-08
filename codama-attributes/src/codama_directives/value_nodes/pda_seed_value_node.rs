use crate::utils::{removed_name_key, removed_syntax, FromMeta, SetOnce};
use codama_nodes::{
    AccountValueNode, DataValueNode, Identifier, PdaSeedValueNode, PdaSeedValueValue,
    StringValueNode, ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PdaSeedValueValue {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "account" => AccountValueNode::from_meta(meta).map(Self::from),
            "argument" => removed_syntax(meta, "argument(...)", "data(...)"),
            "data" => DataValueNode::from_meta(meta).map(Self::from),
            _ => ValueNode::from_meta(meta).map(Self::from),
        }
    }
}

impl FromMeta for PdaSeedValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        // Account and data value node shortcut.
        match meta.path_str().as_str() {
            "account" => {
                let value = AccountValueNode::from_meta(meta)?;
                return Ok(PdaSeedValueNode::new(value.identifier.clone(), value));
            }
            "argument" => return removed_syntax(meta, "argument(...)", "data(...)"),
            "data" => {
                let value = DataValueNode::from_meta(meta)?;
                let path = value.path.to_string();
                // The seed takes the name of the data field, which a nested path does not give.
                if value.path.is_nested() {
                    return Err(meta.error(format!(
                        r#"a nested path needs a seed name, use seed("name", data("{path}"))"#
                    )));
                }
                let identifier = path.parse::<Identifier>().map_err(|e| meta.error(e))?;
                return Ok(PdaSeedValueNode::new(identifier, value));
            }
            _ => {}
        }

        // Regular seed parsing.
        let pl = meta.assert_directive("seed")?.as_path_list()?;
        let mut name = SetOnce::<Identifier>::new("identifier");
        let mut value: SetOnce<PdaSeedValueValue> = SetOnce::<PdaSeedValueValue>::new("value");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "identifier" => name.set(
                Identifier::try_from(meta.as_value()?.as_expr()?.as_string()?)
                    .map_err(|e| meta.error(e))?,
                meta,
            ),
            "name" => removed_name_key(meta),
            "value" => value.set(PdaSeedValueValue::from_meta(meta.as_value()?)?, meta),
            _ => {
                if let Ok(seed_name) = meta.as_expr().and_then(|e| e.as_string()) {
                    match name.is_set() {
                        false => {
                            let identifier =
                                seed_name.parse::<Identifier>().map_err(|e| meta.error(e))?;
                            return name.set(identifier, meta);
                        }
                        true => return value.set(StringValueNode::new(seed_name).into(), meta),
                    }
                }
                if meta.is_path_or_list() {
                    return value.set(PdaSeedValueValue::from_meta(meta)?, meta);
                }
                if let Ok(seed_value) = PdaSeedValueValue::from_meta(meta) {
                    return value.set(seed_value, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(PdaSeedValueNode::new(name.take(meta)?, value.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::IntegerValueNode;

    #[test]
    fn explicit() {
        let meta: Meta = syn::parse_quote! { seed(identifier = "identifier", value = 42) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", IntegerValueNode::new(42u8))
        );
    }

    #[test]
    fn implicit() {
        let meta: Meta = syn::parse_quote! { seed("identifier", 42) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", IntegerValueNode::new(42u8))
        );
    }

    #[test]
    fn explicit_account_value_node() {
        let meta: Meta = syn::parse_quote! { seed("authority", account("authority")) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("authority", AccountValueNode::new("authority"))
        );
    }

    #[test]
    fn explicit_data_value_node() {
        let meta: Meta = syn::parse_quote! { seed("identifier", data("identifier")) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", DataValueNode::new("identifier"))
        );
    }

    #[test]
    fn implicit_account_value_node() {
        let meta: Meta = syn::parse_quote! { account("authority") };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("authority", AccountValueNode::new("authority"))
        );
    }

    #[test]
    fn implicit_data_value_node() {
        let meta: Meta = syn::parse_quote! { data("identifier") };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", DataValueNode::new("identifier"))
        );
    }

    #[test]
    fn implicit_data_value_node_with_nested_path() {
        for (meta, expected) in [
            (
                syn::parse_quote! { data("config.mint") },
                r#"a nested path needs a seed name, use seed("name", data("config.mint"))"#,
            ),
            (
                syn::parse_quote! { data("fees[0]") },
                r#"a nested path needs a seed name, use seed("name", data("fees[0]"))"#,
            ),
            (
                syn::parse_quote! { data("[0].mint") },
                r#"a nested path needs a seed name, use seed("name", data("[0].mint"))"#,
            ),
        ] {
            let error = PdaSeedValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn explicit_data_value_node_with_nested_path() {
        let meta: Meta = syn::parse_quote! { seed("mint", data("config.mint")) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("mint", DataValueNode::new("config.mint"))
        );
    }

    #[test]
    fn argument_is_removed() {
        let hint = "argument(...) is removed in v2, use data(...)";
        for meta in [
            syn::parse_quote! { argument("identifier") },
            syn::parse_quote! { seed("identifier", argument("identifier")) },
            syn::parse_quote! { seed(identifier = "identifier", value = argument("identifier")) },
        ] {
            let error = PdaSeedValueNode::from_meta(&meta).unwrap_err();
            assert_eq!(error.to_string(), hint);
        }
    }

    #[test]
    fn err_todo() {
        let meta: Meta = syn::parse_quote! { seed() };
        let error = PdaSeedValueNode::from_meta(&meta).unwrap_err();
        assert_eq!(error.to_string(), "identifier is missing");
    }

    #[test]
    fn name_is_removed() {
        let meta: Meta = syn::parse_quote! { seed(name = "mint", account("mint")) };
        let error = PdaSeedValueNode::from_meta(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "name = ... is removed in v2, use identifier = ..."
        );
    }
}
