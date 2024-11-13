mod amount_type_node;
mod nested_type_node;
mod number_type_node;
mod post_offset_type_node;
mod pre_offset_type_node;
mod sol_amount_type_node;
mod string_type_node;
mod traits;
mod type_node;

pub use amount_type_node::*;
pub use nested_type_node::*;
pub use number_type_node::*;
pub use post_offset_type_node::*;
pub use pre_offset_type_node::*;
pub use sol_amount_type_node::*;
pub use string_type_node::*;
pub use traits::*;
pub use type_node::*;

pub fn main() {
    // Passing a direct NumberTypeNode to a SolAmountTypeNode.
    AmountTypeNode::new(NumberTypeNode::new(U16, Endian::Little), 2, None);

    // Passing a nested NumberTypeNode to a SolAmountTypeNode.
    let amount = SolAmountTypeNode::new(PostOffsetTypeNode::absolute(
        PreOffsetTypeNode::relative(NumberTypeNode::new(U16, Endian::Little), -10),
        42,
    ));
    amount.number.get_nested_type_node();
}
