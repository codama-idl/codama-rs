use codama_macros::codama;

#[codama(type = zeroable_option(integer(u8)))]
pub struct ImplicitTest;

#[codama(type = zeroable_option(item = integer(u8)))]
pub struct ExplicitTest;

fn main() {}
