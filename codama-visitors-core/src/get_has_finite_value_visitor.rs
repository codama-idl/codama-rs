use crate::{LinkableDictionary, MergeVisitor, NodeStack, Visitor, VisitorExt};
use codama_nodes::{CountNode, DefinedTypeNode, NodeRef};
use std::cell::RefCell;

/// Creates a visitor that tells whether the visited type has at least one finite value, following
/// links, like `getHasFiniteValueVisitor` in the JavaScript library.
///
/// Only the shape of values matters: options may be `None`, enums may pick any variant and
/// collections without a fixed positive count may be empty, so all of them stop a recursion.
/// Transforms, prefixes and sizes only prevent a value from existing when their constants link back to
/// a type being visited, since those constants could then never be created. Links that cannot be
/// resolved count as finite.
///
/// A link back to a defined type being visited has no finite value on its own: a finite value never
/// needs to nest a type within itself, so another branch must provide one. Enums without variants have
/// no value at all.
///
/// Links are resolved from the closest program of the current path of `stack`, which the visitor
/// records its nodes on. Visit nodes with their full path, using [`NodeStack::visit_path`] with that
/// same stack, as links visited without a program are not resolved.
///
/// ```
/// use codama_nodes::{
///     DefinedTypeLinkNode, DefinedTypeNode, NodeRef, OptionTypeNode, ProgramNode, RootNode,
///     StructFieldTypeNode, StructTypeNode, TypeNode,
/// };
/// use codama_visitors_core::{
///     get_has_finite_value_visitor, get_linkables_from_node, visit, NodePath, NodeStack,
/// };
///
/// // list = struct { next: option<link(list)> }, loop = struct { next: link(loop) }
/// let next = |r#type: TypeNode| StructTypeNode::new(vec![StructFieldTypeNode::new("next", r#type)]);
/// let list = OptionTypeNode::new(DefinedTypeLinkNode::new("list"));
/// let root = RootNode::new(ProgramNode {
///     defined_types: vec![
///         DefinedTypeNode::new("list", next(list.into())),
///         DefinedTypeNode::new("loop", next(DefinedTypeLinkNode::new("loop").into())),
///     ],
///     ..ProgramNode::new("token", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
/// });
/// let linkables = get_linkables_from_node(&root);
/// let stack = NodeStack::default();
/// let visitor = get_has_finite_value_visitor(&linkables, &stack);
///
/// // Visits the defined type at `index` with its full path.
/// let has_finite_value = |index: usize| {
///     let path = NodePath::from(vec![
///         NodeRef::from(&root),
///         NodeRef::from(&root.program),
///         NodeRef::from(&root.program.defined_types[index]),
///     ]);
///     stack.visit_path(path, |node| visit(&visitor, node))
/// };
/// assert!(has_finite_value(0));
/// assert!(!has_finite_value(1));
/// ```
pub fn get_has_finite_value_visitor<'s, 'a: 's>(
    linkables: &'s LinkableDictionary<'a>,
    stack: &'s NodeStack<'a>,
) -> impl Visitor<'a, bool> + 's {
    // The defined types being visited, from the outermost one.
    let visiting = RefCell::new(Vec::<&'a DefinedTypeNode>::new());
    // Only `true` is cached: a `false` may only hold because of the defined types being visited,
    // whereas a `true` holds whichever are, so it never needs computing again.
    let finite = RefCell::new(Vec::<&'a DefinedTypeNode>::new());
    let contains = |types: &RefCell<Vec<&'a DefinedTypeNode>>, defined_type| {
        types
            .borrow()
            .iter()
            .any(|other| std::ptr::eq(*other, defined_type))
    };

    MergeVisitor::new(|_, values: Vec<bool>| values.into_iter().all(|value| value))
        .extend(move |node, scope| match node {
            NodeRef::ArrayType(array) => {
                !has_fixed_positive_count(&array.count) || scope.visit(&*array.item)
            }
            NodeRef::DefinedType(defined_type) => {
                if contains(&finite, defined_type) {
                    return true;
                }
                visiting.borrow_mut().push(defined_type);
                let has_finite_value = scope.next(node);
                visiting.borrow_mut().pop();
                if has_finite_value {
                    finite.borrow_mut().push(defined_type);
                }
                has_finite_value
            }
            NodeRef::DefinedTypeLink(_) => {
                let Some(linked_path) = linkables.get_path(&stack.get_path()) else {
                    return true;
                };
                let linked = linked_path.last().expect("a linked path is never empty");
                let NodeRef::DefinedType(linked_type) = linked else {
                    unreachable!("a defined type link resolves to a defined type");
                };
                if contains(&visiting, linked_type) {
                    return false;
                }
                stack.visit_path(linked_path, |linked| scope.visit(linked))
            }
            NodeRef::EnumType(r#enum) => r#enum.variants.iter().any(|variant| scope.visit(variant)),
            // The defined type of an enum value only references it.
            NodeRef::EnumValue(_) => true,
            NodeRef::MapType(map) => {
                !has_fixed_positive_count(&map.count)
                    || (scope.visit(&*map.key) && scope.visit(&*map.value))
            }
            NodeRef::OptionType(_)
            | NodeRef::RemainderOptionType(_)
            | NodeRef::ZeroableOptionType(_) => true,
            NodeRef::SetType(set) => {
                !has_fixed_positive_count(&set.count) || scope.visit(&*set.item)
            }
            node => scope.next(node),
        })
        .record_node_stack(stack)
}

fn has_fixed_positive_count(count: &CountNode) -> bool {
    matches!(count, CountNode::Fixed(count) if count.value > 0)
}
