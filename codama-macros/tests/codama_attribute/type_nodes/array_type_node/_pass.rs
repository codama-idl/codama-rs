use codama_macros::codama;

#[codama(type = array(number(u8), fixed_count(3)))]
pub struct ImplicitTest;

#[codama(type = array(number(u8), 3))]
pub struct BareIntegerCountTest;

#[codama(type = array(item = number(u8), count = fixed_count(3)))]
pub struct ExplicitTest;

#[codama(type = array(string, prefixed_count(number(u32))))]
pub struct PrefixedCountTest;

#[codama(type = array(number(u8), remainder_count))]
pub struct RemainderCountTest;

fn main() {}
