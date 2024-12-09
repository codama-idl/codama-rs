use codama_macros::node;

#[node(number_type(u32, be))]
pub struct Foo(usize);

#[codama_macros::node(number_type(u32, be))]
pub struct Bar(usize);

fn main() {}
