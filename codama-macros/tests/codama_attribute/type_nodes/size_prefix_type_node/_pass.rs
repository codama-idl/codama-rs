use codama_macros::codama;

#[codama(type = size_prefix(string, number(u32)))]
pub struct ImplicitTest;

#[codama(type = size_prefix(type = string, prefix = number(u32)))]
pub struct ExplicitTest;

#[codama(type = size_prefix(prefix = number(u32), type = string))]
pub struct ExplicitReorderedTest;

#[codama(type = size_prefix(string, prefix = number(u32)))]
pub struct MixedTest;

#[codama(type = size_prefix(string, fixed_size(number(u32), 4)))]
pub struct NestedPrefixTest;

#[codama(type = option(size_prefix(string, number(u8))))]
pub struct NestedInsideOptionTest;

fn main() {}
