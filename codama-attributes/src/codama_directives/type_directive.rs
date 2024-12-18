use codama_nodes::TypeNode;
use codama_syn_helpers::Meta;

use crate::utils::FromMeta;

#[derive(Debug, PartialEq)]
pub struct TypeDirective {
    pub node: TypeNode,
}

impl TryFrom<&Meta> for TypeDirective {
    type Error = syn::Error;

    fn try_from(meta: &Meta) -> syn::Result<Self> {
        let pv = meta.assert_directive("type")?.as_path_value()?;
        let node = TypeNode::from_meta(&pv.value)?;
        Ok(Self { node })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{NumberFormat::U16, NumberTypeNode};
    use syn::parse_quote;

    #[test]
    fn ok() {
        let meta: Meta = parse_quote! { type = number(u16, le) };
        let node = TypeDirective::try_from(&meta).unwrap().node;

        assert_eq!(node, NumberTypeNode::le(U16).into());
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { type =  };
        let error = TypeDirective::try_from(&meta).unwrap_err();
        assert!(error.to_string().contains("unrecognized type"));
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { type = (number(u16, le), public_key) };
        let error = TypeDirective::try_from(&meta).unwrap_err();
        assert!(error
            .to_string()
            .contains("expected a single value, found a list"));
    }

    #[test]
    fn unrecognized_type() {
        let meta: Meta = parse_quote! { type = banana };
        let error = TypeDirective::try_from(&meta).unwrap_err();
        assert!(error.to_string().contains("unrecognized type"));
    }
}
