use codama_macros::node;

#[node(numberTypeNode(u32, unrecognized = 42, le))]
pub struct Foo(usize);

fn main() {}
