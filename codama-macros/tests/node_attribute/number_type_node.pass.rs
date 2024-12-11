use codama::node;

#[node(number_type(u32, be))]
pub struct A(usize);

#[codama::node(number_type(u32, be))]
pub struct B(usize);

#[codama_macros::node(number_type(u32, be))]
pub struct C(usize);

#[node(number_type(u32))]
pub struct D(usize);

fn main() {}
