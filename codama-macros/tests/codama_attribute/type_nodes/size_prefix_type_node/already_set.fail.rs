use codama_macros::codama;

#[codama(type = size_prefix(type = string, type = bytes))]
pub struct TypeTest;

#[codama(type = size_prefix(string, number(u32), number(u8)))]
pub struct PrefixTest;

fn main() {}
