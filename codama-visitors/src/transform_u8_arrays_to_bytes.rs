use codama_nodes::{
    ArrayTypeNode, BytesTypeNode, CountNode, Endianness, FixedSizeTypeNode, NumberFormat,
    NumberTypeNode, SizePrefixTypeNode, TypeNode,
};
use codama_visitors_core::{fold_type_node, TransformVisitor};

/// Rewrites every array of little-endian `u8`s into the equivalent bytes type,
/// which downstream codecs and renderers handle far more efficiently than a
/// per-element number array.
///
/// The count node decides the result, mirroring `@codama/visitors`'
/// `transformU8ArraysToBytesVisitor`:
/// - `[u8; N]` (fixed count) -> `fixedSizeTypeNode(bytesTypeNode(), N)`
/// - `[u8]` with a length prefix -> `sizePrefixTypeNode(bytesTypeNode(), prefix)`
/// - `[u8]` to the end of the buffer (remainder) -> `bytesTypeNode()`
///
/// Arrays whose item is not a `u8` (or not little-endian) are left untouched.
/// The transform runs bottom-up, so nested `u8` arrays are converted too.
///
/// ```
/// use codama_nodes::{ArrayTypeNode, BytesTypeNode, FixedSizeTypeNode, NumberTypeNode, TypeNode, U8};
/// use codama_visitors::{TransformU8ArraysToBytes, TransformVisitor};
///
/// let input: TypeNode = ArrayTypeNode::fixed(NumberTypeNode::le(U8), 32).into();
/// let output = TransformU8ArraysToBytes.visit_type_node(input);
/// assert_eq!(
///     output,
///     FixedSizeTypeNode::<TypeNode>::new(BytesTypeNode::new(), 32).into(),
/// );
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct TransformU8ArraysToBytes;

impl TransformVisitor for TransformU8ArraysToBytes {
    fn visit_type_node(&mut self, node: TypeNode) -> TypeNode {
        // Bottom-up: transform children first so nested arrays are converted
        // before we inspect this node.
        let node = fold_type_node(self, node);

        match node {
            TypeNode::Array(array) if is_u8(&array.item) => match *array.count {
                CountNode::Fixed(count) => match usize::try_from(count.value) {
                    Ok(size) => {
                        FixedSizeTypeNode::<TypeNode>::new(BytesTypeNode::new(), size).into()
                    }
                    // A length that doesn't fit in `usize` can't be a fixed-size
                    // byte array on this target; leave the array as-is.
                    Err(_) => ArrayTypeNode {
                        item: array.item,
                        count: Box::new(CountNode::Fixed(count)),
                    }
                    .into(),
                },
                CountNode::Prefixed(count) => {
                    SizePrefixTypeNode::<TypeNode>::new(BytesTypeNode::new(), count.prefix).into()
                }
                CountNode::Remainder(_) => BytesTypeNode::new().into(),
            },
            other => other,
        }
    }
}

/// Returns `true` if the type is a little-endian `u8` number node.
fn is_u8(item: &TypeNode) -> bool {
    matches!(
        item,
        TypeNode::Number(NumberTypeNode {
            format: NumberFormat::U8,
            endian: Endianness::Le,
        })
    )
}
