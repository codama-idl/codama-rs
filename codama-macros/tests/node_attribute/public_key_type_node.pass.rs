use codama_macros::node;

#[node(public_key_type)]
pub struct FooA(usize);

#[node(public_key_type())]
pub struct FooB(usize);

#[codama_macros::node(public_key_type)]
pub struct FooC(usize);

fn main() {}
