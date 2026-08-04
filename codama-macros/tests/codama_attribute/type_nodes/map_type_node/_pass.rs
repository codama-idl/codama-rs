use codama_macros::codama;

#[codama(type = map(public_key, number(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = map(string, number(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = map(key = string, value = number(u8), count = prefixed_count(number(u32))))]
pub struct ExplicitTest;

#[codama(type = map(string, number(u8), remainder_count))]
pub struct RemainderCountTest;

fn main() {}
