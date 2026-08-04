use codama_macros::codama;

#[codama(type = set(number(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = set(number(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = set(item = number(u8), count = prefixed_count(number(u32))))]
pub struct ExplicitTest;

fn main() {}
