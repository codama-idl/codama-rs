use crate::{LinkableDictionary, MergeVisitor, NodePath, NodeStack, Visitor, VisitorExt};
use codama_nodes::NodeRef;

/// Creates a visitor that lists the paths of the defined types the visited node links to, without
/// following these links, like `getDefinedTypeLinksVisitor` in the JavaScript library. Links that
/// cannot be resolved are skipped.
///
/// Only links shaping a type are listed: the defined type of an `enumValueNode` merely references it,
/// so it is left out.
///
/// Links are resolved from the closest program of the current path of `stack`, which the visitor
/// records its nodes on. Visit nodes with their full path, using [`NodeStack::visit_path`] with that
/// same stack, as links visited without a program are not resolved.
///
/// ```
/// use codama_nodes::{
///     DefinedTypeLinkNode, DefinedTypeNode, IntegerTypeNode, NodeRef, OptionTypeNode, ProgramNode,
///     RootNode, StructFieldTypeNode, StructTypeNode, U64, U8,
/// };
/// use codama_visitors_core::{
///     get_defined_type_links_visitor, get_linkables_from_node, visit, NodePath, NodeStack,
/// };
///
/// // node = struct { value: link(amount), next: option<link(node)> }
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![
///         DefinedTypeNode::new(
///             "node",
///             StructTypeNode::new(vec![
///                 StructFieldTypeNode::new("value", DefinedTypeLinkNode::new("amount")),
///                 StructFieldTypeNode::new(
///                     "next",
///                     OptionTypeNode::new(DefinedTypeLinkNode::new("node")),
///                 ),
///             ]),
///         ),
///         DefinedTypeNode::new("amount", IntegerTypeNode::le(U64)),
///     ],
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
/// let linkables = get_linkables_from_node(&root);
/// let stack = NodeStack::default();
/// let visitor = get_defined_type_links_visitor(&linkables, &stack);
///
/// let path = NodePath::from(vec![
///     NodeRef::from(&root),
///     NodeRef::from(&root.program),
///     NodeRef::from(&root.program.defined_types[0]),
/// ]);
/// let links = stack.visit_path(path, |node| visit(&visitor, node));
/// assert_eq!(
///     links.iter().map(ToString::to_string).collect::<Vec<_>>(),
///     ["[programNode]token.[definedTypeNode]amount", "[programNode]token.[definedTypeNode]node"]
/// );
/// ```
pub fn get_defined_type_links_visitor<'s, 'a: 's>(
    linkables: &'s LinkableDictionary<'a>,
    stack: &'s NodeStack<'a>,
) -> impl Visitor<'a, Vec<NodePath<'a>>> + 's {
    MergeVisitor::new(|_, links: Vec<Vec<NodePath<'a>>>| links.concat())
        .extend(move |node, scope| match node {
            NodeRef::DefinedTypeLink(_) => {
                let mut links: Vec<_> = linkables.get_path(&stack.get_path()).into_iter().collect();
                links.extend(scope.next(node));
                links
            }
            NodeRef::EnumValue(value) => match value.value.as_ref() {
                Some(value) => scope.visit(value),
                None => vec![],
            },
            node => scope.next(node),
        })
        .record_node_stack(stack)
}
