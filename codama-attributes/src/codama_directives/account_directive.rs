use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, AttributeContext, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{
    CamelCaseString, Docs, InstructionAccountNode, InstructionInputValueNode, IsAccountSigner,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct AccountDirective {
    pub account: InstructionAccountNode,
}

impl AccountDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        meta.assert_directive("account")?;
        let mut name = SetOnce::<CamelCaseString>::new("name");
        if let AttributeContext::Field(syn::Field {
            ident: Some(ident), ..
        }) = ctx
        {
            name = name.initial_value(ident.to_string().into())
        }
        let mut is_writable = SetOnce::<bool>::new("writable").initial_value(false);
        let mut is_signer = SetOnce::<IsAccountSigner>::new("signer").initial_value(false.into());
        let mut is_optional = SetOnce::<bool>::new("optional").initial_value(false);
        let mut default_value = SetOnce::<InstructionInputValueNode>::new("default_value");
        match meta.is_path_or_empty_list() {
            true => (),
            false => meta
                .as_path_list()?
                .each(|ref meta| match meta.path_str().as_str() {
                    "name" => name.set(String::from_meta(meta)?.into(), meta),
                    "writable" => is_writable.set(bool::from_meta(meta)?, meta),
                    "signer" => is_signer.set(IsAccountSigner::from_meta(meta)?, meta),
                    "optional" => is_optional.set(bool::from_meta(meta)?, meta),
                    "default_value" => {
                        let value = &meta.as_path_value()?.value;
                        default_value.set(InstructionInputValueNode::from_meta(value)?, meta)
                    }
                    _ => Err(meta.error("unrecognized attribute")),
                })?,
        }
        Ok(AccountDirective {
            account: InstructionAccountNode {
                name: name.take(meta)?,
                is_writable: is_writable.take(meta)?,
                is_signer: is_signer.take(meta)?,
                is_optional: is_optional.take(meta)?,
                // TODO: `docs` for account directives not attached to fields.
                docs: Docs::default(),
                default_value: default_value.option(),
            },
        })
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a AccountDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive {
            CodamaDirective::Account(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "account".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a AccountDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::PayerValueNode;

    #[test]
    fn fully_set() {
        let meta: Meta = syn::parse_quote! { account(name = "payer", writable, signer, optional, default_value = payer) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                account: InstructionAccountNode {
                    name: "payer".into(),
                    is_writable: true,
                    is_signer: IsAccountSigner::True,
                    is_optional: true,
                    default_value: Some(PayerValueNode::new().into()),
                    docs: Docs::default(),
                },
            }
        );
    }

    #[test]
    fn fully_set_with_explicit_values() {
        let meta: Meta = syn::parse_quote! { account(
            name = "payer",
            writable = true,
            signer = "either",
            optional = false,
            default_value = payer
        ) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                account: InstructionAccountNode {
                    name: "payer".into(),
                    is_writable: true,
                    is_signer: IsAccountSigner::Either,
                    is_optional: false,
                    default_value: Some(PayerValueNode::new().into()),
                    docs: Docs::default(),
                },
            }
        );
    }

    #[test]
    fn empty_on_named_field() {
        let meta: Meta = syn::parse_quote! { account };
        let field = syn::parse_quote! { authority: AccountMeta };
        let ctx = AttributeContext::Field(&field);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                account: InstructionAccountNode {
                    name: "authority".into(),
                    is_writable: false,
                    is_signer: IsAccountSigner::False,
                    is_optional: false,
                    default_value: None,
                    docs: Docs::default(),
                },
            }
        );
    }

    #[test]
    fn empty_on_struct() {
        let meta: Meta = syn::parse_quote! { account };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let error = AccountDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }
}
