use codama::node;

#[node(public_key_type)]
pub struct A(usize);

#[node(public_key_type())]
pub struct B(usize);

#[codama::node(public_key_type)]
pub struct C(usize);

#[codama_macros::node(public_key_type)]
pub struct D(usize);

fn main() {}
