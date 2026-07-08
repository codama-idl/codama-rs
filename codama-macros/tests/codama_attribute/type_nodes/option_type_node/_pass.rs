use codama_macros::codama;

#[codama(type = option(number(u8)))]
pub struct ImplicitTest;

#[codama(type = option(item = number(u8)))]
pub struct ExplicitTest;

#[codama(type = option(number(u8), fixed))]
pub struct FixedTest;

#[codama(type = option(item = number(u8), fixed = true))]
pub struct ExplicitFixedTest;

#[codama(type = option(number(u8), prefix = number(u16)))]
pub struct PrefixTest;

fn main() {}
