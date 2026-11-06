use codama_macros::codama;

#[codama(type = hidden_suffix(string, constant(integer(u8), 1)))]
pub struct SingleConstantTest;

#[codama(type = hidden_suffix(string, constant(integer(u8), 1), constant(integer(u8), 2)))]
pub struct MultipleConstantsTest;

#[codama(type = hidden_suffix(string))]
pub struct NoConstantsTest;

fn main() {}
