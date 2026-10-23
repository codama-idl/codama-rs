use crate::{
    AmountNumberDisplayNode, EnumVariantDisplayNode, InstructionAccountDisplayNode,
    InstructionDisplayNode, StringDisplayNode, StructFieldDisplayNode, UnitNumberDisplayNode,
    ValueLabelDisplayNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum DisplayNode {
    AmountNumber(AmountNumberDisplayNode),
    EnumVariant(EnumVariantDisplayNode),
    Instruction(InstructionDisplayNode),
    InstructionAccount(InstructionAccountDisplayNode),
    String(StringDisplayNode),
    StructField(StructFieldDisplayNode),
    UnitNumber(UnitNumberDisplayNode),
    ValueLabel(ValueLabelDisplayNode),
}
