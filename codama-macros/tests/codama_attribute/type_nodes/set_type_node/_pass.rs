use codama_macros::codama;

#[codama(type = set(integer(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = set(integer(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = set(item = integer(u8), count = prefixed_count(integer(u32))))]
pub struct ExplicitTest;

fn main() {}
