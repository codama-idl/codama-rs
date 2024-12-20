use crate::{
    AccountBumpValueNode, AccountValueNode, ArgumentValueNode, ArrayValueNode, BooleanValueNode,
    BytesValueNode, ConditionalValueNode, ConstantValueNode, EnumValueNode, IdentityValueNode,
    MapValueNode, NoneValueNode, NumberValueNode, PayerValueNode, PdaValueNode, ProgramIdValueNode,
    ProgramLinkNode, PublicKeyValueNode, ResolverValueNode, SetValueNode, SomeValueNode,
    StringValueNode, StructValueNode, TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum InstructionInputValueNode {
    // ContextualValueNodes.
    Account(AccountValueNode),
    AccountBump(AccountBumpValueNode),
    Argument(ArgumentValueNode),
    Conditional(ConditionalValueNode),
    Identity(IdentityValueNode),
    Payer(PayerValueNode),
    Pda(PdaValueNode),
    ProgramId(ProgramIdValueNode),
    Resolver(ResolverValueNode),

    // ValueNodes.
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Enum(EnumValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    Number(NumberValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),

    // LinkNodes.
    ProgramLink(ProgramLinkNode),
}
