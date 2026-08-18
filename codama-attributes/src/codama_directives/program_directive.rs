use crate::{
    utils::SetOnce, Attribute, AttributeContext, CodamaAttribute, CodamaDirective, TryFromFilter,
};
use codama_errors::CodamaError;
use codama_nodes::{CamelCaseString, Node, ProgramNode};
use codama_syn_helpers::{extensions::*, Meta};

/// The `#[codama(program(name = ..., address = ...))]` directive.
///
/// Its required arguments depend on the scope it is attached to:
///
/// - At the **crate root** ([`AttributeContext::Crate`]) it *overrides* the
///   primary program's identity, so both `name` and `address` are optional —
///   any omitted field falls back to the crate defaults (the Cargo.toml package
///   name and the `declare_id!`/`package.metadata.solana.program-id` address).
/// - At any **other scope** (an item, a module block or a file-module) it
///   declares a *distinct* program that has no defaults to fall back on, so both
///   `name` and `address` are required.
#[derive(Debug, PartialEq)]
pub struct ProgramDirective {
    pub name: Option<CamelCaseString>,
    pub address: Option<String>,
}

impl ProgramDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        let pl = meta.assert_directive("program")?.as_path_list()?;

        let mut name = SetOnce::<CamelCaseString>::new("name");
        let mut address = SetOnce::<String>::new("address");

        pl.each(|ref meta| match meta.path_str().as_str() {
            "name" => name.set(meta.as_value()?.as_expr()?.as_string()?.into(), meta),
            "address" => address.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        // Both arguments are optional at the crate root (override), required elsewhere.
        if let AttributeContext::Crate(_) = ctx {
            Ok(Self {
                name: name.option(),
                address: address.option(),
            })
        } else {
            Ok(Self {
                name: Some(name.take(meta)?),
                address: Some(address.take(meta)?),
            })
        }
    }

    pub fn apply(attributes: &crate::Attributes, node: Node) -> Node {
        match attributes.get_last(Self::filter) {
            Some(pd) => pd.update_or_wrap_program_node(node),
            None => node,
        }
    }

    pub fn update_or_wrap_program_node(&self, node: Node) -> Node {
        // Wrapping a leaf node only happens outside the crate root, where both fields are set.
        let name = self.name.clone().unwrap_or_default();
        let public_key = self.address.clone().unwrap_or_default();
        match node {
            // Updating an existing program only overrides the fields the directive set.
            Node::Root(mut root) => {
                if let Some(name) = self.name.clone() {
                    root.program.name = name;
                }
                if let Some(public_key) = self.address.clone() {
                    root.program.public_key = public_key;
                }
                root.into()
            }
            Node::Program(mut program) => {
                if let Some(name) = self.name.clone() {
                    program.name = name;
                }
                if let Some(public_key) = self.address.clone() {
                    program.public_key = public_key;
                }
                program.into()
            }
            Node::Account(account) => ProgramNode {
                name,
                public_key,
                accounts: vec![account],
                ..ProgramNode::default()
            }
            .into(),
            Node::Constant(constant) => ProgramNode {
                name,
                public_key,
                constants: vec![constant],
                ..ProgramNode::default()
            }
            .into(),
            Node::Instruction(instruction) => ProgramNode {
                name,
                public_key,
                instructions: vec![instruction],
                ..ProgramNode::default()
            }
            .into(),
            Node::Error(error) => ProgramNode {
                name,
                public_key,
                errors: vec![error],
                ..ProgramNode::default()
            }
            .into(),
            Node::Pda(pda) => ProgramNode {
                name,
                public_key,
                pdas: vec![pda],
                ..ProgramNode::default()
            }
            .into(),
            Node::Event(event) => ProgramNode {
                name,
                public_key,
                events: vec![event],
                ..ProgramNode::default()
            }
            .into(),
            other => other,
        }
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a ProgramDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Program(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "program".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a ProgramDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item_ctx() -> syn::Item {
        syn::parse_quote! { struct Foo; }
    }

    fn crate_file() -> syn::File {
        syn::parse_quote! {}
    }

    #[test]
    fn ok() {
        let item = item_ctx();
        let ctx = AttributeContext::Item(&item);
        let meta: Meta = syn::parse_quote! { program(name = "associatedToken", address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL") };
        let directive = ProgramDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            ProgramDirective {
                name: Some(CamelCaseString::from("associatedToken")),
                address: Some("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string()),
            }
        );
    }

    #[test]
    fn name_missing_at_item_scope() {
        let item = item_ctx();
        let ctx = AttributeContext::Item(&item);
        let meta: Meta =
            syn::parse_quote! { program(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL") };
        let error = ProgramDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "name is missing");
    }

    #[test]
    fn address_missing_at_item_scope() {
        let item = item_ctx();
        let ctx = AttributeContext::Item(&item);
        let meta: Meta = syn::parse_quote! { program(name = "associatedToken") };
        let error = ProgramDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "address is missing");
    }

    #[test]
    fn name_only_at_crate_scope() {
        let file = crate_file();
        let ctx = AttributeContext::Crate(&file);
        let meta: Meta = syn::parse_quote! { program(name = "associatedToken") };
        let directive = ProgramDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            ProgramDirective {
                name: Some(CamelCaseString::from("associatedToken")),
                address: None,
            }
        );
    }

    #[test]
    fn address_only_at_crate_scope() {
        let file = crate_file();
        let ctx = AttributeContext::Crate(&file);
        let meta: Meta =
            syn::parse_quote! { program(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL") };
        let directive = ProgramDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            ProgramDirective {
                name: None,
                address: Some("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".to_string()),
            }
        );
    }

    #[test]
    fn empty_at_crate_scope() {
        let file = crate_file();
        let ctx = AttributeContext::Crate(&file);
        let meta: Meta = syn::parse_quote! { program() };
        let directive = ProgramDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            ProgramDirective {
                name: None,
                address: None,
            }
        );
    }
}
