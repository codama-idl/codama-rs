use crate::{
    utils::{FromMeta, SetOnce},
    AttributeContext,
};
use codama_nodes::{CamelCaseString, Docs, InstructionAccountNode, IsAccountSigner};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct AccountDirective {
    pub name: CamelCaseString,
    pub is_writable: bool,
    pub is_signer: IsAccountSigner,
    pub is_optional: bool,
    // TODO: `docs` for account directives not attached to fields.
}

impl AccountDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        meta.assert_directive("account")?;
        let mut name = SetOnce::<CamelCaseString>::new("name");
        match ctx {
            AttributeContext::Field(syn::Field {
                ident: Some(ident), ..
            }) => name = name.initial_value(ident.to_string().into()),
            _ => (),
        }
        let mut is_writable = SetOnce::<bool>::new("writable").initial_value(false);
        let mut is_signer = SetOnce::<IsAccountSigner>::new("signer").initial_value(false.into());
        let mut is_optional = SetOnce::<bool>::new("optional").initial_value(false);
        match meta.is_path_or_empty_list() {
            true => (),
            false => meta
                .as_path_list()?
                .each(|ref meta| match meta.path_str().as_str() {
                    "name" => name.set(String::from_meta(meta)?.into(), meta),
                    "writable" => is_writable.set(bool::from_meta(meta)?, meta),
                    "signer" => is_signer.set(IsAccountSigner::from_meta(meta)?, meta),
                    "optional" => is_optional.set(bool::from_meta(meta)?, meta),
                    _ => Err(meta.error("unrecognized attribute")),
                })?,
        }
        Ok(AccountDirective {
            name: name.take(meta)?,
            is_writable: is_writable.take(meta)?,
            is_signer: is_signer.take(meta)?,
            is_optional: is_optional.take(meta)?,
        })
    }
}

impl From<&AccountDirective> for InstructionAccountNode {
    fn from(value: &AccountDirective) -> Self {
        Self {
            name: value.name.clone(),
            is_writable: value.is_writable,
            is_signer: value.is_signer,
            is_optional: value.is_optional,
            docs: Docs::default(),
            default_value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_set() {
        let meta: Meta = syn::parse_quote! { account(name = "payer", writable, signer, optional) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                name: "payer".into(),
                is_writable: true,
                is_signer: IsAccountSigner::True,
                is_optional: true,
            }
        );
    }

    #[test]
    fn fully_set_with_explicit_values() {
        let meta: Meta = syn::parse_quote! { account(name = "payer", writable = true, signer = "either", optional = false) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                name: "payer".into(),
                is_writable: true,
                is_signer: IsAccountSigner::Either,
                is_optional: false,
            }
        );
    }

    #[test]
    fn empty_on_nammed_field() {
        let meta: Meta = syn::parse_quote! { account };
        let field = syn::parse_quote! { authority: AccountMeta };
        let ctx = AttributeContext::Field(&field);
        let directive = AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            AccountDirective {
                name: "authority".into(),
                is_writable: false,
                is_signer: IsAccountSigner::False,
                is_optional: false,
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
