use codama_macros::codama;

#[codama(type = map(public_key, integer(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = map(string, integer(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = map(key = string, value = integer(u8), count = prefixed_count(integer(u32))))]
pub struct ExplicitTest;

#[codama(type = map(string, integer(u8), remainder_count))]
pub struct RemainderCountTest;

fn main() {}
