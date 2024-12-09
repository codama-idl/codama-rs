use codama_macros::node;

#[node(publicKeyTypeNode)]
pub struct FooA(usize);

#[node(publicKeyTypeNode())]
pub struct FooB(usize);

#[codama_macros::node(publicKeyTypeNode)]
pub struct FooC(usize);

fn main() {}
