use codama_macros::node;

#[node(numberTypeNode(be, le, u32))]
pub struct Foo(usize);

fn main() {}
