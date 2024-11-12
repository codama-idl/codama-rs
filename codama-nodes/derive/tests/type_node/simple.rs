use codama_nodes_derive::TypeNode;

pub trait TypeNodeTrait {}

#[derive(TypeNode)]
pub struct NumberTypeNode {}

fn main() {
    // TODO: Assert that NumberTypeNode implements TypeNodeTrait.
}
