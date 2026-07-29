use codama_macros::codama;

#[codama(type = remainder_option(number(u8)))]
pub struct ImplicitTest;

#[codama(type = remainder_option(item = number(u8)))]
pub struct ExplicitTest;

#[codama(type = remainder_option(size_prefix(string, number(u8))))]
pub struct NestedTest;

fn main() {}
