use crate::utils::{FromMeta, SetOnce};
use codama_nodes::DefinedTypeLinkNode;
use codama_syn_helpers::{extensions::*, Meta};

impl FromMeta for DefinedTypeLinkNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        let pl = meta.assert_directive("link")?.as_path_list()?;
        let mut name: SetOnce<String> = SetOnce::new("name");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            _ => {
                if let Ok(expr) = meta.as_expr() {
                    return name.set(expr.as_string()?, meta);
                }
                Err(meta.error("unrecognized attribute"))
            }
        })?;

        Ok(Self::new(name.take(meta)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_type, assert_type_err};
    use codama_nodes::OptionTypeNode;

    #[test]
    fn implicit() {
        assert_type!(
            { link("customString") },
            DefinedTypeLinkNode::new("customString").into()
        );
    }

    #[test]
    fn explicit() {
        assert_type!(
            { link(name = "customString") },
            DefinedTypeLinkNode::new("customString").into()
        );
    }

    #[test]
    fn as_option_item() {
        assert_type!(
            { option(link("customString")) },
            OptionTypeNode::new(DefinedTypeLinkNode::new("customString")).into()
        );
        assert_type!(
            { option(link("customString"), fixed) },
            OptionTypeNode::fixed(DefinedTypeLinkNode::new("customString")).into()
        );
    }

    #[test]
    fn expected_string() {
        assert_type_err!({ link(42) }, "expected a string");
        assert_type_err!({ link(name = 42) }, "expected a string");
    }

    #[test]
    fn unrecognized_attribute() {
        assert_type_err!({ link(foo = 42) }, "unrecognized attribute");
    }
}
