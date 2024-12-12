use codama::codama;

#[codama(node(fixed_size_type(boolean_type, 42)))]
pub struct A(usize);

#[codama(node(fixed_size_type(type = boolean_type, size = 42)))]
pub struct B(usize);

fn main() {}
