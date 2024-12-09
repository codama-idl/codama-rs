use codama::node;

#[node(number_type(u32, be))]
pub struct FooA(usize);

#[codama::node(number_type(u32, be))]
pub struct FooB(usize);

#[codama_macros::node(number_type(u32, be))]
pub struct FooC(usize);

fn main() {}
