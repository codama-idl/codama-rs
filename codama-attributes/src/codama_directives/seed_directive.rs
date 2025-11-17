use crate::{
    utils::{FromMeta, SetOnce},
    Attribute, AttributeContext, CodamaAttribute, CodamaDirective,
};
use codama_errors::CodamaError;
use codama_nodes::{ConstantPdaSeedNode, PdaSeedNode, TypeNode, ValueNode, VariablePdaSeedNode};
use codama_syn_helpers::{extensions::*, Meta};

#[derive(Debug, PartialEq)]
pub struct SeedDirective {
    pub seed: SeedDirectiveType,
}

#[derive(Debug, PartialEq)]
pub enum SeedDirectiveType {
    Linked(String),
    Defined(PdaSeedNode),
}

impl SeedDirective {
    pub fn parse(meta: &Meta, ctx: &AttributeContext) -> syn::Result<Self> {
        let pl = meta.assert_directive("seed")?.as_path_list()?;

        let constant_seed = pl
            .parse_metas()?
            .iter()
            .find_map(|m| match m.path_str().as_str() {
                "name" => Some(false),
                "value" => Some(true),
                _ => None,
            })
            .ok_or_else(|| meta.error("seed must at least specify `name` for variable seeds or `type` and `value` for constant seeds"))?;

        let mut name = SetOnce::<String>::new("name");
        let mut r#type = SetOnce::<TypeNode>::new("type");
        let mut value = SetOnce::<ValueNode>::new("value");

        pl.each(|ref meta| match (meta.path_str().as_str(), constant_seed) {
            ("name", true) => Err(meta.error("constant seeds cannot specify name")),
            ("name", false) => name.set(meta.as_value()?.as_expr()?.as_string()?, meta),
            ("value", true) => value.set(ValueNode::from_meta(meta.as_value()?)?, meta),
            ("value", false) => Err(meta.error("variable seeds cannot specify value")),
            ("type", _) => r#type.set(TypeNode::from_meta(meta.as_value()?)?, meta),
            _ => Err(meta.error("unrecognized attribute")),
        })?;

        // Resolve linked seed if possible.
        if !constant_seed && !r#type.is_set() {
            let name = name.take(meta)?;
            if !has_matching_field(ctx, &name) {
                let message = format!("Could not find field \"{name}\". Either specify a `type` for the seed or use a name that matches a struct or variant field.");
                return Err(meta.error(message));
            }
            return Ok(Self {
                seed: SeedDirectiveType::Linked(name),
            });
        }

        match constant_seed {
            true => Ok(Self {
                seed: SeedDirectiveType::Defined(
                    ConstantPdaSeedNode::new(r#type.take(meta)?, value.take(meta)?).into(),
                ),
            }),
            false => Ok(Self {
                seed: SeedDirectiveType::Defined(
                    VariablePdaSeedNode::new(name.take(meta)?, r#type.take(meta)?).into(),
                ),
            }),
        }
    }
}

fn has_matching_field(ctx: &AttributeContext, name: &str) -> bool {
    let Some(fields) = ctx.get_named_fields() else {
        return false;
    };

    fields
        .named
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|id| id == name))
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a SeedDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Seed(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "seed".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a SeedDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
    }
}

#[cfg(test)]
mod tests {
    use codama_nodes::{NumberFormat::U8, NumberTypeNode, NumberValueNode, PublicKeyTypeNode};

    use super::*;

    #[test]
    fn defined_constant() {
        let meta: Meta = syn::parse_quote! { seed(type = number(u8), value = 42) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = SeedDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            SeedDirective {
                seed: SeedDirectiveType::Defined(
                    ConstantPdaSeedNode::new(NumberTypeNode::le(U8), NumberValueNode::new(42u8))
                        .into()
                ),
            }
        );
    }

    #[test]
    fn defined_variable() {
        let meta: Meta = syn::parse_quote! { seed(name = "authority", type = public_key) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let directive = SeedDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            SeedDirective {
                seed: SeedDirectiveType::Defined(
                    VariablePdaSeedNode::new("authority", PublicKeyTypeNode::new()).into()
                ),
            }
        );
    }

    #[test]
    fn linked_seed() {
        let meta: Meta = syn::parse_quote! { seed(name = "authority") };
        let item = syn::parse_quote! { struct Foo { authority: PubKey } };
        let ctx = AttributeContext::Item(&item);
        let directive = SeedDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            SeedDirective {
                seed: SeedDirectiveType::Linked("authority".to_string()),
            }
        );
    }

    #[test]
    fn linked_seed_in_variant() {
        let meta: Meta = syn::parse_quote! { seed(name = "authority") };
        let item: syn::Variant = syn::parse_quote! { Foo { authority: PubKey } };
        let ctx = AttributeContext::Variant(&item);
        let directive = SeedDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(
            directive,
            SeedDirective {
                seed: SeedDirectiveType::Linked("authority".to_string()),
            }
        );
    }

    #[test]
    fn cannot_identify_seed_type() {
        let meta: Meta = syn::parse_quote! { seed(type = public_key) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let error = SeedDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "seed must at least specify `name` for variable seeds or `type` and `value` for constant seeds");
    }

    #[test]
    fn cannot_find_linked_field() {
        let meta: Meta = syn::parse_quote! { seed(name = "authority") };
        let item = syn::parse_quote! { struct Foo { owner: PubKey } };
        let ctx = AttributeContext::Item(&item);
        let error = SeedDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Could not find field \"authority\". Either specify a `type` for the seed or use a name that matches a struct or variant field."
        );
    }

    #[test]
    fn value_with_name() {
        let meta: Meta = syn::parse_quote! { seed(name = "amount", value = 42) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let error = SeedDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "variable seeds cannot specify value");
    }

    #[test]
    fn name_with_value() {
        let meta: Meta = syn::parse_quote! { seed(value = 42, name = "amount") };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = AttributeContext::Item(&item);
        let error = SeedDirective::parse(&meta, &ctx).unwrap_err();
        assert_eq!(error.to_string(), "constant seeds cannot specify name");
    }
}
