mod nested_type_node;
mod number_type_node;
mod post_offset_type_node;
mod sol_amount_type_node;
mod string_type_node;
mod type_node;

pub use nested_type_node::*;
pub use number_type_node::*;
pub use post_offset_type_node::*;
pub use sol_amount_type_node::*;
pub use string_type_node::*;
pub use type_node::*;

pub fn main() {
    // Passing a direct NumberTypeNode to a SolAmountTypeNode.
    SolAmountTypeNode::new(NumberTypeNode::new(U16, Endian::Little));

    // Passing a nested NumberTypeNode to a SolAmountTypeNode.
    let amount = SolAmountTypeNode::new(PostOffsetTypeNode::absolute(
        NumberTypeNode::new(U16, Endian::Little),
        42,
    ));
    amount.number.get_number_type_node();
}
