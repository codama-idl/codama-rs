use codama_macros::codama;

#[codama(type = option(integer(u8)))]
pub struct ImplicitTest;

#[codama(type = option(item = integer(u8)))]
pub struct ExplicitTest;

#[codama(type = option(integer(u8), fixed))]
pub struct FixedTest;

#[codama(type = option(item = integer(u8), fixed = true))]
pub struct ExplicitFixedTest;

#[codama(type = option(integer(u8), prefix = integer(u16)))]
pub struct PrefixTest;

fn main() {}
