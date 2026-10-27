use codama_nodes::{HasTransforms, IntegerTypeNode};

/// Builds a node around an integer and moves the integer transforms to that node, as `@codama/upgrade` does.
/// They keep their order and apply after the transforms the node already has.
pub fn hoist_integer_transforms<T: HasTransforms>(
    mut number: IntegerTypeNode,
    wrap: impl FnOnce(IntegerTypeNode) -> T,
) -> T {
    let transforms = std::mem::take(&mut number.transforms);
    let mut node = wrap(number);
    node.transforms_mut().extend(transforms);
    node
}
