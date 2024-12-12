use codama::codama;

#[codama(node(public_key_type))]
pub struct A(usize);

#[codama(node(public_key_type()))]
pub struct B(usize);

#[codama::codama(node(public_key_type))]
pub struct C(usize);

#[codama_macros::codama(node(public_key_type))]
pub struct D(usize);

fn main() {}
