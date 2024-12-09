use codama_macros::node;

#[node(numberTypeNode(u32, be))]
pub struct Foo(usize);

#[codama_macros::node(numberTypeNode(u32, be))]
pub struct Bar(usize);

fn main() {}
