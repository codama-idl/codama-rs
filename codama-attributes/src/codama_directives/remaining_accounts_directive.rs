use crate::{
    utils::{removed_syntax, FromMeta, SetOnce},
    Attribute, Attributes, CodamaAttribute, CodamaDirective, TryFromFilter,
};
use codama_errors::CodamaError;
use codama_nodes::{
    CamelCaseString, Docs, InstructionAccountDisplayNode, InstructionRemainingAccountsNode,
    IsSigner,
};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct RemainingAccountsDirective {
    pub identifier: CamelCaseString,
    pub is_signer: Option<IsSigner>,
    pub is_writable: Option<bool>,
    pub is_optional: Option<bool>,
    pub docs: Docs,
    pub display: Option<InstructionAccountDisplayNode>,
}

impl RemainingAccountsDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let pl = meta
            .assert_directive("remaining_accounts")?
            .as_path_list()?;
        let mut identifier = SetOnce::<CamelCaseString>::new("name");
        let mut is_signer = SetOnce::<IsSigner>::new("signer");
        let mut is_writable = SetOnce::<bool>::new("writable");
        let mut is_optional = SetOnce::<bool>::new("optional");
        let mut docs = SetOnce::<Docs>::new("docs");
        let mut display = SetOnce::<InstructionAccountDisplayNode>::new("display");
        pl.each(|ref meta| match meta.path_str().as_str() {
            "argument" => removed_syntax(meta, "argument(...)", "remaining_accounts(\"x\", ...)"),
            "name" => identifier.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "signer" => is_signer.set(IsSigner::from_meta(meta)?, meta),
            "writable" => is_writable.set(bool::from_meta(meta)?, meta),
            "optional" => is_optional.set(bool::from_meta(meta)?, meta),
            "docs" => docs.set(Docs::from_meta(meta)?, meta),
            "display" => display.set(InstructionAccountDisplayNode::from_meta(meta)?, meta),
            _ => match meta.as_expr().and_then(|expr| expr.as_string()) {
                Ok(value) => identifier.set(value.into(), meta),
                Err(_) => Err(meta.error("unrecognized attribute")),
            },
        })?;
        Ok(RemainingAccountsDirective {
            identifier: identifier.take(meta)?,
            is_signer: is_signer.option(),
            is_writable: is_writable.option(),
            is_optional: is_optional.option(),
            docs: docs.option().unwrap_or_default(),
            display: display.option(),
        })
    }

    /// Construct an `InstructionRemainingAccountsNode` from this directive.
    pub fn to_instruction_remaining_accounts_node(&self) -> InstructionRemainingAccountsNode {
        InstructionRemainingAccountsNode {
            identifier: self.identifier.clone(),
            is_optional: self.is_optional,
            is_signer: self.is_signer,
            is_writable: self.is_writable,
            docs: self.docs.clone(),
            display: self.display.clone(),
            plugins: vec![],
        }
    }
}

impl RemainingAccountsDirective {
    pub fn nodes(attributes: &Attributes) -> Vec<InstructionRemainingAccountsNode> {
        attributes
            .iter()
            .filter_map(RemainingAccountsDirective::filter)
            .map(RemainingAccountsDirective::to_instruction_remaining_accounts_node)
            .collect()
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a RemainingAccountsDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::RemainingAccounts(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "remaining_accounts".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a RemainingAccountsDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::DisplaySkip;

    #[test]
    fn name_only() {
        let meta: Meta = syn::parse_quote! { remaining_accounts("signers") };
        let directive = RemainingAccountsDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            RemainingAccountsDirective {
                identifier: "signers".into(),
                is_signer: None,
                is_writable: None,
                is_optional: None,
                docs: Docs::default(),
                display: None,
            }
        );
    }

    #[test]
    fn fully_set() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(
            "signers",
            signer,
            writable,
            optional,
            docs = "Additional multisig signers."
        ) };
        let directive = RemainingAccountsDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            RemainingAccountsDirective {
                identifier: "signers".into(),
                is_signer: Some(IsSigner::True),
                is_writable: Some(true),
                is_optional: Some(true),
                docs: vec!["Additional multisig signers.".to_string()].into(),
                display: None,
            }
        );
    }

    #[test]
    fn fully_set_with_explicit_values() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(
            name = "signers",
            signer = "either",
            writable = false,
            optional = true,
            docs = ["Line 1", "Line 2"]
        ) };
        let directive = RemainingAccountsDirective::parse(&meta).unwrap();
        assert_eq!(
            directive,
            RemainingAccountsDirective {
                identifier: "signers".into(),
                is_signer: Some(IsSigner::Either),
                is_writable: Some(false),
                is_optional: Some(true),
                docs: vec!["Line 1".to_string(), "Line 2".to_string()].into(),
                display: None,
            }
        );
    }

    #[test]
    fn with_display() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(
            "signers",
            display(label = "Signers", skip = never)
        ) };
        let directive = RemainingAccountsDirective::parse(&meta).unwrap();
        let expected_display = InstructionAccountDisplayNode {
            label: Some("Signers".into()),
            skip: Some(DisplaySkip::Never),
            plugins: vec![],
        };
        assert_eq!(directive.display, Some(expected_display.clone()));
        assert_eq!(
            directive.to_instruction_remaining_accounts_node().display,
            Some(expected_display)
        );
    }

    #[test]
    fn missing_name() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(signer, optional) };
        let error = RemainingAccountsDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }

    #[test]
    fn name_already_set() {
        let meta: Meta = syn::parse_quote! { remaining_accounts("a", name = "b") };
        let error = RemainingAccountsDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "name is already set");
    }

    #[test]
    fn argument_is_removed() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(argument("signers"), signer) };
        let error = RemainingAccountsDirective::parse(&meta).unwrap_err();
        assert_eq!(
            error.to_string(),
            "argument(...) is removed in v2, use remaining_accounts(\"x\", ...)"
        );
    }

    #[test]
    fn invalid_signer() {
        let meta: Meta = syn::parse_quote! { remaining_accounts("signers", signer = "maybe") };
        let error = RemainingAccountsDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "expected boolean or `\"either\"`");
    }

    #[test]
    fn unrecognized_attribute() {
        let meta: Meta = syn::parse_quote! { remaining_accounts("signers", banana) };
        let error = RemainingAccountsDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized attribute");
    }

    #[test]
    fn to_node() {
        let meta: Meta = syn::parse_quote! { remaining_accounts(
            "signers",
            signer = "either",
            optional,
            docs = "Additional multisig signers."
        ) };
        let directive = RemainingAccountsDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.to_instruction_remaining_accounts_node(),
            InstructionRemainingAccountsNode {
                identifier: "signers".into(),
                is_optional: Some(true),
                is_signer: Some(IsSigner::Either),
                is_writable: None,
                docs: vec!["Additional multisig signers.".to_string()].into(),
                display: None,
                plugins: vec![],
            }
        );
    }
}
