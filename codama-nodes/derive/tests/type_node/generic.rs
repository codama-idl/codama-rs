use codama_nodes_derive::TypeNode;

pub trait TypeNodeTrait {}

pub trait SomeTrait {}
impl SomeTrait for u32 {}

#[derive(TypeNode)]
pub struct NumberTypeNode<T: SomeTrait> {
    pub value: T,
}

fn main() {
    // TODO: Assert that NumberTypeNode implements TypeNodeTrait.
}
