use codama::codama;

#[codama(node(number_type(u32, be)))]
pub struct A(usize);

#[codama::codama(node(number_type(u32, be)))]
pub struct B(usize);

#[codama_macros::codama(node(number_type(u32, be)))]
pub struct C(usize);

#[codama(node(number_type(u32)))]
pub struct D(usize);

fn main() {}
