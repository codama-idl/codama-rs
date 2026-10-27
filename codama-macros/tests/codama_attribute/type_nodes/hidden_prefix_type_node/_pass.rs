use codama_macros::codama;

#[codama(type = hidden_prefix(string, constant(integer(u8), 1)))]
pub struct SingleConstantTest;

#[codama(type = hidden_prefix(string, constant(integer(u8), 1), constant(integer(u8), 2)))]
pub struct MultipleConstantsTest;

fn main() {}
