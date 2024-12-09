use codama_macros::node;

#[node(number_type(be, le, u32))]
pub struct Foo(usize);

fn main() {}
