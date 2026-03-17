use crate::utils::FromMeta;
use codama_syn_helpers::Meta;
use std::fmt;

/// A directive that needs to be resolved by an external extension.
/// Resolvable directives are detected by the `prefix::name(...)` syntax where
/// the path contains exactly two segments separated by `::`.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvableDirective {
    /// The namespace prefix, e.g. `"wellknown"` in `wellknown::ata(...)`.
    pub namespace: String,
    /// The directive name, e.g. `"ata"` in `wellknown::ata(...)`.
    pub name: String,
    /// The full Meta content for the extension to parse during resolution.
    pub meta: Meta,
}

impl fmt::Display for ResolvableDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// A value that is either resolved to a concrete node `T` or deferred
/// to an extension for resolution.
#[derive(Debug, Clone, PartialEq)]
pub enum Resolvable<T> {
    /// The value has been resolved to a concrete node.
    Resolved(T),
    /// The value needs to be resolved by an extension.
    Unresolved(Box<ResolvableDirective>),
}

impl<T: FromMeta> Resolvable<T> {
    /// Parse a `Meta` into a `Resolvable<T>`. If the path has exactly two
    /// segments (i.e. `prefix::name`), returns `Unresolved(ResolvableDirective)`.
    /// Otherwise, delegates to `T::from_meta`.
    pub fn from_meta(meta: &Meta) -> syn::Result<Self> {
        if let Ok(path) = meta.path() {
            if path.segments.len() == 2 {
                let namespace = path.segments[0].ident.to_string();
                let name = path.segments[1].ident.to_string();
                return Ok(Resolvable::Unresolved(Box::new(ResolvableDirective {
                    namespace,
                    name,
                    meta: meta.clone(),
                })));
            }
        }
        T::from_meta(meta).map(Resolvable::Resolved)
    }
}

impl<T> Resolvable<T> {
    /// Returns a reference to the resolved value, or `None` if unresolved.
    pub fn resolved(&self) -> Option<&T> {
        match self {
            Resolvable::Resolved(value) => Some(value),
            Resolvable::Unresolved(_) => None,
        }
    }

    /// Returns the resolved value, or an error if unresolved.
    pub fn try_resolved(&self) -> Result<&T, codama_errors::CodamaError> {
        match self {
            Resolvable::Resolved(value) => Ok(value),
            Resolvable::Unresolved(directive) => {
                Err(codama_errors::CodamaError::UnresolvedDirective {
                    namespace: directive.namespace.clone(),
                    name: directive.name.clone(),
                })
            }
        }
    }

    /// Consumes self and returns the resolved value, or an error if unresolved.
    pub fn try_into_resolved(self) -> Result<T, codama_errors::CodamaError> {
        match self {
            Resolvable::Resolved(value) => Ok(value),
            Resolvable::Unresolved(directive) => {
                Err(codama_errors::CodamaError::UnresolvedDirective {
                    namespace: directive.namespace,
                    name: directive.name,
                })
            }
        }
    }

    /// Returns `true` if this is an unresolved directive.
    pub fn is_unresolved(&self) -> bool {
        matches!(self, Resolvable::Unresolved(_))
    }

    /// Returns `true` if this is a resolved value.
    pub fn is_resolved(&self) -> bool {
        matches!(self, Resolvable::Resolved(_))
    }

    /// Maps the resolved value using the given function.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Resolvable<U> {
        match self {
            Resolvable::Resolved(value) => Resolvable::Resolved(f(value)),
            Resolvable::Unresolved(directive) => Resolvable::Unresolved(directive),
        }
    }
}

impl<T> From<T> for Resolvable<T> {
    fn from(value: T) -> Self {
        Resolvable::Resolved(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_nodes::{InstructionInputValueNode, PublicKeyTypeNode, RegisteredTypeNode};

    // -- Resolvable::from_meta --

    #[test]
    fn from_meta_resolves_builtin_type() {
        let meta: Meta = syn::parse_quote! { public_key };
        let result = Resolvable::<RegisteredTypeNode>::from_meta(&meta).unwrap();
        assert!(result.is_resolved());
        assert_eq!(
            result,
            Resolvable::Resolved(PublicKeyTypeNode::new().into())
        );
    }

    #[test]
    fn from_meta_detects_resolvable_type() {
        let meta: Meta = syn::parse_quote! { foo::custom_type };
        let result = Resolvable::<RegisteredTypeNode>::from_meta(&meta).unwrap();
        assert!(result.is_unresolved());
        let Resolvable::Unresolved(ref directive) = result else {
            panic!("expected unresolved");
        };
        assert_eq!(directive.namespace, "foo");
        assert_eq!(directive.name, "custom_type");
    }

    #[test]
    fn from_meta_detects_resolvable_type_with_args() {
        let meta: Meta = syn::parse_quote! { foo::custom_type(42) };
        let result = Resolvable::<RegisteredTypeNode>::from_meta(&meta).unwrap();
        assert!(result.is_unresolved());
        let Resolvable::Unresolved(ref directive) = result else {
            panic!("expected unresolved");
        };
        assert_eq!(directive.namespace, "foo");
        assert_eq!(directive.name, "custom_type");
    }

    #[test]
    fn from_meta_resolves_builtin_value() {
        let meta: Meta = syn::parse_quote! { payer };
        let result = Resolvable::<InstructionInputValueNode>::from_meta(&meta).unwrap();
        assert!(result.is_resolved());
    }

    #[test]
    fn from_meta_detects_resolvable_value() {
        let meta: Meta = syn::parse_quote! { wellknown::ata(account("owner"), account("tokenProgram"), account("mint")) };
        let result = Resolvable::<InstructionInputValueNode>::from_meta(&meta).unwrap();
        assert!(result.is_unresolved());
        let Resolvable::Unresolved(ref directive) = result else {
            panic!("expected unresolved");
        };
        assert_eq!(directive.namespace, "wellknown");
        assert_eq!(directive.name, "ata");
    }

    #[test]
    fn from_meta_errors_on_unrecognized_builtin() {
        let meta: Meta = syn::parse_quote! { banana };
        let result = Resolvable::<RegisteredTypeNode>::from_meta(&meta);
        assert!(result.is_err());
    }

    // -- Resolvable helpers --

    #[test]
    fn resolved_returns_some_for_resolved() {
        let r: Resolvable<u32> = Resolvable::Resolved(42);
        assert_eq!(r.resolved(), Some(&42));
    }

    #[test]
    fn resolved_returns_none_for_unresolved() {
        let r: Resolvable<u32> = Resolvable::Unresolved(Box::new(ResolvableDirective {
            namespace: "foo".into(),
            name: "bar".into(),
            meta: syn::parse_quote! { foo::bar },
        }));
        assert_eq!(r.resolved(), None);
    }

    #[test]
    fn try_resolved_returns_ok_for_resolved() {
        let r: Resolvable<u32> = Resolvable::Resolved(42);
        assert_eq!(r.try_resolved().unwrap(), &42);
    }

    #[test]
    fn try_resolved_returns_err_for_unresolved() {
        let r: Resolvable<u32> = Resolvable::Unresolved(Box::new(ResolvableDirective {
            namespace: "foo".into(),
            name: "bar".into(),
            meta: syn::parse_quote! { foo::bar },
        }));
        let err = r.try_resolved().unwrap_err();
        assert!(matches!(
            err,
            codama_errors::CodamaError::UnresolvedDirective { .. }
        ));
    }

    #[test]
    fn try_into_resolved_returns_value_for_resolved() {
        let r: Resolvable<u32> = Resolvable::Resolved(42);
        assert_eq!(r.try_into_resolved().unwrap(), 42);
    }

    #[test]
    fn try_into_resolved_returns_err_for_unresolved() {
        let r: Resolvable<u32> = Resolvable::Unresolved(Box::new(ResolvableDirective {
            namespace: "foo".into(),
            name: "bar".into(),
            meta: syn::parse_quote! { foo::bar },
        }));
        let err = r.try_into_resolved().unwrap_err();
        assert!(matches!(
            err,
            codama_errors::CodamaError::UnresolvedDirective { .. }
        ));
    }

    #[test]
    fn map_transforms_resolved() {
        let r: Resolvable<u32> = Resolvable::Resolved(42);
        let mapped = r.map(|v| v.to_string());
        assert_eq!(mapped, Resolvable::Resolved("42".to_string()));
    }

    #[test]
    fn map_preserves_unresolved() {
        let r: Resolvable<u32> = Resolvable::Unresolved(Box::new(ResolvableDirective {
            namespace: "foo".into(),
            name: "bar".into(),
            meta: syn::parse_quote! { foo::bar },
        }));
        let mapped: Resolvable<String> = r.map(|v| v.to_string());
        assert!(mapped.is_unresolved());
    }

    // -- Directive-level integration --

    #[test]
    fn type_directive_with_resolvable() {
        let meta: Meta = syn::parse_quote! { type = foo::custom_type };
        let directive = crate::TypeDirective::parse(&meta).unwrap();
        assert!(directive.node.is_unresolved());
    }

    #[test]
    fn default_value_directive_with_resolvable() {
        let meta: Meta = syn::parse_quote! { default_value = bar::my_value(1, 2, 3) };
        let directive = crate::DefaultValueDirective::parse(&meta).unwrap();
        assert!(directive.node.is_unresolved());
        let Resolvable::Unresolved(ref d) = directive.node else {
            panic!("expected unresolved");
        };
        assert_eq!(d.namespace, "bar");
        assert_eq!(d.name, "my_value");
    }

    #[test]
    fn account_directive_with_resolvable_default_value() {
        let meta: Meta = syn::parse_quote! { account(name = "vault", writable, default_value = wellknown::ata(account("owner"))) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = crate::AttributeContext::Item(&item);
        let directive = crate::AccountDirective::parse(&meta, &ctx).unwrap();
        assert_eq!(directive.name, codama_nodes::CamelCaseString::new("vault"));
        assert!(directive.is_writable);
        assert!(directive.default_value.as_ref().unwrap().is_unresolved());
    }

    #[test]
    fn seed_directive_with_resolvable_type() {
        let meta: Meta = syn::parse_quote! { seed(name = "authority", type = foo::custom_pubkey) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = crate::AttributeContext::Item(&item);
        let directive = crate::SeedDirective::parse(&meta, &ctx).unwrap();
        match &directive.seed {
            crate::SeedDirectiveType::Variable { name, r#type } => {
                assert_eq!(name, "authority");
                assert!(r#type.is_unresolved());
            }
            _ => panic!("expected Variable seed"),
        }
    }

    // -- Nested resolvable directives --

    #[test]
    fn field_directive_with_resolvable_type_and_value() {
        let meta: Meta =
            syn::parse_quote! { field("age", foo::custom_type, default_value = bar::custom_value) };
        let directive = crate::FieldDirective::parse(&meta).unwrap();
        assert!(directive.r#type.is_unresolved());
        assert!(directive.default_value.as_ref().unwrap().is_unresolved());
        let Resolvable::Unresolved(ref t) = directive.r#type else {
            panic!("expected unresolved type");
        };
        assert_eq!(t.namespace, "foo");
        assert_eq!(t.name, "custom_type");
        let Resolvable::Unresolved(ref v) = directive.default_value.as_ref().unwrap() else {
            panic!("expected unresolved value");
        };
        assert_eq!(v.namespace, "bar");
        assert_eq!(v.name, "custom_value");
    }

    #[test]
    fn argument_directive_with_resolvable_type() {
        let meta: Meta = syn::parse_quote! { argument("age", foo::number_type) };
        let directive = crate::ArgumentDirective::parse(&meta).unwrap();
        assert!(directive.r#type.is_unresolved());
        let Resolvable::Unresolved(ref t) = directive.r#type else {
            panic!("expected unresolved type");
        };
        assert_eq!(t.namespace, "foo");
        assert_eq!(t.name, "number_type");
    }

    #[test]
    fn seed_directive_with_resolvable_type_and_value() {
        let meta: Meta =
            syn::parse_quote! { seed(type = foo::custom_type, value = bar::custom_value) };
        let item = syn::parse_quote! { struct Foo; };
        let ctx = crate::AttributeContext::Item(&item);
        let directive = crate::SeedDirective::parse(&meta, &ctx).unwrap();
        match &directive.seed {
            crate::SeedDirectiveType::Constant { r#type, value } => {
                assert!(r#type.is_unresolved());
                assert!(value.is_unresolved());
            }
            _ => panic!("expected Constant seed"),
        }
    }
}
