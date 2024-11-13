use codama_nodes_derive::TypeNode;

pub trait TypeNodeTrait {}

pub trait SomeTrait {}
impl SomeTrait for u32 {}

#[derive(TypeNode)]
pub struct NumberTypeNode<T: SomeTrait> {
    pub value: T,
}

fn assert_implements_type_node_trait<T: TypeNodeTrait>() {}

fn main() {
    // This line will only compile if NumberTypeNode<u32> implements TypeNodeTrait.
    assert_implements_type_node_trait::<NumberTypeNode<u32>>();
}
