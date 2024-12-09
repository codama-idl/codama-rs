use codama_macros::node;

#[node(number_type(u32, le), public_key_type())]
pub struct Foo(usize);

fn main() {}
