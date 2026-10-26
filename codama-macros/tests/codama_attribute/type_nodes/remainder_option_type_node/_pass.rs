use codama_macros::codama;

#[codama(type = remainder_option(integer(u8)))]
pub struct ImplicitTest;

#[codama(type = remainder_option(item = integer(u8)))]
pub struct ExplicitTest;

fn main() {}
