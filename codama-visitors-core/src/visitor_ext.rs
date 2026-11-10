use crate::{
    ExtendScope, ExtendVisitor, LinkableDictionary, MapVisitor, NodeStack, NonNullableVisitor,
    RecordLinkablesOnFirstVisitVisitor, RecordNodeStackVisitor, TapVisitor, Visitor,
};
use codama_errors::CodamaResult;
use codama_nodes::{Node, NodeRef};

/// Adapters that wrap a visitor into a new one, chained like the adapters of an iterator.
///
/// Each wrapper runs before the visitor it wraps, so the last adapter of a chain is the outermost:
/// in `visitor.extend(..).record_node_stack(&stack)`, the stack holds a node before the extension
/// visits it. Every wrapper, except [`VisitorExt::map`] and [`VisitorExt::non_null`], applies to every
/// node of the tree, as the children of a node are visited through the whole chain.
///
/// **[`VisitorExt::map`] and [`VisitorExt::non_null`] only apply to the node that the visit starts
/// from, so they must come last.** The wrapped visitor visits the rest of the tree on its own, so the
/// wrappers added after them only see that first node: `IdentityVisitor.non_null().record_node_stack(&stack)`
/// records the starting node alone, while `IdentityVisitor.record_node_stack(&stack).non_null()`
/// records every node.
pub trait VisitorExt<'a, T>: Visitor<'a, T> + Sized {
    /// Runs `extension` on every node, before or instead of this visitor. See [`ExtendVisitor`].
    fn extend<F>(self, extension: F) -> ExtendVisitor<Self, F>
    where
        F: Fn(NodeRef<'a>, &ExtendScope<'_, 'a, T>) -> T,
    {
        ExtendVisitor::new(self, extension)
    }

    /// Runs `tap` on every node before this visitor visits it. See [`TapVisitor`].
    fn tap<F>(self, tap: F) -> TapVisitor<Self, F>
    where
        F: Fn(NodeRef<'a>),
    {
        TapVisitor::new(self, tap)
    }

    /// Maps the value of the node that the visit starts from with `map`, which also receives that node.
    /// See [`MapVisitor`].
    fn map<U, F>(self, map: F) -> MapVisitor<Self, F, T>
    where
        F: Fn(T, NodeRef<'a>) -> U,
    {
        MapVisitor::new(self, map)
    }

    /// Records the path of the node being visited into `stack`. See [`RecordNodeStackVisitor`].
    fn record_node_stack<'s>(
        self,
        stack: &'s NodeStack<'a>,
    ) -> RecordNodeStackVisitor<'s, 'a, Self> {
        RecordNodeStackVisitor::new(self, stack)
    }

    /// Records the tree into `linkables` before visiting it. See [`RecordLinkablesOnFirstVisitVisitor`].
    fn record_linkables_on_first_visit<'l>(
        self,
        linkables: &'l LinkableDictionary<'a>,
    ) -> RecordLinkablesOnFirstVisitVisitor<'l, 'a, Self> {
        RecordLinkablesOnFirstVisitVisitor::new(self, linkables)
    }

    /// Returns the visited node, with an error if the visitor deletes it. See [`NonNullableVisitor`].
    fn non_null(self) -> NonNullableVisitor<Self>
    where
        Self: Visitor<'a, CodamaResult<Option<Node>>>,
    {
        NonNullableVisitor::new(self)
    }
}

impl<'a, T, V> VisitorExt<'a, T> for V where V: Visitor<'a, T> {}
