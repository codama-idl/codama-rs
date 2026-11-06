use codama_macros::codama;

#[codama(type = sentinel(string, constant(integer(u8), 255)))]
pub struct ImplicitTest;

#[codama(type = sentinel(type = string, sentinel = constant(integer(u8), 255)))]
pub struct ExplicitTest;

fn main() {}
