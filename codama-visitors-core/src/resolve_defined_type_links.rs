use crate::{LinkableDictionary, NodePath};
use codama_errors::{CodamaError, CodamaResult};
use codama_nodes::{DefinedTypeNode, NodeRef};

/// Follows the defined type links at the end of `type_path` until a type that is not a link, and
/// returns the path of that type, or `None` for alias cycles, such as `a = b` and `b = a`, which have no
/// value. A path that does not end with a link is returned as is.
///
/// Fails with `LinkedNodeNotFound` when a link points to no recorded defined type.
///
/// ```
/// use codama_nodes::{
///     DefinedTypeLinkNode, DefinedTypeNode, FloatTypeNode, NodeRef, ProgramNode, RootNode,
///     StructFieldTypeNode, StructTypeNode, F64,
/// };
/// use codama_visitors_core::{get_linkables_from_node, resolve_defined_type_links, NodePath};
///
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![
///         DefinedTypeNode::new("rate", DefinedTypeLinkNode::new("ratio")),
///         DefinedTypeNode::new("ratio", FloatTypeNode::le(F64)),
///     ],
///     ..ProgramNode::new("exchange", "Exchange11111111111111111111111111111111111")
/// });
/// let linkables = get_linkables_from_node(&root);
///
/// // `rate` resolves to the float type of `ratio`.
/// let rate = &root.program.defined_types[0];
/// let path = NodePath::from(vec![NodeRef::from(&root.program), NodeRef::from(rate), NodeRef::from(&*rate.r#type)]);
/// let resolved = resolve_defined_type_links(&path, &linkables).unwrap().unwrap();
/// assert_eq!(resolved.to_string(), "[programNode]exchange.[definedTypeNode]ratio.[floatTypeNode]");
/// ```
pub fn resolve_defined_type_links<'a>(
    type_path: &NodePath<'a>,
    linkables: &LinkableDictionary<'a>,
) -> CodamaResult<Option<NodePath<'a>>> {
    let mut current = type_path.clone();
    let mut followed_types: Vec<&DefinedTypeNode> = vec![];
    while let Some(NodeRef::DefinedTypeLink(_)) = current.last() {
        let mut defined_type_path = linkables.get_path_or_err(&current)?;
        let defined_type: &DefinedTypeNode = defined_type_path
            .last()
            .and_then(|node| node.try_into().ok())
            .ok_or_else(|| CodamaError::LinkedNodeNotFound {
                path: current.to_string(),
            })?;
        if followed_types
            .iter()
            .any(|followed| std::ptr::eq(*followed, defined_type))
        {
            return Ok(None);
        }
        followed_types.push(defined_type);
        defined_type_path.push(&*defined_type.r#type);
        current = defined_type_path;
    }
    Ok(Some(current))
}
