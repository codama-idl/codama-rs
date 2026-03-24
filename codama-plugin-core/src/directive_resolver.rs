use codama_attributes::ResolvableDirective;
use codama_errors::CodamaResult;
use codama_nodes::{InstructionInputValueNode, RegisteredTypeNode};

/// Trait that allows plugins to resolve directives from other plugins.
/// The framework builds a `DirectiveResolver` from all installed plugins
/// and passes it to `resolve_type_directive` / `resolve_value_directive`.
pub trait DirectiveResolver {
    /// Resolve a resolvable directive into a type node.
    /// Returns an error if no plugin can resolve it.
    fn resolve_type_directive(
        &self,
        directive: &ResolvableDirective,
    ) -> CodamaResult<RegisteredTypeNode>;

    /// Resolve a resolvable directive into an instruction input value node.
    /// Returns an error if no plugin can resolve it.
    fn resolve_value_directive(
        &self,
        directive: &ResolvableDirective,
    ) -> CodamaResult<InstructionInputValueNode>;
}
