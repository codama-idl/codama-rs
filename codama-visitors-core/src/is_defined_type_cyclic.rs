use crate::{get_defined_type_links_visitor, visit, LinkableDictionary, NodePath, NodeStack};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{DefinedTypeNode, NodeKinds, NodeRef, ProgramNode};

/// Whether the defined type at the end of `path` links back to itself, directly or through other
/// defined types, following the recorded links (see
/// [`get_defined_type_links_visitor`](crate::get_defined_type_links_visitor)). Like
/// `isDefinedTypeCyclic` in the JavaScript library.
///
/// The path must contain the program defining the type, since links are resolved from it, and end with
/// a defined type. Otherwise, it fails with `NodePathProgramMissing` or `UnexpectedNodeKind`.
///
/// ```
/// use codama_nodes::{
///     DefinedTypeLinkNode, DefinedTypeNode, NodeRef, OptionTypeNode, ProgramNode, RootNode,
/// };
/// use codama_visitors_core::{get_linkables_from_node, is_defined_type_cyclic, NodePath};
///
/// // list = option<link(list)>
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![DefinedTypeNode::new(
///         "list",
///         OptionTypeNode::new(DefinedTypeLinkNode::new("list")),
///     )],
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
/// let linkables = get_linkables_from_node(&root);
/// let path = NodePath::from(vec![
///     NodeRef::from(&root),
///     NodeRef::from(&root.program),
///     NodeRef::from(&root.program.defined_types[0]),
/// ]);
/// assert!(is_defined_type_cyclic(&path, &linkables).unwrap());
/// ```
pub fn is_defined_type_cyclic<'a>(
    path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<bool> {
    let target = get_defined_type_in_program(path)?;
    let stack = NodeStack::default();
    let links_visitor = get_defined_type_links_visitor(linkables, &stack);
    let mut followed: Vec<&'a DefinedTypeNode> = vec![];
    let mut pending = vec![path.clone()];
    while let Some(path) = pending.pop() {
        for linked_path in stack.visit_path(path, |node| visit(&links_visitor, node)) {
            let linked_type = defined_type_at_end_of(&linked_path);
            if std::ptr::eq(linked_type, target) {
                return Ok(true);
            }
            if followed
                .iter()
                .any(|followed| std::ptr::eq(*followed, linked_type))
            {
                continue;
            }
            followed.push(linked_type);
            pending.push(linked_path);
        }
    }
    Ok(false)
}

/// The defined type at the end of `path`, as long as `path` contains the program defining it.
pub(crate) fn get_defined_type_in_program<'a>(
    path: &NodePath<'a>,
) -> CodamaResult<&'a DefinedTypeNode> {
    if path.find_last::<ProgramNode>().is_none() {
        return Err(CodamaError::NodePathProgramMissing {
            path: path.to_string(),
        });
    }
    match path.last() {
        Some(NodeRef::DefinedType(defined_type)) => Ok(defined_type),
        _ => Err(CodamaError::UnexpectedNodeKind {
            expected_kinds: DefinedTypeNode::KINDS.join(", "),
            path: path.to_string(),
        }),
    }
}

/// The defined type at the end of a path that a defined type link resolves to, as listed by
/// [`get_defined_type_links_visitor`](crate::get_defined_type_links_visitor).
fn defined_type_at_end_of<'a>(linked_path: &NodePath<'a>) -> &'a DefinedTypeNode {
    match linked_path.last() {
        Some(NodeRef::DefinedType(defined_type)) => defined_type,
        _ => unreachable!("a defined type link resolves to a defined type"),
    }
}
