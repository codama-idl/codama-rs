use codama::node;

#[node(public_key_type)]
pub struct FooA(usize);

#[node(public_key_type())]
pub struct FooB(usize);

#[codama::node(public_key_type)]
pub struct FooC(usize);

#[codama_macros::node(public_key_type)]
pub struct FooD(usize);

fn main() {}
