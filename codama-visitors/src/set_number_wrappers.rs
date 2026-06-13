use codama_nodes::{AmountTypeNode, DateTimeTypeNode, Node, RegisteredTypeNode, SolAmountTypeNode};
use codama_visitors_core::{bottom_up_transformer, BottomUpTransformer, TransformRule};

/// How a matched `numberTypeNode` should be wrapped.
#[derive(Debug, Clone)]
pub enum NumberWrapper {
    /// Wrap as an `amountTypeNode` with the given decimals and optional unit.
    Amount { decimals: u32, unit: Option<String> },
    /// Wrap as a `solAmountTypeNode` (lamports).
    SolAmount,
    /// Wrap as a `dateTimeTypeNode` (unix timestamp).
    DateTime,
}

/// Wraps number types selected by a [`NodeSelector`] path into a semantic type
/// (`amount`, `solAmount` or `dateTime`).
///
/// The Rust counterpart of `@codama/visitors`' `setNumberWrappersVisitor`. Each
/// entry maps a selector *stack* to a wrapper; the visitor appends
/// `.[numberTypeNode]` to that stack, so the selector identifies the context and
/// the visitor wraps the number found there. Built on
/// [`bottom_up_transformer`](codama_visitors_core::bottom_up_transformer).
///
/// ```
/// use codama_nodes::{AccountNode, NumberTypeNode, ProgramNode, RootNode, StructFieldTypeNode, StructTypeNode, U64};
/// use codama_visitors::{set_number_wrappers, NumberWrapper, TransformVisitor};
///
/// let data = StructTypeNode::new(vec![StructFieldTypeNode::new("lamports", NumberTypeNode::le(U64))]);
/// let mut program = ProgramNode::new("p", "Myprogram1111111111111111111111111111111111");
/// program.accounts.push(AccountNode::new("treasury", data));
/// let root = RootNode::new(program);
///
/// let mut visitor = set_number_wrappers([("[accountNode]treasury", NumberWrapper::SolAmount)]);
/// let _root = visitor.visit_root(root);
/// ```
///
/// Note: in the current transformer, number types reachable only as
/// [`NestedTypeNode`](codama_nodes::NestedTypeNode) leaves (e.g. an
/// `enumTypeNode`'s size) and selection by struct-field name are not yet
/// supported — see [`BottomUpTransformer`] for the full scope.
pub fn set_number_wrappers<S: Into<String>>(
    map: impl IntoIterator<Item = (S, NumberWrapper)>,
) -> BottomUpTransformer {
    let rules = map
        .into_iter()
        .map(|(selector_stack, wrapper)| {
            TransformRule::new(
                format!("{}.[numberTypeNode]", selector_stack.into()),
                move |node, _path| wrap_number(node, &wrapper),
            )
        })
        .collect();
    bottom_up_transformer(rules)
}

fn wrap_number(node: Node, wrapper: &NumberWrapper) -> Node {
    // The selector guarantees a number type, but stay total just in case.
    let Node::Type(RegisteredTypeNode::Number(number)) = node else {
        return node;
    };
    let wrapped = match wrapper {
        NumberWrapper::Amount { decimals, unit } => {
            RegisteredTypeNode::Amount(AmountTypeNode::new(number, *decimals, unit.clone()))
        }
        NumberWrapper::SolAmount => RegisteredTypeNode::SolAmount(SolAmountTypeNode::new(number)),
        NumberWrapper::DateTime => RegisteredTypeNode::DateTime(DateTimeTypeNode::new(number)),
    };
    Node::Type(wrapped)
}
