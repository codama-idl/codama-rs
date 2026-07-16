use crate::{utils::FromMeta, Attribute, CodamaAttribute, CodamaDirective, Resolvable};
use codama_errors::CodamaError;
use codama_nodes::{DefinedTypeLinkNode, Node, RegisteredTypeNode, TypeNode};
use codama_syn_helpers::Meta;

#[derive(Debug, PartialEq)]
pub struct TypeDirective {
    pub node: Resolvable<TypeDirectiveNode>,
}

impl TypeDirective {
    pub fn parse(meta: &Meta) -> syn::Result<Self> {
        let value = meta.assert_directive("type")?.as_value()?;
        Ok(Self {
            node: Resolvable::<TypeDirectiveNode>::from_meta(value)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub enum TypeDirectiveNode {
    Registered(RegisteredTypeNode),
    Link(DefinedTypeLinkNode),
}

impl FromMeta for TypeDirectiveNode {
    fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta.path_str().as_str() {
            "link" => DefinedTypeLinkNode::from_meta(meta).map(Self::from),
            _ => RegisteredTypeNode::from_meta(meta).map(Self::from),
        }
    }
}

impl From<TypeDirectiveNode> for Node {
    fn from(node: TypeDirectiveNode) -> Self {
        match node {
            TypeDirectiveNode::Registered(node) => Self::Type(node),
            TypeDirectiveNode::Link(node) => node.into(),
        }
    }
}

impl TryFrom<TypeDirectiveNode> for TypeNode {
    type Error = CodamaError;

    fn try_from(node: TypeDirectiveNode) -> Result<Self, Self::Error> {
        match node {
            TypeDirectiveNode::Registered(node) => Self::try_from(node),
            TypeDirectiveNode::Link(node) => Ok(Self::Link(node)),
        }
    }
}

impl<'a> TryFrom<&'a CodamaAttribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a CodamaAttribute) -> Result<Self, Self::Error> {
        match attribute.directive.as_ref() {
            CodamaDirective::Type(ref a) => Ok(a),
            _ => Err(CodamaError::InvalidCodamaDirective {
                expected: "type".to_string(),
                actual: attribute.directive.name().to_string(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a Attribute<'a>> for &'a TypeDirective {
    type Error = CodamaError;

    fn try_from(attribute: &'a Attribute) -> Result<Self, Self::Error> {
        <&CodamaAttribute>::try_from(attribute)?.try_into()
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
        let directive = TypeDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.node,
            Resolvable::Resolved(RegisteredTypeNode::from(NumberTypeNode::le(U16)).into())
        );
    }

    #[test]
    fn ok_with_link() {
        let meta: Meta = parse_quote! { type = link("customString") };
        let directive = TypeDirective::parse(&meta).unwrap();
        assert_eq!(
            directive.node,
            Resolvable::Resolved(DefinedTypeLinkNode::new("customString").into())
        );
    }

    #[test]
    fn resolvable_directive() {
        let meta: Meta = parse_quote! { type = foo::custom_type };
        let directive = TypeDirective::parse(&meta).unwrap();

        assert!(directive.node.is_unresolved());
        let Resolvable::Unresolved(ref d) = directive.node else {
            panic!("expected unresolved directive");
        };
        assert_eq!(d.namespace, "foo");
        assert_eq!(d.name, "custom_type");
    }

    #[test]
    fn no_input() {
        let meta: Meta = parse_quote! { type =  };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }

    #[test]
    fn multiple_inputs() {
        let meta: Meta = parse_quote! { type = (number(u16, le), public_key) };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "expected a single value, found a list");
    }

    #[test]
    fn unrecognized_type() {
        let meta: Meta = parse_quote! { type = banana };
        let error = TypeDirective::parse(&meta).unwrap_err();
        assert_eq!(error.to_string(), "unrecognized type");
    }
}
