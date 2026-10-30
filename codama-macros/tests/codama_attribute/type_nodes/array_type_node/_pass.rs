use codama_macros::codama;

#[codama(type = array(integer(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = array(integer(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = array(item = integer(u8), count = fixed_count(3)))]
pub struct ExplicitTest;

#[codama(type = array(string, prefixed_count(integer(u32))))]
pub struct PrefixedCountTest;

#[codama(type = array(integer(u8), remainder_count))]
pub struct RemainderCountTest;

fn main() {}
