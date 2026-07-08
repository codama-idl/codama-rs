use codama_macros::codama;

#[codama(type = zeroable_option(number(u8)))]
pub struct ImplicitTest;

#[codama(type = zeroable_option(item = number(u8)))]
pub struct ExplicitTest;

fn main() {}
