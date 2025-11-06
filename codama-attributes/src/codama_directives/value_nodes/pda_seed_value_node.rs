use crate::utils::{FromMeta, SetOnce};
use codama_nodes::{
    AccountValueNode, ArgumentValueNode, CamelCaseString, PdaSeedValueNode, PdaSeedValueValueNode,
    StringValueNode, ValueNode,
};
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for PdaSeedValueValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "account" => AccountValueNode::from_meta(meta).map(Self::from),
            "argument" => ArgumentValueNode::from_meta(meta).map(Self::from),
            _ => ValueNode::from_meta(meta).map(Self::from),
        }
    }
}

impl FromMeta for PdaSeedValueNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        // Account and Argument value node shortcut.
        if let Ok(value) = AccountValueNode::from_meta(meta) {
            return Ok(PdaSeedValueNode::new(value.name.clone(), value));
        }
        if let Ok(value) = ArgumentValueNode::from_meta(meta) {
            return Ok(PdaSeedValueNode::new(value.name.clone(), value));
        }

        // Regular seed parsing.
        let pl = meta.assert_directive("seed")?.as_path_list()?;
        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut value: SetOnce<PdaSeedValueValueNode> =
            SetOnce::<PdaSeedValueValueNode>::new("value");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "value" => value.set(PdaSeedValueValueNode::from_meta(meta.as_value()?)?, meta),
            _ => {
                if let Ok(seed_name) = meta.as_expr().and_then(|e| e.as_string()) {
                    match name.is_set() {
                        false => return name.set(seed_name.into(), meta),
                        true => return value.set(StringValueNode::new(seed_name).into(), meta),
                    }
                }
                if let Ok(seed_value) = PdaSeedValueValueNode::from_meta(meta) {
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
    use codama_nodes::NumberValueNode;

    use super::*;

    #[test]
    fn explicit() {
        let meta: Meta = syn::parse_quote! { seed(name = "identifier", value = 42) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", NumberValueNode::new(42u8))
        );
    }

    #[test]
    fn implicit() {
        let meta: Meta = syn::parse_quote! { seed("identifier", 42) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", NumberValueNode::new(42u8))
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
    fn explicit_argument_value_node() {
        let meta: Meta = syn::parse_quote! { seed("identifier", argument("identifier")) };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", ArgumentValueNode::new("identifier"))
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
    fn implicit_argument_value_node() {
        let meta: Meta = syn::parse_quote! { argument("identifier") };
        let node = PdaSeedValueNode::from_meta(&meta).unwrap();
        assert_eq!(
            node,
            PdaSeedValueNode::new("identifier", ArgumentValueNode::new("identifier"))
        );
    }

    #[test]
    fn err_todo() {
        let meta: Meta = syn::parse_quote! { seed() };
        let error = PdaSeedValueNode::from_meta(&meta).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }
}
