use crate::{
    AccountValueNode, ArrayValueNode, BooleanValueNode, BytesValueNode, ConstantValueNode,
    DataValueNode, EnumValueNode, FloatValueNode, InjectedValueNode, IntegerValueNode,
    MapValueNode, NoneValueNode, PublicKeyValueNode, SetValueNode, SomeValueNode, StringValueNode,
    StructValueNode, TupleValueNode,
};
use codama_nodes_derive::node_union;

#[node_union]
pub enum PdaSeedValueValue {
    Account(AccountValueNode),
    Array(ArrayValueNode),
    Boolean(BooleanValueNode),
    Bytes(BytesValueNode),
    Constant(ConstantValueNode),
    Data(DataValueNode),
    Enum(EnumValueNode),
    Float(FloatValueNode),
    Injected(InjectedValueNode),
    Integer(IntegerValueNode),
    Map(MapValueNode),
    None(NoneValueNode),
    PublicKey(PublicKeyValueNode),
    Set(SetValueNode),
    Some(SomeValueNode),
    String(StringValueNode),
    Struct(StructValueNode),
    Tuple(TupleValueNode),
}
