use codama::node;

#[node(fixed_size_type(boolean_type, 42))]
pub struct A(usize);

#[node(fixed_size_type(node = boolean_type, size = 42))]
pub struct B(usize);

fn main() {}
