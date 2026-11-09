use crate::Visitor;
use codama_nodes::NodeRef;

/// Wraps a visitor with a function that runs on every node, before or instead of the wrapped visitor.
///
/// Created by [`VisitorExt::extend`](crate::VisitorExt::extend). The function receives the node and
/// an [`ExtendScope`], and pattern-matches the node to choose what to do with it:
///
/// - `scope.next(node)` visits the node with the wrapped visitor, which is what a node the
///   function does not handle should get;
/// - `scope.visit(node)` visits any node with the whole visitor, wrappers included, such as a
///   single child of the node or a node it links to.
///
/// It covers both `extendVisitor` and `interceptVisitor` of the JavaScript library.
///
/// ```
/// use codama_nodes::{IntegerTypeNode, NodeRef, StructFieldTypeNode, U8};
/// use codama_visitors_core::{visit, MergeVisitor, VisitorExt};
///
/// // Lists the identifiers of the struct fields of a tree.
/// let visitor = MergeVisitor::new(|_, values: Vec<Vec<String>>| values.concat()).extend(
///     |node, scope| match node {
///         NodeRef::StructFieldType(field) => {
///             let mut identifiers = vec![field.identifier.to_string()];
///             identifiers.extend(scope.next(node));
///             identifiers
///         }
///         node => scope.next(node),
///     },
/// );
///
/// let field = StructFieldTypeNode::new("amount", IntegerTypeNode::le(U8));
/// assert_eq!(visit(&visitor, &field), ["amount"]);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ExtendVisitor<V, F> {
    visitor: V,
    extension: F,
}

impl<V, F> ExtendVisitor<V, F> {
    /// Wraps `visitor` with `extension`. Prefer [`VisitorExt::extend`](crate::VisitorExt::extend).
    pub fn new<'a, T>(visitor: V, extension: F) -> Self
    where
        V: Visitor<'a, T>,
        F: Fn(NodeRef<'a>, &ExtendScope<'_, 'a, T>) -> T,
    {
        Self { visitor, extension }
    }
}

impl<'a, T, V, F> Visitor<'a, T> for ExtendVisitor<V, F>
where
    V: Visitor<'a, T>,
    F: Fn(NodeRef<'a>, &ExtendScope<'_, 'a, T>) -> T,
{
    fn visit(&self, node: NodeRef<'a>, recurse: &dyn Fn(NodeRef<'a>) -> T) -> T {
        let next = |node: NodeRef<'a>| self.visitor.visit(node, recurse);
        let scope = ExtendScope {
            next: &next,
            visit: recurse,
        };
        (self.extension)(node, &scope)
    }
}

/// What the function of an [`ExtendVisitor`] can visit, like `{ next, self }` in the JavaScript library.
pub struct ExtendScope<'s, 'a, T> {
    next: &'s dyn Fn(NodeRef<'a>) -> T,
    visit: &'s dyn Fn(NodeRef<'a>) -> T,
}

impl<'a, T> ExtendScope<'_, 'a, T> {
    /// Visits `node` with the wrapped visitor, whose children still go through the whole visitor.
    pub fn next(&self, node: impl Into<NodeRef<'a>>) -> T {
        (self.next)(node.into())
    }

    /// Visits `node` with the whole visitor, including this extension and every other wrapper.
    pub fn visit(&self, node: impl Into<NodeRef<'a>>) -> T {
        (self.visit)(node.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{visit, MergeVisitor, VisitorExt};
    use codama_nodes::{
        HasKind, IntegerTypeNode, NodeRef, StringTypeNode, StructFieldTypeNode, StructTypeNode, U8,
    };

    /// Lists the kinds of every node, parents first.
    fn kinds<'a>() -> MergeVisitor<impl Fn(NodeRef<'a>, Vec<Vec<&'static str>>) -> Vec<&'static str>>
    {
        MergeVisitor::new(|node: NodeRef<'a>, values: Vec<Vec<&'static str>>| {
            std::iter::once(node.kind())
                .chain(values.into_iter().flatten())
                .collect()
        })
    }

    fn r#struct() -> StructTypeNode {
        StructTypeNode::new(vec![
            StructFieldTypeNode::new("a", IntegerTypeNode::le(U8)),
            StructFieldTypeNode::new("b", StringTypeNode::utf8()),
        ])
    }

    #[test]
    fn next_visits_with_the_wrapped_visitor_and_keeps_the_extension_on_children() {
        let r#struct = r#struct();
        // The extension skips integers, including the ones reached through `next` on a parent.
        let visitor = kinds().extend(|node, scope| match node {
            NodeRef::IntegerType(_) => vec![],
            node => scope.next(node),
        });
        assert_eq!(
            visit(&visitor, &r#struct),
            [
                "structTypeNode",
                "structFieldTypeNode",
                "structFieldTypeNode",
                "stringTypeNode"
            ]
        );
    }

    #[test]
    fn visit_visits_another_node_with_the_whole_visitor() {
        let r#struct = r#struct();
        // Struct fields only visit their type, through the whole visitor.
        let visitor = kinds().extend(|node, scope| match node {
            NodeRef::StructFieldType(field) => scope.visit(&*field.r#type),
            NodeRef::IntegerType(_) => vec!["integer"],
            node => scope.next(node),
        });
        assert_eq!(
            visit(&visitor, &r#struct),
            ["structTypeNode", "integer", "stringTypeNode"]
        );
    }

    #[test]
    fn extensions_stack_with_the_outermost_first() {
        let integer = IntegerTypeNode::le(U8);
        let visitor = kinds()
            .extend(|node, scope| match node {
                NodeRef::IntegerType(_) => vec!["inner"],
                node => scope.next(node),
            })
            .extend(|node, scope| match node {
                NodeRef::IntegerType(_) => {
                    let mut kinds = vec!["outer"];
                    kinds.extend(scope.next(node));
                    kinds
                }
                node => scope.next(node),
            });
        assert_eq!(visit(&visitor, &integer), ["outer", "inner"]);
    }
}
