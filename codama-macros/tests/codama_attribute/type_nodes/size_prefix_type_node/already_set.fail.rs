use codama_macros::codama;

#[codama(type = size_prefix(type = string, type = bytes))]
pub struct TypeTest;

#[codama(type = size_prefix(string, integer(u32), integer(u8)))]
pub struct PrefixTest;

fn main() {}
