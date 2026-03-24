use crate::DirectiveResolver;
use codama_attributes::ResolvableDirective;
use codama_errors::CodamaResult;
use codama_korok_visitors::KorokVisitable;
use codama_nodes::{InstructionInputValueNode, RegisteredTypeNode};

pub trait KorokPlugin {
    /// Try to resolve a resolvable type directive belonging to this plugin.
    /// Returns `None` if this plugin does not handle the given directive.
    fn resolve_type_directive(
        &self,
        _directive: &ResolvableDirective,
        _resolver: &dyn DirectiveResolver,
    ) -> Option<CodamaResult<RegisteredTypeNode>> {
        None
    }

    /// Try to resolve a resolvable value directive belonging to this plugin.
    /// Returns `None` if this plugin does not handle the given directive.
    fn resolve_value_directive(
        &self,
        _directive: &ResolvableDirective,
        _resolver: &dyn DirectiveResolver,
    ) -> Option<CodamaResult<InstructionInputValueNode>> {
        None
    }

    fn on_initialized(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_fields_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_program_items_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }

    fn on_root_node_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
        Ok(())
    }
}

/// A `DirectiveResolver` built from all installed plugins.
/// Iterates over plugins to find one that can resolve the given directive.
pub struct CompositeDirectiveResolver<'a> {
    plugins: &'a [Box<dyn KorokPlugin + 'a>],
}

impl<'a> CompositeDirectiveResolver<'a> {
    pub fn new(plugins: &'a [Box<dyn KorokPlugin + 'a>]) -> Self {
        Self { plugins }
    }
}

impl<'a> DirectiveResolver for CompositeDirectiveResolver<'a> {
    fn resolve_type_directive(
        &self,
        directive: &ResolvableDirective,
    ) -> CodamaResult<RegisteredTypeNode> {
        for plugin in self.plugins {
            if let Some(result) = plugin.resolve_type_directive(directive, self) {
                return result;
            }
        }
        Err(codama_errors::CodamaError::UnresolvedDirective {
            namespace: directive.namespace.clone(),
            name: directive.name.clone(),
        })
    }

    fn resolve_value_directive(
        &self,
        directive: &ResolvableDirective,
    ) -> CodamaResult<InstructionInputValueNode> {
        for plugin in self.plugins {
            if let Some(result) = plugin.resolve_value_directive(directive, self) {
                return result;
            }
        }
        Err(codama_errors::CodamaError::UnresolvedDirective {
            namespace: directive.namespace.clone(),
            name: directive.name.clone(),
        })
    }
}

pub type ResolvePluginsResult<'a> = Box<dyn Fn(&mut dyn KorokVisitable) -> CodamaResult<()> + 'a>;

/// Combine all plugins into a single function that runs them in sequence.
pub fn resolve_plugins<'a>(plugins: &'a [Box<dyn KorokPlugin + 'a>]) -> ResolvePluginsResult<'a> {
    Box::new(move |visitable: &mut dyn KorokVisitable| {
        // Phase 0: Resolve all resolvable directives.
        let resolver = CompositeDirectiveResolver::new(plugins);
        visitable.accept(&mut crate::ResolveDirectivesVisitor::new(&resolver))?;

        // Phase 1: Initialize.
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_initialized(visitable))?;
        // Phase 2: Set fields.
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_fields_set(visitable))?;
        // Phase 3: Set program items.
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_program_items_set(visitable))?;
        // Phase 4: Set root node.
        plugins
            .iter()
            .try_for_each(|plugin| plugin.on_root_node_set(visitable))?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use codama_korok_visitors::KorokVisitor;
    use codama_nodes::PublicKeyTypeNode;
    use std::{cell::RefCell, rc::Rc};

    // -- Lifecycle ordering tests --

    struct LoggingPlugin {
        id: String,
        logs: Rc<RefCell<Vec<String>>>,
    }
    impl LoggingPlugin {
        fn new(id: &str, logs: Rc<RefCell<Vec<String>>>) -> Self {
            Self {
                id: id.into(),
                logs,
            }
        }
    }
    impl KorokPlugin for LoggingPlugin {
        fn on_initialized(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - initialized", self.id));
            Ok(())
        }
        fn on_fields_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_fields_set", self.id));
            Ok(())
        }
        fn on_program_items_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_program_items_set", self.id));
            Ok(())
        }
        fn on_root_node_set(&self, _visitable: &mut dyn KorokVisitable) -> CodamaResult<()> {
            self.logs
                .borrow_mut()
                .push(format!("Plugin {} - on_root_node_set", self.id));
            Ok(())
        }
    }

    struct MockVisitable;
    impl KorokVisitable for MockVisitable {
        fn accept(&mut self, _visitor: &mut dyn KorokVisitor) -> CodamaResult<()> {
            Ok(())
        }
        fn get_children(&mut self) -> Vec<&mut dyn KorokVisitable> {
            Vec::new()
        }
    }

    #[test]
    fn test_resolve_plugins() -> CodamaResult<()> {
        let logs = Rc::new(RefCell::new(Vec::new()));
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![
            Box::new(LoggingPlugin::new("A", logs.clone())),
            Box::new(LoggingPlugin::new("B", logs.clone())),
        ];

        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut MockVisitable)?;

        assert_eq!(
            logs.borrow().as_slice(),
            &[
                "Plugin A - initialized",
                "Plugin B - initialized",
                "Plugin A - on_fields_set",
                "Plugin B - on_fields_set",
                "Plugin A - on_program_items_set",
                "Plugin B - on_program_items_set",
                "Plugin A - on_root_node_set",
                "Plugin B - on_root_node_set",
            ]
        );
        Ok(())
    }

    // -- CompositeDirectiveResolver tests --

    struct MockTypePlugin;
    impl KorokPlugin for MockTypePlugin {
        fn resolve_type_directive(
            &self,
            directive: &ResolvableDirective,
            _resolver: &dyn DirectiveResolver,
        ) -> Option<CodamaResult<RegisteredTypeNode>> {
            if directive.namespace == "mock" && directive.name == "pubkey" {
                Some(Ok(PublicKeyTypeNode::new().into()))
            } else {
                None
            }
        }
    }

    struct MockValuePlugin;
    impl KorokPlugin for MockValuePlugin {
        fn resolve_value_directive(
            &self,
            directive: &ResolvableDirective,
            _resolver: &dyn DirectiveResolver,
        ) -> Option<CodamaResult<InstructionInputValueNode>> {
            if directive.namespace == "mock" && directive.name == "payer" {
                Some(Ok(codama_nodes::PayerValueNode::new().into()))
            } else {
                None
            }
        }
    }

    fn make_directive(namespace: &str, name: &str) -> ResolvableDirective {
        ResolvableDirective {
            namespace: namespace.into(),
            name: name.into(),
            meta: syn::parse_quote! { foo::bar },
        }
    }

    #[test]
    fn composite_resolver_dispatches_type() {
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(MockTypePlugin)];
        let resolver = CompositeDirectiveResolver::new(&plugins);
        let directive = make_directive("mock", "pubkey");
        let result = resolver.resolve_type_directive(&directive).unwrap();
        assert_eq!(result, PublicKeyTypeNode::new().into());
    }

    #[test]
    fn composite_resolver_dispatches_value() {
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(MockValuePlugin)];
        let resolver = CompositeDirectiveResolver::new(&plugins);
        let directive = make_directive("mock", "payer");
        let result = resolver.resolve_value_directive(&directive).unwrap();
        assert_eq!(result, codama_nodes::PayerValueNode::new().into());
    }

    #[test]
    fn composite_resolver_returns_error_when_unresolved() {
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![];
        let resolver = CompositeDirectiveResolver::new(&plugins);
        let directive = make_directive("unknown", "thing");
        let err = resolver.resolve_type_directive(&directive).unwrap_err();
        assert!(matches!(
            err,
            codama_errors::CodamaError::UnresolvedDirective {
                namespace,
                name,
            } if namespace == "unknown" && name == "thing"
        ));
    }

    #[test]
    fn composite_resolver_skips_non_matching_plugins() {
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![
            Box::new(MockValuePlugin), // doesn't handle types
            Box::new(MockTypePlugin),  // handles mock::pubkey
        ];
        let resolver = CompositeDirectiveResolver::new(&plugins);
        let directive = make_directive("mock", "pubkey");
        let result = resolver.resolve_type_directive(&directive).unwrap();
        assert_eq!(result, PublicKeyTypeNode::new().into());
    }

    // -- Nested resolution (two plugins) --

    /// Plugin A resolves `a::wrapper` by calling the resolver for its inner type.
    struct PluginA;
    impl KorokPlugin for PluginA {
        fn resolve_type_directive(
            &self,
            directive: &ResolvableDirective,
            resolver: &dyn DirectiveResolver,
        ) -> Option<CodamaResult<RegisteredTypeNode>> {
            if directive.namespace != "a" || directive.name != "wrapper" {
                return None;
            }
            // Simulate parsing the inner directive and delegating to resolver.
            let inner = ResolvableDirective {
                namespace: "b".into(),
                name: "inner".into(),
                meta: syn::parse_quote! { b::inner },
            };
            Some(resolver.resolve_type_directive(&inner))
        }
    }

    /// Plugin B resolves `b::inner` → PublicKeyTypeNode.
    struct PluginB;
    impl KorokPlugin for PluginB {
        fn resolve_type_directive(
            &self,
            directive: &ResolvableDirective,
            _resolver: &dyn DirectiveResolver,
        ) -> Option<CodamaResult<RegisteredTypeNode>> {
            if directive.namespace == "b" && directive.name == "inner" {
                Some(Ok(PublicKeyTypeNode::new().into()))
            } else {
                None
            }
        }
    }

    #[test]
    fn nested_resolution_across_two_plugins() {
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(PluginA), Box::new(PluginB)];
        let resolver = CompositeDirectiveResolver::new(&plugins);
        let directive = make_directive("a", "wrapper");
        // PluginA resolves a::wrapper by asking the resolver for b::inner,
        // which PluginB resolves to PublicKeyTypeNode.
        let result = resolver.resolve_type_directive(&directive).unwrap();
        assert_eq!(result, PublicKeyTypeNode::new().into());
    }

    // -- E2e: resolve directives on a korok tree --

    #[test]
    fn e2e_resolves_type_directive_on_korok() -> CodamaResult<()> {
        use codama_attributes::{Resolvable, TryFromFilter, TypeDirective};

        // Build a korok tree from source with a resolvable type directive.
        let item: syn::Item = syn::parse_quote! {
            #[codama(type = mock::pubkey)]
            struct MyAccount;
        };
        let mut korok = codama_koroks::StructKorok::parse(&item)?;

        // Verify the directive is unresolved before resolution.
        let directive_before = korok
            .attributes
            .get_last(TypeDirective::filter)
            .expect("should have a type directive");
        assert!(directive_before.node.is_unresolved());

        // Run resolve_plugins with MockTypePlugin.
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(MockTypePlugin)];
        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut korok)?;

        // Verify the directive is now resolved.
        let directive_after = korok
            .attributes
            .get_last(TypeDirective::filter)
            .expect("should still have a type directive");
        assert!(directive_after.node.is_resolved());
        assert_eq!(
            directive_after.node,
            Resolvable::Resolved(PublicKeyTypeNode::new().into())
        );
        Ok(())
    }

    #[test]
    fn e2e_resolves_nested_directives_across_two_plugins() -> CodamaResult<()> {
        use codama_attributes::{Resolvable, TryFromFilter, TypeDirective};

        // Build a korok tree from source with a resolvable type directive.
        // PluginA handles a::wrapper, which internally delegates to PluginB for b::inner.
        let item: syn::Item = syn::parse_quote! {
            #[codama(type = a::wrapper)]
            struct MyAccount;
        };
        let mut korok = codama_koroks::StructKorok::parse(&item)?;

        // Verify unresolved before.
        let directive_before = korok
            .attributes
            .get_last(TypeDirective::filter)
            .expect("should have a type directive");
        assert!(directive_before.node.is_unresolved());

        // Run resolve_plugins with both plugins.
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(PluginA), Box::new(PluginB)];
        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut korok)?;

        // Verify the directive is resolved to PublicKeyTypeNode
        // (PluginA delegated to PluginB which returns PublicKeyTypeNode).
        let directive_after = korok
            .attributes
            .get_last(TypeDirective::filter)
            .expect("should still have a type directive");
        assert!(directive_after.node.is_resolved());
        assert_eq!(
            directive_after.node,
            Resolvable::Resolved(PublicKeyTypeNode::new().into())
        );
        Ok(())
    }

    #[test]
    fn e2e_resolves_value_directive_on_korok() -> CodamaResult<()> {
        use codama_attributes::{DefaultValueDirective, Resolvable, TryFromFilter};

        // Build a korok tree from source with a resolvable default value directive.
        let field: syn::Field = syn::parse_quote! {
            #[codama(default_value = mock::payer)]
            pub authority: Pubkey
        };
        let mut korok = codama_koroks::FieldKorok::parse(&field)?;

        // Verify the directive is unresolved before resolution.
        let directive_before = korok
            .attributes
            .get_last(DefaultValueDirective::filter)
            .expect("should have a default value directive");
        assert!(directive_before.node.is_unresolved());

        // Run resolve_plugins with MockValuePlugin.
        let plugins: Vec<Box<dyn KorokPlugin>> = vec![Box::new(MockValuePlugin)];
        let run_plugins = resolve_plugins(&plugins);
        run_plugins(&mut korok)?;

        // Verify the directive is now resolved.
        let directive_after = korok
            .attributes
            .get_last(DefaultValueDirective::filter)
            .expect("should still have a default value directive");
        assert!(directive_after.node.is_resolved());
        assert_eq!(
            directive_after.node,
            Resolvable::Resolved(codama_nodes::PayerValueNode::new().into())
        );
        Ok(())
    }

    #[test]
    fn e2e_unresolved_directive_errors() {
        // Build a korok with a resolvable directive but no plugin to resolve it.
        let item: syn::Item = syn::parse_quote! {
            #[codama(type = unknown::thing)]
            struct MyAccount;
        };
        let mut korok = codama_koroks::StructKorok::parse(&item).unwrap();

        let plugins: Vec<Box<dyn KorokPlugin>> = vec![];
        let run_plugins = resolve_plugins(&plugins);
        let err = run_plugins(&mut korok).unwrap_err();
        assert!(matches!(
            err,
            codama_errors::CodamaError::UnresolvedDirective { .. }
        ));
    }
}
