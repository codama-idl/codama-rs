use codama_macros::node;

#[node(number_type(u32, unrecognized = 42, le))]
pub struct Foo(usize);

fn main() {}
