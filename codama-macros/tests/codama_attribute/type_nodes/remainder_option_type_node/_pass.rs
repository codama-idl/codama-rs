use codama_macros::codama;

#[codama(type = remainder_option(integer(u8)))]
pub struct ImplicitTest;

#[codama(type = remainder_option(item = integer(u8)))]
pub struct ExplicitTest;

#[codama(type = remainder_option(size_prefix(string, integer(u8))))]
pub struct NestedTest;

fn main() {}
