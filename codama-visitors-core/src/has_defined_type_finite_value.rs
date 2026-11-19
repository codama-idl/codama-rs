use crate::{
    get_has_finite_value_visitor, is_defined_type_cyclic::get_defined_type_in_program, visit,
    LinkableDictionary, NodePath, NodeStack,
};
use codama_errors::CodamaResult;

/// Whether the defined type at the end of `path` has at least one finite value, following the recorded
/// links (see [`get_has_finite_value_visitor`](crate::get_has_finite_value_visitor)). Like
/// `hasDefinedTypeFiniteValue` in the JavaScript library.
///
/// Types lack one when they are cyclic without a way out, e.g. `a = link(a)` or
/// `node = struct { next: link(node) }`, or when they contain an enum without variants.
///
/// The path must contain the program defining the type, since links are resolved from it, and end with
/// a defined type. Otherwise, it fails with `NodePathProgramMissing` or `UnexpectedNodeKind`.
///
/// ```
/// use codama_nodes::{DefinedTypeLinkNode, DefinedTypeNode, NodeRef, ProgramNode, RootNode};
/// use codama_visitors_core::{get_linkables_from_node, has_defined_type_finite_value, NodePath};
///
/// // alias = link(alias)
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![DefinedTypeNode::new("alias", DefinedTypeLinkNode::new("alias"))],
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
/// let linkables = get_linkables_from_node(&root);
/// let path = NodePath::from(vec![
///     NodeRef::from(&root),
///     NodeRef::from(&root.program),
///     NodeRef::from(&root.program.defined_types[0]),
/// ]);
/// assert!(!has_defined_type_finite_value(&path, &linkables).unwrap());
/// ```
pub fn has_defined_type_finite_value<'a>(
    path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<bool> {
    get_defined_type_in_program(path)?;
    let stack = NodeStack::default();
    let visitor = get_has_finite_value_visitor(linkables, &stack);
    Ok(stack.visit_path(path.clone(), |node| visit(&visitor, node)))
}
