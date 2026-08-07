use codama_macros::codama;

#[codama(type = sentinel(string, constant(number(u8), 255)))]
pub struct ImplicitTest;

#[codama(type = sentinel(type = string, sentinel = constant(number(u8), 255)))]
pub struct ExplicitTest;

fn main() {}
