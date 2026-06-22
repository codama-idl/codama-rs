use crate::{
    AmountNumberDisplayNode, DateTimeNumberDisplayNode, DurationNumberDisplayNode,
    EnumVariantDisplayNode, InstructionAccountDisplayNode, InstructionDisplayNode,
    StringDisplayNode, StructFieldDisplayNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum DisplayNode {
    AmountNumber(AmountNumberDisplayNode),
    DateTimeNumber(DateTimeNumberDisplayNode),
    DurationNumber(DurationNumberDisplayNode),
    EnumVariant(EnumVariantDisplayNode),
    Instruction(InstructionDisplayNode),
    InstructionAccount(InstructionAccountDisplayNode),
    String(StringDisplayNode),
    StructField(StructFieldDisplayNode),
}
